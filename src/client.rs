use crate::encryption::xxtea_encrypt;
use crate::protocol::*;
use pyo3::prelude::*;
use rand::Rng;
use std::mem;
use std::net::{ToSocketAddrs, UdpSocket};
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[pyclass]
pub struct KmBoxNetClient {
    pub(crate) socket: UdpSocket,
    pub(crate) dest_addr: std::net::SocketAddr,
    pub(crate) mac: u32,
    pub(crate) indexpts: u32,
    pub(crate) key: [u8; 16],
    pub(crate) soft_mouse: SoftMouse,
    pub(crate) soft_keyboard: SoftKeyboard,
    pub(crate) mask_keyboard_mouse_flag: i32,
}

impl KmBoxNetClient {
    /// Initialize the KmBoxNet client.
    /// Connects to the device and performs the handshake.
    pub fn new(ip: &str, port: u16, mac_str: &str) -> Result<Self, KmError> {
        let addr_str = format!("{}:{}", ip, port);
        let dest_addr = addr_str
            .to_socket_addrs()
            .map_err(|_| KmError::CreatSocket)?
            .next()
            .ok_or(KmError::CreatSocket)?;

        let socket = UdpSocket::bind("0.0.0.0:0").map_err(|_| KmError::CreatSocket)?;
        socket
            .set_read_timeout(Some(Duration::from_millis(3000)))
            .map_err(|_| KmError::CreatSocket)?;
        socket
            .set_write_timeout(Some(Duration::from_millis(1000)))
            .map_err(|_| KmError::CreatSocket)?;

        let mac = Self::str_to_hex(mac_str);

        // Setup encryption key
        let mut key = [0u8; 16];
        key[0] = (mac >> 24) as u8;
        key[1] = (mac >> 16) as u8;
        key[2] = (mac >> 8) as u8;
        key[3] = (mac >> 0) as u8;

        let mut client = Self {
            socket,
            dest_addr,
            mac,
            indexpts: 0,
            key,
            soft_mouse: SoftMouse::default(),
            soft_keyboard: SoftKeyboard::default(),
            mask_keyboard_mouse_flag: 0,
        };

        // Handshake
        client.connect()?;

        Ok(client)
    }

    fn str_to_hex(s: &str) -> u32 {
        let s = s.replace("-", "").replace(":", "");
        u32::from_str_radix(&s, 16).unwrap_or(0)
    }

    fn connect(&mut self) -> Result<(), KmError> {
        let mut tx = ClientTx::default();
        tx.head.mac = self.mac;
        tx.head.rand = rand::thread_rng().gen();
        tx.head.indexpts = 0;
        tx.head.cmd = CMD_CONNECT;

        // Reset local state
        self.soft_mouse = SoftMouse::default();
        self.soft_keyboard = SoftKeyboard::default();

        let head_size = mem::size_of::<CmdHead>();
        let data =
            unsafe { std::slice::from_raw_parts(&tx as *const ClientTx as *const u8, head_size) };

        self.socket
            .send_to(data, self.dest_addr)
            .map_err(|_| KmError::NetTx)?;

        // Wait for response (simple check)
        let mut buf = [0u8; 1024];
        match self.socket.recv_from(&mut buf) {
            Ok((_size, _src)) => Ok(()),
            Err(_) => Err(KmError::NetRxTimeout),
        }
    }

    pub(crate) fn send_command<T>(&mut self, cmd: u32, payload: &T) -> Result<(), KmError> {
        self.send_command_with_rand(cmd, payload, rand::thread_rng().gen())
    }

    pub(crate) fn send_command_with_rand<T>(
        &mut self,
        cmd: u32,
        payload: &T,
        rand_val: u32,
    ) -> Result<(), KmError> {
        self.indexpts = self.indexpts.wrapping_add(1);

        let mut tx = ClientTx::default();
        tx.head.mac = self.mac;
        tx.head.rand = rand_val;
        tx.head.indexpts = self.indexpts;
        tx.head.cmd = cmd;

        // Copy payload into data buffer
        let payload_size = mem::size_of::<T>();
        unsafe {
            ptr::copy_nonoverlapping(
                payload as *const T as *const u8,
                tx.data.as_mut_ptr(),
                payload_size,
            );
        }

        let total_size = mem::size_of::<CmdHead>() + payload_size;
        let data =
            unsafe { std::slice::from_raw_parts(&tx as *const ClientTx as *const u8, total_size) };

        self.socket
            .send_to(data, self.dest_addr)
            .map_err(|_| KmError::NetTx)?;

        // Wait for ack
        let mut buf = [0u8; 1024];
        self.socket
            .recv_from(&mut buf)
            .map_err(|_| KmError::NetRxTimeout)?;

        Ok(())
    }

    pub(crate) fn send_command_encrypted<T>(
        &mut self,
        cmd: u32,
        payload: &T,
    ) -> Result<(), KmError> {
        self.send_command_encrypted_with_rand(cmd, payload, rand::thread_rng().gen())
    }

