use crate::keys::*;
use crate::protocol::{KmError, CMD_KEYBOARD_ALL};
use crate::KmBoxNetClient;
use pyo3::prelude::*;
use std::thread;
use std::time::Duration;

#[pymethods]
impl KmBoxNetClient {
    // --- Keyboard Functions ---

    /// 键盘按键按下
    /// vk_key: 键值
    /// Press a key (keydown).
    /// Handles modifier keys and normal keys with duplicate checking and queue management.
    pub fn keydown(&mut self, vkey: i32) -> Result<(), KmError> {
        let vk_u8 = vkey as u8;

        // Handle modifier keys (0xE0 - 0xE7)
        if vk_u8 >= KEY_LEFTCONTROL && vk_u8 <= KEY_RIGHT_GUI {
            match vk_u8 {
                KEY_LEFTCONTROL => self.soft_keyboard.ctrl |= BIT0,
                KEY_LEFTSHIFT => self.soft_keyboard.ctrl |= BIT1,
                KEY_LEFTALT => self.soft_keyboard.ctrl |= BIT2,
                KEY_LEFT_GUI => self.soft_keyboard.ctrl |= BIT3,
                KEY_RIGHTCONTROL => self.soft_keyboard.ctrl |= BIT4,
                KEY_RIGHTSHIFT => self.soft_keyboard.ctrl |= BIT5,
                KEY_RIGHTALT => self.soft_keyboard.ctrl |= BIT6,
                KEY_RIGHT_GUI => self.soft_keyboard.ctrl |= BIT7,
                _ => {}
            }
        } else {
            // Check if key already exists in the queue
            for i in 0..10 {
                if self.soft_keyboard.button[i] == vk_u8 {
                    let keyboard = self.soft_keyboard;
                    return self.send_command(CMD_KEYBOARD_ALL, &keyboard);
                }
            }

            // Find empty slot
            let mut found_slot = false;
            for i in 0..10 {
                if self.soft_keyboard.button[i] == 0 {
                    self.soft_keyboard.button[i] = vk_u8;
                    found_slot = true;
                    break;
                }
            }

            // If queue is full, shift everything left and add to the end
            if !found_slot {
                self.soft_keyboard.button.copy_within(1..10, 0);
                self.soft_keyboard.button[9] = vk_u8;
            }
        }

        let keyboard = self.soft_keyboard;
        self.send_command(CMD_KEYBOARD_ALL, &keyboard)
    }

    /// 键盘按键松开
    /// vk_key: 键值
    /// Release a key (keyup).
    /// Removes the key from the report and shifts subsequent keys to maintain a continuous queue.
    pub fn keyup(&mut self, vkey: i32) -> Result<(), KmError> {
        let vk_u8 = vkey as u8;

        // Handle modifier keys
        if vk_u8 >= KEY_LEFTCONTROL && vk_u8 <= KEY_RIGHT_GUI {
            match vk_u8 {
                KEY_LEFTCONTROL => self.soft_keyboard.ctrl &= !BIT0,
                KEY_LEFTSHIFT => self.soft_keyboard.ctrl &= !BIT1,
                KEY_LEFTALT => self.soft_keyboard.ctrl &= !BIT2,
                KEY_LEFT_GUI => self.soft_keyboard.ctrl &= !BIT3,
                KEY_RIGHTCONTROL => self.soft_keyboard.ctrl &= !BIT4,
                KEY_RIGHTSHIFT => self.soft_keyboard.ctrl &= !BIT5,
                KEY_RIGHTALT => self.soft_keyboard.ctrl &= !BIT6,
                KEY_RIGHT_GUI => self.soft_keyboard.ctrl &= !BIT7,
                _ => {}
            }
        } else {
            // Find key and remove it, shifting others left
            for i in 0..10 {
                if self.soft_keyboard.button[i] == vk_u8 {
                    // Shift remaining keys to fill the gap
                    if i < 9 {
                        self.soft_keyboard.button.copy_within((i + 1)..10, i);
                    }
                    self.soft_keyboard.button[9] = 0;
                    break;
                }
            }
        }

        let keyboard = self.soft_keyboard;
        self.send_command(CMD_KEYBOARD_ALL, &keyboard)
    }

