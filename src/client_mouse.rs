use crate::protocol::{
    KmError, CMD_BAZER_MOVE, CMD_MOUSE_AUTOMOVE, CMD_MOUSE_LEFT, CMD_MOUSE_MIDDLE, CMD_MOUSE_MOVE,
    CMD_MOUSE_RIGHT, CMD_MOUSE_WHEEL,
};
use crate::KmBoxNetClient;
use pyo3::prelude::*;

#[pymethods]
impl KmBoxNetClient {
    // --- Mouse Functions ---

    /// 鼠标移动x,y个单位。一次性移动。无轨迹模拟，速度最快.
    /// 自己写轨迹移动时使用此函数。
    /// 返回值：0正常执行，其他值异常。
    /// Move the mouse relative to current position.
    /// x, y: relative movement
    pub fn mouse_move(&mut self, x: i32, y: i32) -> Result<(), KmError> {
        self.soft_mouse.x = x;
        self.soft_mouse.y = y;
        let mouse = self.soft_mouse;
        let res = self.send_command(CMD_MOUSE_MOVE, &mouse);
        self.soft_mouse.x = 0;
        self.soft_mouse.y = 0;
        res
    }

    /// 鼠标左键控制
    /// isdown :0松开 ，1按下
    /// 返回值：0正常执行，其他值异常。
    /// Set mouse left button state.
    pub fn mouse_left(&mut self, is_down: bool) -> Result<(), KmError> {
        self.soft_mouse.button = if is_down {
            self.soft_mouse.button | 0x01
        } else {
            self.soft_mouse.button & !0x01
        };
        let mouse = self.soft_mouse;
        self.send_command(CMD_MOUSE_LEFT, &mouse)
    }

    /// 鼠标右键控制
    /// isdown :0松开 ，1按下
    /// 返回值：0正常执行，其他值异常。
    /// Set mouse right button state.
    pub fn mouse_right(&mut self, is_down: bool) -> Result<(), KmError> {
        self.soft_mouse.button = if is_down {
            self.soft_mouse.button | 0x02
        } else {
            self.soft_mouse.button & !0x02
        };
        let mouse = self.soft_mouse;
        self.send_command(CMD_MOUSE_RIGHT, &mouse)
    }

    /// 鼠标中键控制
    /// isdown :0松开 ，1按下
    /// 返回值：0正常执行，其他值异常。
    /// Set mouse middle button state.
    pub fn mouse_middle(&mut self, is_down: bool) -> Result<(), KmError> {
        self.soft_mouse.button = if is_down {
            self.soft_mouse.button | 0x04
        } else {
            self.soft_mouse.button & !0x04
        };
        let mouse = self.soft_mouse;
        self.send_command(CMD_MOUSE_MIDDLE, &mouse)
    }

    /// 鼠标侧键1控制
    /// isdown :0松开 ，1按下
    /// 返回值：0正常执行，其他值异常。
    /// Set mouse side button 1 (Back) state.
    pub fn mouse_side1(&mut self, is_down: bool) -> Result<(), KmError> {
        self.soft_mouse.button = if is_down {
            self.soft_mouse.button | 0x08
        } else {
            self.soft_mouse.button & !0x08
        };
        // C++ implementation uses CMD_MOUSE_LEFT for side buttons but updates the button mask
        let mouse = self.soft_mouse;
        self.send_command(CMD_MOUSE_LEFT, &mouse)
    }

    /// 鼠标侧键2控制
    /// isdown :0松开 ，1按下
    /// 返回值：0正常执行，其他值异常。
    /// Set mouse side button 2 (Forward) state.
    pub fn mouse_side2(&mut self, is_down: bool) -> Result<(), KmError> {
        self.soft_mouse.button = if is_down {
            self.soft_mouse.button | 0x10
        } else {
            self.soft_mouse.button & !0x10
        };
        let mouse = self.soft_mouse;
        self.send_command(CMD_MOUSE_LEFT, &mouse)
    }

    /// 鼠标滚轮控制
    /// Scroll mouse wheel.
    pub fn mouse_wheel(&mut self, wheel: i32) -> Result<(), KmError> {
        self.soft_mouse.wheel = wheel;
        let mouse = self.soft_mouse;
        let res = self.send_command(CMD_MOUSE_WHEEL, &mouse);
        self.soft_mouse.wheel = 0;
        res
    }