    pub(crate) fn send_command_encrypted_with_rand<T>(
        &mut self,
        cmd: u32,
        payload: &T,
        rand_val: u32,
    ) -> Result<(), KmError> {
        self.indexpts = self.indexpts.wrapping_add(1);

        let mut tx = ClientTx::default();
        tx.head.mac = self.mac;
        tx.head.rand = rand_val;
        tx.head.indexpts = self.indexpts;
        tx.head.cmd = cmd;

        let payload_size = mem::size_of::<T>();
        unsafe {
            ptr::copy_nonoverlapping(
                payload as *const T as *const u8,
                tx.data.as_mut_ptr(),
                payload_size,
            );
        }

        let total_size = mem::size_of::<CmdHead>() + payload_size;

        // Prepare buffer for encryption
        let mut enc_buffer = [0u8; 128]; // Fixed 128 bytes buffer as per C++
        unsafe {
            ptr::copy_nonoverlapping(
                &tx as *const ClientTx as *const u8,
                enc_buffer.as_mut_ptr(),
                total_size.min(128),
            );
        }

        xxtea_encrypt(&mut enc_buffer, &self.key);

        self.socket
            .send_to(&enc_buffer, self.dest_addr)
            .map_err(|_| KmError::NetTx)?;

        // Wait for ack
        let mut buf = [0u8; 1024];
        self.socket
            .recv_from(&mut buf)
            .map_err(|_| KmError::NetRxTimeout)?;

        Ok(())
    }
}

#[pymethods]
impl KmBoxNetClient {
    #[new]
    fn py_new(ip: &str, port: u16, mac: &str) -> PyResult<Self> {
        Self::new(ip, port, mac).map_err(Into::into)
    }
}

/// Independent monitor class to receive and process physical keyboard/mouse events.
#[pyclass]
pub struct KmBoxNetMonitor {
    running: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl KmBoxNetMonitor {
    /// Start monitoring on the specified UDP port.
    /// `callback` is called whenever a packet is received from the device.
    ///
    /// # Arguments
    ///
    /// * `port` - The UDP port to bind locally to receive monitor packets.
    /// * `callback` - A closure that processes the received `HardMouse` and `HardKeyboard` data.
    pub fn start<F>(port: u16, mut callback: F) -> Result<Self, KmError>
    where
        F: FnMut(HardMouse, HardKeyboard) + Send + 'static,
    {
        let socket =
            UdpSocket::bind(format!("0.0.0.0:{}", port)).map_err(|_| KmError::CreatSocket)?;
        socket
            .set_read_timeout(Some(Duration::from_millis(100)))
            .map_err(|_| KmError::CreatSocket)?;

        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        let handle = thread::spawn(move || {
            let mut buf = [0u8; 1024];
            while running_clone.load(Ordering::Relaxed) {
                match socket.recv_from(&mut buf) {
                    Ok((len, _)) => {
                        if len >= 20 {
                            // Parse Mouse (8 bytes)
                            // struct { u8 report_id, u8 buttons, i16 x, i16 y, i16 wheel }
                            let mut mouse = HardMouse::default();
                            mouse.buttons = buf[1];
                            mouse.x = i16::from_le_bytes([buf[2], buf[3]]);
                            mouse.y = i16::from_le_bytes([buf[4], buf[5]]);
                            mouse.wheel = i16::from_le_bytes([buf[6], buf[7]]);

                            // Parse Keyboard (12 bytes)
                            // struct { u8 report_id, u8 buttons, u8 data[10] }
                            let mut keyboard = HardKeyboard::default();
                            let kb_offset = 8;
                            keyboard.buttons = buf[kb_offset + 1];
                            keyboard
                                .data
                                .extend_from_slice(&buf[kb_offset + 2..kb_offset + 12]);

                            callback(mouse, keyboard);
                        }
                    }
                    Err(_) => {
                        // Timeout allows checking running flag
                        continue;
                    }
                }
            }
        });

        Ok(Self {
            running,
            handle: Some(handle),
        })
    }

    /// Stop the monitor thread.
    pub fn stop(&mut self) {
        if self.running.load(Ordering::Relaxed) {
            self.running.store(false, Ordering::Relaxed);
            if let Some(handle) = self.handle.take() {
                let _ = handle.join();
            }
        }
    }
}

#[pymethods]
impl KmBoxNetMonitor {
    #[new]
    fn py_start(port: u16, callback: PyObject) -> PyResult<Self> {
        let callback = Arc::new(callback);
        Self::start(port, move |mouse: HardMouse, keyboard: HardKeyboard| {
            Python::with_gil(|py| {
                let args = (mouse, keyboard);
                if let Err(e) = callback.bind(py).call1(args) {
                    e.print(py);
                }
            });
        })
        .map_err(Into::into)
    }

    fn shutdown(&mut self) {
        self.stop();
    }
}

impl Drop for KmBoxNetMonitor {
    fn drop(&mut self) {
        self.stop();
    }
}
