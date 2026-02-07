use crate::protocol::*;
use crate::KmBoxNetClient;
use pyo3::prelude::*;
use std::mem;

#[pymethods]
impl KmBoxNetClient {
    // --- Monitor Functions ---

    /// Enable/Disable monitoring of physical keyboard and mouse on the device.
    /// This function sends the command to the device to start/stop streaming data to the specified port.
    /// To receive the data, use `KmBoxMonitor::start(port, callback)`.
    ///
    /// port: UDP port to listen on. If 0, monitoring is disabled on the device.
    pub fn monitor(&mut self, port: u16) -> Result<(), KmError> {
        self.indexpts = self.indexpts.wrapping_add(1);

        let mut tx = ClientTx::default();
        tx.head.mac = self.mac;
        tx.head.indexpts = self.indexpts;
        tx.head.cmd = CMD_MONITOR;

        tx.head.rand = if port > 0 {
            (port as u32) | 0xaa55_0000 // 0xaa55 << 16
        } else {
            0
        };

        let length = mem::size_of::<CmdHead>();
        let data =
            unsafe { std::slice::from_raw_parts(&tx as *const ClientTx as *const u8, length) };

        self.socket
            .send_to(data, self.dest_addr)
            .map_err(|_| KmError::NetTx)?;

        let mut buf = [0u8; 1024];
        self.socket
            .recv_from(&mut buf)
            .map_err(|_| KmError::NetRxTimeout)?;

        Ok(())
    }

    // --- Masking Functions ---

    fn send_mask_command(&mut self, rand_val: u32) -> Result<(), KmError> {
        self.indexpts = self.indexpts.wrapping_add(1);
        let mut tx = ClientTx::default();
        tx.head.mac = self.mac;
        tx.head.indexpts = self.indexpts;
        tx.head.cmd = CMD_MASK_MOUSE;
        tx.head.rand = rand_val;

        let length = mem::size_of::<CmdHead>();
        let data =
            unsafe { std::slice::from_raw_parts(&tx as *const ClientTx as *const u8, length) };

        self.socket
            .send_to(data, self.dest_addr)
            .map_err(|_| KmError::NetTx)?;

        let mut buf = [0u8; 1024];
        self.socket
            .recv_from(&mut buf)
            .map_err(|_| KmError::NetRxTimeout)?;
        Ok(())
    }

    pub fn mask_mouse_left(&mut self, enable: bool) -> Result<(), KmError> {
        if enable {
            self.mask_keyboard_mouse_flag |= 0x01;
        } else {
            self.mask_keyboard_mouse_flag &= !0x01;
        }
        self.send_mask_command(self.mask_keyboard_mouse_flag as u32)
    }

    pub fn mask_mouse_right(&mut self, enable: bool) -> Result<(), KmError> {
        if enable {
            self.mask_keyboard_mouse_flag |= 0x02;
        } else {
            self.mask_keyboard_mouse_flag &= !0x02;
        }
        self.send_mask_command(self.mask_keyboard_mouse_flag as u32)
    }

    pub fn mask_mouse_middle(&mut self, enable: bool) -> Result<(), KmError> {
        if enable {
            self.mask_keyboard_mouse_flag |= 0x04;
        } else {
            self.mask_keyboard_mouse_flag &= !0x04;
        }
        self.send_mask_command(self.mask_keyboard_mouse_flag as u32)
    }

    pub fn mask_mouse_side1(&mut self, enable: bool) -> Result<(), KmError> {
        if enable {
            self.mask_keyboard_mouse_flag |= 0x08;
        } else {
            self.mask_keyboard_mouse_flag &= !0x08;
        }
        self.send_mask_command(self.mask_keyboard_mouse_flag as u32)
    }

    pub fn mask_mouse_side2(&mut self, enable: bool) -> Result<(), KmError> {
        if enable {
            self.mask_keyboard_mouse_flag |= 0x10;
        } else {
            self.mask_keyboard_mouse_flag &= !0x10;
        }
        self.send_mask_command(self.mask_keyboard_mouse_flag as u32)
    }

    pub fn mask_mouse_x(&mut self, enable: bool) -> Result<(), KmError> {
        if enable {
            self.mask_keyboard_mouse_flag |= 0x20;
        } else {
            self.mask_keyboard_mouse_flag &= !0x20;
        }
        self.send_mask_command(self.mask_keyboard_mouse_flag as u32)
    }

    pub fn mask_mouse_y(&mut self, enable: bool) -> Result<(), KmError> {
        if enable {
            self.mask_keyboard_mouse_flag |= 0x40;
        } else {
            self.mask_keyboard_mouse_flag &= !0x40;
        }
        self.send_mask_command(self.mask_keyboard_mouse_flag as u32)
    }

    pub fn mask_mouse_wheel(&mut self, enable: bool) -> Result<(), KmError> {
        if enable {
            self.mask_keyboard_mouse_flag |= 0x80;
        } else {
            self.mask_keyboard_mouse_flag &= !0x80;
        }
        self.send_mask_command(self.mask_keyboard_mouse_flag as u32)
    }

    pub fn mask_keyboard(&mut self, vkey: i32) -> Result<(), KmError> {
        let val = (self.mask_keyboard_mouse_flag & 0xff) | ((vkey & 0xff) << 8);
        self.send_mask_command(val as u32)
    }

    pub fn unmask_keyboard(&mut self, vkey: i32) -> Result<(), KmError> {
        self.indexpts = self.indexpts.wrapping_add(1);
        let mut tx = ClientTx::default();
        tx.head.mac = self.mac;
        tx.head.indexpts = self.indexpts;
        tx.head.cmd = CMD_UNMASK_ALL;
        tx.head.rand = ((self.mask_keyboard_mouse_flag & 0xff) | ((vkey & 0xff) << 8)) as u32;

        let length = mem::size_of::<CmdHead>();
        let data =
            unsafe { std::slice::from_raw_parts(&tx as *const ClientTx as *const u8, length) };

        self.socket
            .send_to(data, self.dest_addr)
            .map_err(|_| KmError::NetTx)?;

        let mut buf = [0u8; 1024];
        self.socket
            .recv_from(&mut buf)
            .map_err(|_| KmError::NetRxTimeout)?;
        Ok(())
    }

    pub fn unmask_all(&mut self) -> Result<(), KmError> {
        self.indexpts = self.indexpts.wrapping_add(1);
        let mut tx = ClientTx::default();
        tx.head.mac = self.mac;
        tx.head.indexpts = self.indexpts;
        tx.head.cmd = CMD_UNMASK_ALL;
        self.mask_keyboard_mouse_flag = 0;
        tx.head.rand = 0;

        let length = mem::size_of::<CmdHead>();
        let data =
            unsafe { std::slice::from_raw_parts(&tx as *const ClientTx as *const u8, length) };

        self.socket
            .send_to(data, self.dest_addr)
            .map_err(|_| KmError::NetTx)?;

        let mut buf = [0u8; 1024];
        self.socket
            .recv_from(&mut buf)
            .map_err(|_| KmError::NetRxTimeout)?;
        Ok(())
    }
}
