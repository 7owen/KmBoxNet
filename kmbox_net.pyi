from typing import Callable, List

def xxtea_encrypt(data: bytearray, key: bytes) -> None:
    """
    使用 XXTEA 算法加密数据 (原地修改)。

    Args:
        data: 要加密的数据缓冲区 (必须是 bytearray，且长度至少为 128 字节)。
        key: 16 字节的加密密钥。
    """
    ...

class HardMouse:
    """
    物理鼠标状态类 (Monitor模式使用)
    """

    buttons: int
    """鼠标按键状态掩码 (位图)"""
    x: int
    """X轴相对移动量"""
    y: int
    """Y轴相对移动量"""
    wheel: int
    """滚轮相对移动量"""

class HardKeyboard:
    """
    物理键盘状态类 (Monitor模式使用)
    """

    buttons: int
    """功能键 (Ctrl, Shift, Alt, GUI) 状态掩码"""
    data: List[int]
    """当前按下的普通键码列表 (通常最多10个键)"""

class KmBoxNetMonitor:
    """
    KmBoxNet 监听器，用于接收并处理来自硬件的物理键鼠事件
    """
    def __init__(
        self, port: int, callback: Callable[[HardMouse, HardKeyboard], None]
    ) -> None:
        """
        初始化监听器并开始后台监听线程。

        Args:
            port: 本地监听端口 (需与盒子端配置的推流端口一致)。
            callback: 回调函数，当收到数据时调用。签名: (mouse: HardMouse, keyboard: HardKeyboard) -> None
        """
        ...
    def shutdown(self) -> None:
        """停止监听线程"""
        ...

