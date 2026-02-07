use crate::protocol::*;
use crate::KmBoxNetClient;
use pyo3::prelude::*;
use rand::Rng;
use std::mem;
use std::net::Ipv4Addr;

#[pymethods]
impl KmBoxNetClient {
    // --- System Configuration Functions ---

    /// 重启盒子
    /// Reboot the device.
    pub fn reboot(&mut self) -> Result<(), KmError> {
        self.indexpts = self.indexpts.wrapping_add(1);

        let mut tx = ClientTx::default();
        tx.head.mac = self.mac;
        tx.head.rand = rand::thread_rng().gen();
        tx.head.indexpts = self.indexpts;
        tx.head.cmd = CMD_REBOOT;

        // Reboot command only sends the header
        let length = mem::size_of::<CmdHead>();
        let data =
            unsafe { std::slice::from_raw_parts(&tx as *const ClientTx as *const u8, length) };

        self.socket
            .send_to(data, self.dest_addr)
            .map_err(|_| KmError::NetTx)?;

        // Wait for acknowledgment
        let mut buf = [0u8; 1024];
        self.socket
            .recv_from(&mut buf)
            .map_err(|_| KmError::NetRxTimeout)?;
        Ok(())
    }

    /// 设置配置信息 改IP与端口号
    /// Set IP configuration (IP and port).
    pub fn set_config(&mut self, ip_str: &str, port: u16) -> Result<(), KmError> {
        let ip: Ipv4Addr = ip_str.parse().map_err(|_| KmError::NetCmd)?;
        self.indexpts = self.indexpts.wrapping_add(1);
        let mut tx = ClientTx::default();
        tx.head.mac = self.mac;
        // Convert IP to u32 in Little Endian to match byte order on wire for "inet_addr" result.
        // inet_addr returns network byte order (Big Endian) packed in u32.
        // On LE machine, this results in bytes: octet1, octet2, octet3, octet4.
        tx.head.rand = u32::from_le_bytes(ip.octets());
        tx.head.indexpts = self.indexpts;
        tx.head.cmd = CMD_SETCONFIG;

        tx.data[0] = (port >> 8) as u8;
        tx.data[1] = (port & 0xFF) as u8;

        let length = mem::size_of::<CmdHead>() + 2;
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

    /// 设置盒子device端的VIDPID
    /// Set device VID and PID.
    /// Note: Requires reboot to take effect.
    pub fn set_vid_pid(&mut self, vid: u16, pid: u16) -> Result<(), KmError> {
        self.indexpts = self.indexpts.wrapping_add(1);
        let mut tx = ClientTx::default();
        tx.head.mac = self.mac;
        tx.head.rand = (vid as u32) | ((pid as u32) << 16);
        tx.head.indexpts = self.indexpts;
        tx.head.cmd = CMD_SETVIDPID;

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

    /// 将整个LCD屏幕用指定颜色填充
    /// Fill the LCD screen with a specific color (RGB565).
    pub fn lcd_color(&mut self, rgb565: u16) -> Result<(), KmError> {
        for y in 0..40 {
            self.indexpts = self.indexpts.wrapping_add(1);
            let mut tx = ClientTx::default();
            tx.head.mac = self.mac;
            tx.head.rand = 0 | (y * 4);
            tx.head.indexpts = self.indexpts;
            tx.head.cmd = CMD_SHOWPIC;

            // Fill buffer with 512 pixels
            for i in 0..512 {
                let offset = i * 2;
                tx.data[offset] = (rgb565 & 0xFF) as u8;
                tx.data[offset + 1] = (rgb565 >> 8) as u8;
            }

            let length = mem::size_of::<CmdHead>() + 1024;
            let data =
                unsafe { std::slice::from_raw_parts(&tx as *const ClientTx as *const u8, length) };

            self.socket
                .send_to(data, self.dest_addr)
                .map_err(|_| KmError::NetTx)?;

            let mut buf = [0u8; 1024];
            self.socket
                .recv_from(&mut buf)
                .map_err(|_| KmError::NetRxTimeout)?;
        }
        Ok(())
    }

    /// 在底部显示一张128x80的图片
    /// Display a 128x80 image at the bottom of the LCD.
    /// buff: Image data (128 * 80 * 2 bytes = 20480 bytes).
    pub fn lcd_picture_bottom(&mut self, buff: &[u8]) -> Result<(), KmError> {
        if buff.len() < 128 * 80 * 2 {
            return Err(KmError::NetCmd);
        }

        for y in 0..20 {
            self.indexpts = self.indexpts.wrapping_add(1);
            let mut tx = ClientTx::default();
            tx.head.mac = self.mac;
            tx.head.rand = 80 + (y * 4);
            tx.head.indexpts = self.indexpts;
            tx.head.cmd = CMD_SHOWPIC;

            let start = (y as usize) * 1024;
            let end = start + 1024;
            tx.data[..1024].copy_from_slice(&buff[start..end]);

            let length = mem::size_of::<CmdHead>() + 1024;
            let data =
                unsafe { std::slice::from_raw_parts(&tx as *const ClientTx as *const u8, length) };

            self.socket
                .send_to(data, self.dest_addr)
                .map_err(|_| KmError::NetTx)?;

            let mut buf = [0u8; 1024];
            self.socket
                .recv_from(&mut buf)
                .map_err(|_| KmError::NetRxTimeout)?;
        }
        Ok(())
    }

    /// 整屏显示128x160图片
    /// Display a 128x160 image on the LCD.
    /// buff: Image data (128 * 160 * 2 bytes = 40960 bytes).
    pub fn lcd_picture(&mut self, buff: &[u8]) -> Result<(), KmError> {
        if buff.len() < 128 * 160 * 2 {
            return Err(KmError::NetCmd);
        }

        for y in 0..40 {
            self.indexpts = self.indexpts.wrapping_add(1);
            let mut tx = ClientTx::default();
            tx.head.mac = self.mac;
            tx.head.rand = y * 4;
            tx.head.indexpts = self.indexpts;
            tx.head.cmd = CMD_SHOWPIC;

            let start = (y as usize) * 1024;
            let end = start + 1024;
            tx.data[..1024].copy_from_slice(&buff[start..end]);

            let length = mem::size_of::<CmdHead>() + 1024;
            let data =
                unsafe { std::slice::from_raw_parts(&tx as *const ClientTx as *const u8, length) };

            self.socket
                .send_to(data, self.dest_addr)
                .map_err(|_| KmError::NetTx)?;

            let mut buf = [0u8; 1024];
            self.socket
                .recv_from(&mut buf)
                .map_err(|_| KmError::NetRxTimeout)?;
        }
        Ok(())
    }

    /// 使能盒子的硬件修正功能
    /// Enable hardware curve correction.
    /// type_: 0: Bezier, 1: Missile tracking, 2: Bezier real-time, 3: RM-RT
    /// value: <=0 to disable, >0 to enable (recommended 16-50, max 100). Higher value = smoother but slower.
    pub fn trace_enable(&mut self, type_: i32, value: i32) -> Result<(), KmError> {
        self.indexpts = self.indexpts.wrapping_add(1);
        let mut tx = ClientTx::default();
        tx.head.mac = self.mac;
        tx.head.rand = ((type_ as u32) << 24) | (value as u32);
        tx.head.indexpts = self.indexpts;
        tx.head.cmd = CMD_TRACE_ENABLE;

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
