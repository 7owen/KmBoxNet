use pyo3::prelude::*;

mod client;
mod client_keyboard;
mod client_monitor;
mod client_mouse;
mod client_systemt;

pub mod encryption;
pub mod keys;
pub mod protocol;

// Re-export the main struct for easier access: kmbox_rust::KmBoxNet
pub use client::KmBoxNetClient;
pub use client::KmBoxNetMonitor;

#[pymodule]
fn kmbox_net(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<KmBoxNetClient>()?;
    m.add_class::<KmBoxNetMonitor>()?;
    m.add_class::<protocol::HardMouse>()?;
    m.add_class::<protocol::HardKeyboard>()?;
    m.add_function(wrap_pyfunction!(encryption::py_xxtea_encrypt, m)?)?;

    // Common Keys
    m.add("KEY_A", keys::KEY_A)?;
    m.add("KEY_B", keys::KEY_B)?;
    m.add("KEY_C", keys::KEY_C)?;
    m.add("KEY_D", keys::KEY_D)?;
    m.add("KEY_E", keys::KEY_E)?;
    m.add("KEY_F", keys::KEY_F)?;
    m.add("KEY_G", keys::KEY_G)?;
    m.add("KEY_H", keys::KEY_H)?;
    m.add("KEY_I", keys::KEY_I)?;
    m.add("KEY_J", keys::KEY_J)?;
    m.add("KEY_K", keys::KEY_K)?;
    m.add("KEY_L", keys::KEY_L)?;
    m.add("KEY_M", keys::KEY_M)?;
    m.add("KEY_N", keys::KEY_N)?;
    m.add("KEY_O", keys::KEY_O)?;
    m.add("KEY_P", keys::KEY_P)?;
    m.add("KEY_Q", keys::KEY_Q)?;
    m.add("KEY_R", keys::KEY_R)?;
    m.add("KEY_S", keys::KEY_S)?;
    m.add("KEY_T", keys::KEY_T)?;
    m.add("KEY_U", keys::KEY_U)?;
    m.add("KEY_V", keys::KEY_V)?;
    m.add("KEY_W", keys::KEY_W)?;
    m.add("KEY_X", keys::KEY_X)?;
    m.add("KEY_Y", keys::KEY_Y)?;
    m.add("KEY_Z", keys::KEY_Z)?;

    m.add("KEY_1", keys::KEY_1_EXCLAMATION_MARK)?;
    m.add("KEY_2", keys::KEY_2_AT)?;
    m.add("KEY_3", keys::KEY_3_NUMBER_SIGN)?;
    m.add("KEY_4", keys::KEY_4_DOLLAR)?;
    m.add("KEY_5", keys::KEY_5_PERCENT)?;
    m.add("KEY_6", keys::KEY_6_CARET)?;
    m.add("KEY_7", keys::KEY_7_AMPERSAND)?;
    m.add("KEY_8", keys::KEY_8_ASTERISK)?;
    m.add("KEY_9", keys::KEY_9_OPARENTHESIS)?;
    m.add("KEY_0", keys::KEY_0_CPARENTHESIS)?;

    m.add("KEY_ENTER", keys::KEY_ENTER)?;
    m.add("KEY_ESCAPE", keys::KEY_ESCAPE)?;
    m.add("KEY_BACKSPACE", keys::KEY_BACKSPACE)?;
    m.add("KEY_TAB", keys::KEY_TAB)?;
    m.add("KEY_SPACEBAR", keys::KEY_SPACEBAR)?;
    m.add("KEY_CAPS_LOCK", keys::KEY_CAPS_LOCK)?;

    m.add("KEY_F1", keys::KEY_F1)?;
    m.add("KEY_F2", keys::KEY_F2)?;
    m.add("KEY_F3", keys::KEY_F3)?;
    m.add("KEY_F4", keys::KEY_F4)?;
    m.add("KEY_F5", keys::KEY_F5)?;
    m.add("KEY_F6", keys::KEY_F6)?;
    m.add("KEY_F7", keys::KEY_F7)?;
    m.add("KEY_F8", keys::KEY_F8)?;
    m.add("KEY_F9", keys::KEY_F9)?;
    m.add("KEY_F10", keys::KEY_F10)?;
    m.add("KEY_F11", keys::KEY_F11)?;
    m.add("KEY_F12", keys::KEY_F12)?;

    m.add("KEY_LEFTCONTROL", keys::KEY_LEFTCONTROL)?;
    m.add("KEY_LEFTSHIFT", keys::KEY_LEFTSHIFT)?;
    m.add("KEY_LEFTALT", keys::KEY_LEFTALT)?;
    m.add("KEY_LEFT_GUI", keys::KEY_LEFT_GUI)?;
    m.add("KEY_RIGHTCONTROL", keys::KEY_RIGHTCONTROL)?;
    m.add("KEY_RIGHTSHIFT", keys::KEY_RIGHTSHIFT)?;
    m.add("KEY_RIGHTALT", keys::KEY_RIGHTALT)?;
    m.add("KEY_RIGHT_GUI", keys::KEY_RIGHT_GUI)?;

    Ok(())
}