class KmBoxNetClient:
    """
    KmBoxNet 客户端，用于连接设备并发送控制指令。
    """
    def __init__(self, ip: str, port: int, mac: str) -> None:
        """
        创建客户端并连接设备。

        Args:
            ip: KmBox 设备的 IP 地址。
            port: KmBox 设备的端口号 (默认通常是 8888)。
            mac: 设备的 MAC 地址 (格式如 "00-11-22-33-44-55")，用于通信加密握手。
        """
        ...

    # Mouse Functions
    def mouse_move(self, x: int, y: int) -> None:
        """
        鼠标相对移动。

        Args:
            x: X轴相对位移 (正数向右，负数向左)。
            y: Y轴相对位移 (正数向下，负数向上)。
        """
        ...
    def mouse_left(self, is_down: bool) -> None:
        """
        鼠标左键控制。

        Args:
            is_down: True 表示按下，False 表示弹起。
        """
        ...
    def mouse_right(self, is_down: bool) -> None:
        """
        鼠标右键控制。

        Args:
            is_down: True 表示按下，False 表示弹起。
        """
        ...
    def mouse_middle(self, is_down: bool) -> None:
        """
        鼠标中键控制。

        Args:
            is_down: True 表示按下，False 表示弹起。
        """
        ...
    def mouse_side1(self, is_down: bool) -> None:
        """
        鼠标侧键1 (Forward) 控制。

        Args:
            is_down: True 表示按下，False 表示弹起。
        """
        ...
    def mouse_side2(self, is_down: bool) -> None:
        """
        鼠标侧键2 (Back) 控制。

        Args:
            is_down: True 表示按下，False 表示弹起。
        """
        ...
    def mouse_wheel(self, wheel: int) -> None:
        """
        鼠标滚轮控制。

        Args:
            wheel: 滚动量 (正数向前/上滚，负数向后/下滚)。
        """
        ...
    def mouse_all(self, button: int, x: int, y: int, wheel: int) -> None:
        """
        发送鼠标完整状态数据包。

        Args:
            button: 按键状态掩码。
            x: X轴相对位移。
            y: Y轴相对位移。
            wheel: 滚轮相对位移。
        """
        ...
    def mouse_move_auto(self, x: int, y: int, ms: int) -> None:
        """
        在指定时间内自动模拟平滑移动 (硬件插值)。

        Args:
            x: 目标相对 X 位移。
            y: 目标相对 Y 位移。
            ms: 完成移动所需的毫秒数。
        """
        ...
    def mouse_move_beizer(
        self, x: int, y: int, ms: int, x1: int, y1: int, x2: int, y2: int
    ) -> None:
        """
        贝塞尔曲线拟合移动 (二阶贝塞尔)。

        Args:
            x: 终点 X 坐标 (相对当前)。
            y: 终点 Y 坐标 (相对当前)。
            ms: 耗时 (毫秒)。
            x1, y1: 控制点1坐标 (相对)。
            x2, y2: 控制点2坐标 (相对)。
        """
        ...

    # Encrypted Mouse Functions
    def enc_mouse_move(self, x: int, y: int) -> None:
        """[加密] 鼠标相对移动。"""
        ...
    def enc_mouse_left(self, is_down: bool) -> None:
        """[加密] 鼠标左键控制。"""
        ...
    def enc_mouse_right(self, is_down: bool) -> None:
        """[加密] 鼠标右键控制。"""
        ...
    def enc_mouse_middle(self, is_down: bool) -> None:
        """[加密] 鼠标中键控制。"""
        ...
    def enc_mouse_side1(self, is_down: bool) -> None:
        """[加密] 鼠标侧键1控制。"""
        ...
    def enc_mouse_side2(self, is_down: bool) -> None:
        """[加密] 鼠标侧键2控制。"""
        ...
    def enc_mouse_wheel(self, wheel: int) -> None:
        """[加密] 鼠标滚轮控制。"""
        ...
    def enc_mouse_all(self, button: int, x: int, y: int, wheel: int) -> None:
        """[加密] 发送鼠标完整状态。"""
        ...
    def enc_mouse_move_auto(self, x: int, y: int, ms: int) -> None:
        """[加密] 自动平滑移动。"""
        ...
    def enc_mouse_move_beizer(
        self, x: int, y: int, ms: int, x1: int, y1: int, x2: int, y2: int
    ) -> None:
        """[加密] 贝塞尔曲线移动。"""
        ...

    # Keyboard Functions
    def keydown(self, vkey: int) -> None:
        """
        按下指定按键。

        Args:
            vkey: 键码 (建议使用模块提供的 KEY_* 常量)。
        """
        ...
    def keyup(self, vkey: int) -> None:
        """
        弹起指定按键。

        Args:
            vkey: 键码。
        """
        ...
    def keypress(self, vkey: int, ms: int) -> None:
        """
        点击按键 (按下 -> 延时 -> 弹起)。

        Args:
            vkey: 键码。
            ms: 按住持续时间 (毫秒)。
        """
        ...

    # Encrypted Keyboard Functions
    def enc_keydown(self, vkey: int) -> None:
        """[加密] 按下指定按键。"""
        ...
    def enc_keyup(self, vkey: int) -> None:
        """[加密] 弹起指定按键。"""
        ...
    def enc_keypress(self, vkey: int, ms: int) -> None:
        """[加密] 点击按键。"""
        ...

    # Monitor Control
    def monitor(self, port: int) -> None:
        """
        控制盒子开启或关闭监听推流。

        Args:
            port: 目标推流端口。设置为 0 表示停止推流。
                  (该端口应与 KmBoxNetMonitor 监听的端口一致)
        """
        ...

    # Masking Functions
    def mask_mouse_left(self, enable: bool) -> None:
        """
        屏蔽/解除屏蔽 物理鼠标左键。

        Args:
            enable: True 开启屏蔽 (物理按键无效), False 解除屏蔽。
        """
        ...
    def mask_mouse_right(self, enable: bool) -> None:
        """屏蔽/解除屏蔽 物理鼠标右键。"""
        ...
    def mask_mouse_middle(self, enable: bool) -> None:
        """屏蔽/解除屏蔽 物理鼠标中键。"""
        ...
    def mask_mouse_side1(self, enable: bool) -> None:
        """屏蔽/解除屏蔽 物理鼠标侧键1。"""
        ...
    def mask_mouse_side2(self, enable: bool) -> None:
        """屏蔽/解除屏蔽 物理鼠标侧键2。"""
        ...
    def mask_mouse_x(self, enable: bool) -> None:
        """屏蔽/解除屏蔽 物理鼠标X轴移动。"""
        ...
    def mask_mouse_y(self, enable: bool) -> None:
        """屏蔽/解除屏蔽 物理鼠标Y轴移动。"""
        ...
    def mask_mouse_wheel(self, enable: bool) -> None:
        """屏蔽/解除屏蔽 物理鼠标滚轮。"""
        ...
    def mask_keyboard(self, vkey: int) -> None:
        """
        屏蔽指定物理按键。

        Args:
            vkey: 要屏蔽的键码。
        """
        ...
    def unmask_keyboard(self, vkey: int) -> None:
        """
        解除屏蔽指定物理按键。

        Args:
            vkey: 要解除屏蔽的键码。
        """
        ...
    def unmask_all(self) -> None:
        """
        解除所有键盘和鼠标的屏蔽状态 (恢复正常)。
        """
        ...

    # System Configuration Functions
    def reboot(self) -> None:
        """重启设备。"""
        ...
    def set_config(self, ip: str, port: int) -> None:
        """
        修改设备的 IP 和端口配置。

        Args:
            ip: 新的 IP 地址 (如 "192.168.2.188")。
            port: 新的端口号。
        """
        ...
    def set_vid_pid(self, vid: int, pid: int) -> None:
        """
        修改设备的 VID 和 PID (需重启生效)。

        Args:
            vid: Vendor ID.
            pid: Product ID.
        """
        ...
    def lcd_color(self, rgb565: int) -> None:
        """
        使用指定颜色填充 LCD 屏幕。

        Args:
            rgb565: 16位 RGB565 颜色值。
        """
        ...
    def lcd_picture_bottom(self, buff: bytes) -> None:
        """
        在 LCD 底部显示 128x80 图片。

        Args:
            buff: 图片数据 (128 * 80 * 2 = 20480 字节)。
        """
        ...
    def lcd_picture(self, buff: bytes) -> None:
        """
        在 LCD 全屏显示 128x160 图片。

        Args:
            buff: 图片数据 (128 * 160 * 2 = 40960 字节)。
        """
        ...
    def trace_enable(self, type_: int, value: int) -> None:
        """
        开启/关闭 硬件轨迹修正算法。

        Args:
            type_: 算法类型 (0: 贝塞尔, 1: 导弹追踪, 2: 实时贝塞尔, 3: RM-RT)。
            value: 强度值 (<=0 关闭, >0 开启, 推荐 16-50)。
        """
        ...

