# KmBoxNet

[![PyPI](https://img.shields.io/pypi/v/kmbox-net)](https://pypi.org/project/kmbox-net/)

高性能的 KmBoxNet (网络版) Python 绑定库，基于 Rust 开发。

High-performance Python bindings for KmBoxNet, written in Rust.

## 功能特性 (Features)

*   🚀 **高性能**: 核心通讯逻辑由 Rust 实现，极低延迟。
*   🖱️ **全功能控制**: 支持鼠标移动、点击、滚轮，键盘按键控制。
*   🔐 **加密通信**: 支持加密协议，防止数据包特征检测。
*   👀 **硬件监听**: 支持监听物理键鼠的输入事件 (Monitor模式)。
*   🖼️ **LCD控制**: 支持修改屏幕颜色、显示图片。
*   🔧 **硬件算法**: 内置贝塞尔曲线等硬件级轨迹修正算法。

## 安装 (Installation)

使用 pip 安装：

```bash
pip install kmbox-net
```

*注意：在代码中导入时请使用 `import kmbox_net`。*

## 快速开始 (Quick Start)

### 1. 连接设备与控制

```python
import kmbox_net
import time

# 连接设备
# IP: 盒子显示的IP (例如 192.168.2.188)
# Port: 端口 (默认 8888)
# MAC: 盒子MAC地址 (例如 "0B50E466")，用于握手和加密
client = kmbox_net.KmBoxNetClient("192.168.2.188", 8888, "0B50E466")

# 鼠标相对移动 (X=100, Y=100)
client.mouse_move(100, 100)

# 鼠标左键点击
client.mouse_left(True)  # 按下
time.sleep(0.05)
client.mouse_left(False) # 松开

# 键盘输入 (按下 'A')
client.keydown(kmbox_net.KEY_A)
time.sleep(0.05)
client.keyup(kmbox_net.KEY_A)
```

### 2. 监听物理键鼠 (Monitor)

```python
import kmbox_net
import time

# 定义回调函数
def on_event(mouse: kmbox_net.HardMouse, keyboard: kmbox_net.HardKeyboard):
    # 打印鼠标数据
    if mouse.buttons != 0 or mouse.x != 0 or mouse.y != 0:
        print(f"[Mouse] Btn:{mouse.buttons} X:{mouse.x} Y:{mouse.y}")
    
    # 打印键盘数据
    if keyboard.data:
        print(f"[Keyboard] Keys:{keyboard.data}")

# 1. 在本地端口 12345 开启监听线程
monitor = kmbox_net.KmBoxNetMonitor(12345, on_event)

# 2. 告诉盒子把数据推流到这个端口
# 注意：你需要先连接上盒子
client = kmbox_net.KmBoxNetClient("192.168.2.188", 8888, "0B50E466")
client.monitor(12345)

print("正在监听物理键鼠输入... 按 Ctrl+C 停止")
try:
    while True:
        time.sleep(1)
except KeyboardInterrupt:
    client.monitor(0) # 停止推流
    monitor.shutdown() # 停止本地监听
```

### 3. 加密功能 (Encryption)

```python
import kmbox_net

# 使用加密通道移动鼠标 (防止抓包特征)
client.enc_mouse_move(100, 100)

# XXTEA 加密工具函数 (原地修改)
data = bytearray(128)
key = b"1234567890123456"
kmbox_net.xxtea_encrypt(data, key)
```

## 编译指南 (Building from source)

如果你需要自己编译项目，需要安装 Rust 工具链。

```bash
# 安装 maturin
pip install maturin

# 编译并安装到当前环境
maturin develop --release
```

## License

MIT