    /// 鼠标全报告控制函数
    /// Control all mouse parameters at once.
    /// Note: C++ source uses cmd_mouse_wheel for the 'all' function.
    pub fn mouse_all(&mut self, button: i32, x: i32, y: i32, wheel: i32) -> Result<(), KmError> {
        self.soft_mouse.button = button;
        self.soft_mouse.x = x;
        self.soft_mouse.y = y;
        self.soft_mouse.wheel = wheel;
        let mouse = self.soft_mouse;
        let res = self.send_command(CMD_MOUSE_WHEEL, &mouse);
        self.soft_mouse.x = 0;
        self.soft_mouse.y = 0;
        self.soft_mouse.wheel = 0;
        res
    }

    /// 鼠标移动x,y个单位。模拟人为移动x,y个单位。不会出现键鼠异常的检测.
    /// 没有写移动曲线的推荐用此函数。此函数不会出现跳跃现象，按照最小步进逼近
    /// 目标点。耗时比kmNet_mouse_move高。
    /// ms是设置移动需要多少毫秒.注意ms给的值不要太小，太小一样会出现键鼠数据异常。
    /// 尽量像人操作。实际用时会比ms小。
    /// Move mouse with automatic duration control (firmware side).
    pub fn mouse_move_auto(&mut self, x: i32, y: i32, ms: u32) -> Result<(), KmError> {
        self.soft_mouse.x = x;
        self.soft_mouse.y = y;
        let mouse = self.soft_mouse;
        let res = self.send_command_with_rand(CMD_MOUSE_AUTOMOVE, &mouse, ms);
        self.soft_mouse.x = 0;
        self.soft_mouse.y = 0;
        res
    }