# 键码常量定义
KEY_A: int
KEY_B: int
KEY_C: int
KEY_D: int
KEY_E: int
KEY_F: int
KEY_G: int
KEY_H: int
KEY_I: int
KEY_J: int
KEY_K: int
KEY_L: int
KEY_M: int
KEY_N: int
KEY_O: int
KEY_P: int
KEY_Q: int
KEY_R: int
KEY_S: int
KEY_T: int
KEY_U: int
KEY_V: int
KEY_W: int
KEY_X: int
KEY_Y: int
KEY_Z: int
KEY_1: int
KEY_2: int
KEY_3: int
KEY_4: int
KEY_5: int
KEY_6: int
KEY_7: int
KEY_8: int
KEY_9: int
KEY_0: int
KEY_ENTER: int
KEY_ESCAPE: int
KEY_BACKSPACE: int
KEY_TAB: int
KEY_SPACEBAR: int
KEY_CAPS_LOCK: int
KEY_F1: int
KEY_F2: int
KEY_F3: int
KEY_F4: int
KEY_F5: int
KEY_F6: int
KEY_F7: int
KEY_F8: int
KEY_F9: int
KEY_F10: int
KEY_F11: int
KEY_F12: int
KEY_LEFTCONTROL: int
KEY_LEFTSHIFT: int
KEY_LEFTALT: int
KEY_LEFT_GUI: int
KEY_RIGHTCONTROL: int
KEY_RIGHTSHIFT: int
KEY_RIGHTALT: int
KEY_RIGHT_GUI: int
