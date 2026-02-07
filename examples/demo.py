import time

import kmbox_net

if __name__ == "__main__":
    # 连接设备
    client = kmbox_net.KmBoxNetClient("192.168.2.188", 8888, "0B50E466")

    # 鼠标移动
    client.mouse_move(100, 100)
    time.sleep(1)
    client.mouse_move(-100, -100)

    # 键盘输入
    client.keydown(kmbox_net.KEY_LEFTSHIFT)
    client.keypress(kmbox_net.KEY_H, 50)
    client.keyup(kmbox_net.KEY_LEFTSHIFT)

    # 监控功能
    def on_event(mouse, keyboard):
        if mouse.buttons != 0 or mouse.x != 0 or mouse.y != 0:
            print(f"Mouse: {mouse.x}, {mouse.y}, btn={mouse.buttons}")

        if keyboard.buttons != 0 or len(keyboard.data) > 0:
            print(f"Keyboard: {keyboard.data}")

    monitor = kmbox_net.KmBoxNetMonitor(12345, on_event)
    client.monitor(12345)  # 开启设备监控流

    print("Monitoring... Press Ctrl+C to stop")
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        monitor.shutdown()
        client.monitor(0)  # 关闭设备监控流