    /// 二阶贝塞尔曲线控制
    /// x,y 	:目标点坐标
    /// ms		:拟合此过程用时（单位ms）
    /// x1,y1	:控制点p1点坐标
    /// x2,y2	:控制点p2点坐标
    /// Move mouse using Bezier curve (firmware side).
    pub fn mouse_move_beizer(
        &mut self,
        x: i32,
        y: i32,
        ms: u32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> Result<(), KmError> {
        self.soft_mouse.x = x;
        self.soft_mouse.y = y;
        self.soft_mouse.point[0] = x1;
        self.soft_mouse.point[1] = y1;
        self.soft_mouse.point[2] = x2;
        self.soft_mouse.point[3] = y2;

        let mouse = self.soft_mouse;
        let res = self.send_command_with_rand(CMD_BAZER_MOVE, &mouse, ms);

        self.soft_mouse.x = 0;
        self.soft_mouse.y = 0;
        self.soft_mouse.point = [0; 10];
        res
    }

    // --- Encrypted Mouse Functions ---

    /// 带加密功能的控制
    /// 鼠标移动x,y个单位。一次性移动。无轨迹模拟，速度最快.
    /// 自己写轨迹移动时使用此函数。
    /// 返回值：0正常执行，其他值异常。
    /// 此函数是带加密功能的，可以保证同一个移动指令网络数据包内容都不一样。无法通过网络发码抓捕来特征盒子。
    pub fn enc_mouse_move(&mut self, x: i32, y: i32) -> Result<(), KmError> {
        self.soft_mouse.x = x;
        self.soft_mouse.y = y;
        let mouse = self.soft_mouse;
        let res = self.send_command_encrypted(CMD_MOUSE_MOVE, &mouse);
        self.soft_mouse.x = 0;
        self.soft_mouse.y = 0;
        res
    }

    /// 鼠标左键控制 (加密)
    /// isdown :0松开 ，1按下
    /// 返回值：0正常执行，其他值异常。
    pub fn enc_mouse_left(&mut self, is_down: bool) -> Result<(), KmError> {
        self.soft_mouse.button = if is_down {
            self.soft_mouse.button | 0x01
        } else {
            self.soft_mouse.button & !0x01
        };
        let mouse = self.soft_mouse;
        self.send_command_encrypted(CMD_MOUSE_LEFT, &mouse)
    }

    /// 鼠标右键控制 (加密)
    /// isdown :0松开 ，1按下
    /// 返回值：0正常执行，其他值异常。
    pub fn enc_mouse_right(&mut self, is_down: bool) -> Result<(), KmError> {
        self.soft_mouse.button = if is_down {
            self.soft_mouse.button | 0x02
        } else {
            self.soft_mouse.button & !0x02
        };
        let mouse = self.soft_mouse;
        self.send_command_encrypted(CMD_MOUSE_RIGHT, &mouse)
    }

    /// 鼠标中键控制 (加密)
    /// isdown :0松开 ，1按下
    /// 返回值：0正常执行，其他值异常。
    pub fn enc_mouse_middle(&mut self, is_down: bool) -> Result<(), KmError> {
        self.soft_mouse.button = if is_down {
            self.soft_mouse.button | 0x04
        } else {
            self.soft_mouse.button & !0x04
        };
        let mouse = self.soft_mouse;
        self.send_command_encrypted(CMD_MOUSE_MIDDLE, &mouse)
    }

    /// 鼠标侧键1控制 (加密)
    /// isdown :0松开 ，1按下
    /// 返回值：0正常执行，其他值异常。
    pub fn enc_mouse_side1(&mut self, is_down: bool) -> Result<(), KmError> {
        self.soft_mouse.button = if is_down {
            self.soft_mouse.button | 0x08
        } else {
            self.soft_mouse.button & !0x08
        };
        let mouse = self.soft_mouse;
        self.send_command_encrypted(CMD_MOUSE_LEFT, &mouse)
    }

    /// 鼠标侧键2控制 (加密)
    /// isdown :0松开 ，1按下
    /// 返回值：0正常执行，其他值异常。
    pub fn enc_mouse_side2(&mut self, is_down: bool) -> Result<(), KmError> {
        self.soft_mouse.button = if is_down {
            self.soft_mouse.button | 0x10
        } else {
            self.soft_mouse.button & !0x10
        };
        let mouse = self.soft_mouse;
        self.send_command_encrypted(CMD_MOUSE_LEFT, &mouse)
    }

    /// 鼠标滚轮控制 (加密)
    pub fn enc_mouse_wheel(&mut self, wheel: i32) -> Result<(), KmError> {
        self.soft_mouse.wheel = wheel;
        let mouse = self.soft_mouse;
        let res = self.send_command_encrypted(CMD_MOUSE_WHEEL, &mouse);
        self.soft_mouse.wheel = 0;
        res
    }

    /// 鼠标全报告控制函数 (加密)
    pub fn enc_mouse_all(
        &mut self,
        button: i32,
        x: i32,
        y: i32,
        wheel: i32,
    ) -> Result<(), KmError> {
        self.soft_mouse.button = button;
        self.soft_mouse.x = x;
        self.soft_mouse.y = y;
        self.soft_mouse.wheel = wheel;
        // C++ source uses cmd_mouse_wheel for the 'all' function
        let mouse = self.soft_mouse;
        let res = self.send_command_encrypted(CMD_MOUSE_WHEEL, &mouse);
        self.soft_mouse.x = 0;
        self.soft_mouse.y = 0;
        self.soft_mouse.wheel = 0;
        res
    }

    /// 鼠标移动x,y个单位。模拟人为移动x,y个单位。不会出现键鼠异常的检测. (加密)
    /// 没有写移动曲线的推荐用此函数。此函数不会出现跳跃现象，按照最小步进逼近
    /// 目标点。耗时比kmNet_mouse_move高。
    /// ms是设置移动需要多少毫秒.注意ms给的值不要太小，太小一样会出现键鼠数据异常。
    /// 尽量像人操作。实际用时会比ms小。
    pub fn enc_mouse_move_auto(&mut self, x: i32, y: i32, ms: u32) -> Result<(), KmError> {
        self.soft_mouse.x = x;
        self.soft_mouse.y = y;
        let mouse = self.soft_mouse;
        let res = self.send_command_encrypted_with_rand(CMD_MOUSE_AUTOMOVE, &mouse, ms);
        self.soft_mouse.x = 0;
        self.soft_mouse.y = 0;
        res
    }

    /// 二阶贝塞尔曲线控制 (加密)
    /// x,y 	:目标点坐标
    /// ms		:拟合此过程用时（单位ms）
    /// x1,y1	:控制点p1点坐标
    /// x2,y2	:控制点p2点坐标
    pub fn enc_mouse_move_beizer(
        &mut self,
        x: i32,
        y: i32,
        ms: u32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> Result<(), KmError> {
        self.soft_mouse.x = x;
        self.soft_mouse.y = y;
        self.soft_mouse.point[0] = x1;
        self.soft_mouse.point[1] = y1;
        self.soft_mouse.point[2] = x2;
        self.soft_mouse.point[3] = y2;

        let mouse = self.soft_mouse;
        let res = self.send_command_encrypted_with_rand(CMD_BAZER_MOVE, &mouse, ms);

        self.soft_mouse.x = 0;
        self.soft_mouse.y = 0;
        self.soft_mouse.point = [0; 10];
        res
    }
}