    /// 单击指定按键
    /// ms: 持续时间(毫秒)
    /// Press and release a key with a delay.
    pub fn keypress(&mut self, vkey: i32, ms: u64) -> Result<(), KmError> {
        self.keydown(vkey)?;
        thread::sleep(Duration::from_millis(ms / 2));
        self.keyup(vkey)?;
        thread::sleep(Duration::from_millis(ms / 2));
        Ok(())
    }

    // --- Encrypted Keyboard Functions ---

    /// 键盘按键按下 (加密)
    pub fn enc_keydown(&mut self, vkey: i32) -> Result<(), KmError> {
        // Logic is the same as keydown, but uses encrypted command
        // We reuse keydown's logic by temporarily swapping the send method
        // or just re-implementing the state change part.
        self.update_keyboard_state_down(vkey);
        let keyboard = self.soft_keyboard;
        self.send_command_encrypted(CMD_KEYBOARD_ALL, &keyboard)
    }

    /// 键盘按键松开 (加密)
    pub fn enc_keyup(&mut self, vkey: i32) -> Result<(), KmError> {
        self.update_keyboard_state_up(vkey);
        let keyboard = self.soft_keyboard;
        self.send_command_encrypted(CMD_KEYBOARD_ALL, &keyboard)
    }

    /// 单击指定按键 (加密)
    pub fn enc_keypress(&mut self, vkey: i32, ms: u64) -> Result<(), KmError> {
        self.enc_keydown(vkey)?;
        thread::sleep(Duration::from_millis(ms / 2));
        self.enc_keyup(vkey)?;
        thread::sleep(Duration::from_millis(ms / 2));
        Ok(())
    }

    // Helper methods to avoid duplication between normal and encrypted functions
    fn update_keyboard_state_down(&mut self, vkey: i32) {
        let vk_u8 = vkey as u8;
        if vk_u8 >= KEY_LEFTCONTROL && vk_u8 <= KEY_RIGHT_GUI {
            match vk_u8 {
                KEY_LEFTCONTROL => self.soft_keyboard.ctrl |= BIT0,
                KEY_LEFTSHIFT => self.soft_keyboard.ctrl |= BIT1,
                KEY_LEFTALT => self.soft_keyboard.ctrl |= BIT2,
                KEY_LEFT_GUI => self.soft_keyboard.ctrl |= BIT3,
                KEY_RIGHTCONTROL => self.soft_keyboard.ctrl |= BIT4,
                KEY_RIGHTSHIFT => self.soft_keyboard.ctrl |= BIT5,
                KEY_RIGHTALT => self.soft_keyboard.ctrl |= BIT6,
                KEY_RIGHT_GUI => self.soft_keyboard.ctrl |= BIT7,
                _ => {}
            }
        } else {
            for i in 0..10 {
                if self.soft_keyboard.button[i] == vk_u8 {
                    return;
                }
            }
            let mut found = false;
            for i in 0..10 {
                if self.soft_keyboard.button[i] == 0 {
                    self.soft_keyboard.button[i] = vk_u8;
                    found = true;
                    break;
                }
            }
            if !found {
                self.soft_keyboard.button.copy_within(1..10, 0);
                self.soft_keyboard.button[9] = vk_u8;
            }
        }
    }

    fn update_keyboard_state_up(&mut self, vkey: i32) {
        let vk_u8 = vkey as u8;
        if vk_u8 >= KEY_LEFTCONTROL && vk_u8 <= KEY_RIGHT_GUI {
            match vk_u8 {
                KEY_LEFTCONTROL => self.soft_keyboard.ctrl &= !BIT0,
                KEY_LEFTSHIFT => self.soft_keyboard.ctrl &= !BIT1,
                KEY_LEFTALT => self.soft_keyboard.ctrl &= !BIT2,
                KEY_LEFT_GUI => self.soft_keyboard.ctrl &= !BIT3,
                KEY_RIGHTCONTROL => self.soft_keyboard.ctrl &= !BIT4,
                KEY_RIGHTSHIFT => self.soft_keyboard.ctrl &= !BIT5,
                KEY_RIGHTALT => self.soft_keyboard.ctrl &= !BIT6,
                KEY_RIGHT_GUI => self.soft_keyboard.ctrl &= !BIT7,
                _ => {}
            }
        } else {
            for i in 0..10 {
                if self.soft_keyboard.button[i] == vk_u8 {
                    if i < 9 {
                        self.soft_keyboard.button.copy_within((i + 1)..10, i);
                    }
                    self.soft_keyboard.button[9] = 0;
                    break;
                }
            }
        }
    }
}
