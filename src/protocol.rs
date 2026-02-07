use pyo3::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum KmError {
    CreatSocket = -9000,
    NetVersion = -8999,
    NetTx = -8998,
    NetRxTimeout = -8997,
    NetCmd = -8996,
    NetPts = -8995,
    Success = 0,
    UsbDevTxTimeout = 1,
}

impl std::fmt::Display for KmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KmError::CreatSocket => write!(f, "Failed to create socket"),
            KmError::NetVersion => write!(f, "Network version mismatch"),
            KmError::NetTx => write!(f, "Network transmission error"),
            KmError::NetRxTimeout => write!(f, "Network receive timeout"),
            KmError::NetCmd => write!(f, "Network command error"),
            KmError::NetPts => write!(f, "Network PTS error"),
            KmError::Success => write!(f, "Success"),
            KmError::UsbDevTxTimeout => write!(f, "USB device transmission timeout"),
        }
    }
}

impl std::error::Error for KmError {}

impl From<KmError> for PyErr {
    fn from(err: KmError) -> Self {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(err.to_string())
    }
}

pub const CMD_CONNECT: u32 = 0xaf3c2828;
pub const CMD_MOUSE_MOVE: u32 = 0xaede7345;
pub const CMD_MOUSE_LEFT: u32 = 0x9823AE8D;
pub const CMD_MOUSE_MIDDLE: u32 = 0x97a3AE8D;
pub const CMD_MOUSE_RIGHT: u32 = 0x238d8212;
pub const CMD_MOUSE_WHEEL: u32 = 0xffeead38;
pub const CMD_MOUSE_AUTOMOVE: u32 = 0xaede7346;
pub const CMD_KEYBOARD_ALL: u32 = 0x123c2c2f;
pub const CMD_REBOOT: u32 = 0xaa8855aa;
pub const CMD_BAZER_MOVE: u32 = 0xa238455a;
pub const CMD_MONITOR: u32 = 0x27388020;
pub const CMD_DEBUG: u32 = 0x27382021;
pub const CMD_MASK_MOUSE: u32 = 0x23234343;
pub const CMD_UNMASK_ALL: u32 = 0x23344343;
pub const CMD_SETCONFIG: u32 = 0x1d3d3323;
pub const CMD_SETVIDPID: u32 = 0xffed3232;
pub const CMD_SHOWPIC: u32 = 0x12334883;
pub const CMD_TRACE_ENABLE: u32 = 0xbbcdddac;

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct CmdHead {
    pub mac: u32,
    pub rand: u32,
    pub indexpts: u32,
    pub cmd: u32,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct SoftMouse {
    pub button: i32,
    pub x: i32,
    pub y: i32,
    pub wheel: i32,
    pub point: [i32; 10],
}

impl Default for SoftMouse {
    fn default() -> Self {
        Self {
            button: 0,
            x: 0,
            y: 0,
            wheel: 0,
            point: [0; 10],
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct SoftKeyboard {
    pub ctrl: u8,
    pub resvel: u8,
    pub button: [u8; 10],
}

impl Default for SoftKeyboard {
    fn default() -> Self {
        Self {
            ctrl: 0,
            resvel: 0,
            button: [0; 10],
        }
    }
}

#[pyclass]
#[derive(Debug, Copy, Clone, Default)]
pub struct HardMouse {
    #[pyo3(get)]
    pub buttons: u8,
    #[pyo3(get)]
    pub x: i16,
    #[pyo3(get)]
    pub y: i16,
    #[pyo3(get)]
    pub wheel: i16,
}

#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct HardKeyboard {
    #[pyo3(get)]
    pub buttons: u8,
    #[pyo3(get)]
    pub data: Vec<u8>,
}

// Structure to match the union layout for serialization/encryption
#[repr(C)]
pub struct ClientTx {
    pub head: CmdHead,
    // Buffer large enough to hold the largest union member + padding if necessary
    // C++ uses a union with char buff[1024].
    // We use a fixed size array here.
    pub data: [u8; 1024],
}

impl Default for ClientTx {
    fn default() -> Self {
        Self {
            head: CmdHead::default(),
            data: [0; 1024],
        }
    }
}
