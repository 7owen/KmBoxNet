use kmbox_net::{keys, KmBoxNetClient, KmBoxNetMonitor};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting KmBox Demo...");

    // Device configuration
    // Replace these values with your actual device details found on the device screen
    let ip = "192.168.2.188";
    let port = 8888;
    let mac = "0B50E466"; // Can be formatted as "12:34:56:78" or "12345678"
    let monitor_port = 12345; // Local port to receive monitor data

    println!("Connecting to {}:{} (MAC: {})...", ip, port, mac);

    // Initialize connection
    // This performs a handshake with the device
    let mut kmbox = match KmBoxNetClient::new(ip, port, mac) {
        Ok(client) => {
            println!("Connected successfully!");
            client
        }
        Err(e) => {
            eprintln!("Failed to connect to device: {:?}", e);
            return Ok(());
        }
    };

    // Ensure we are in a clean state
    kmbox.unmask_all()?;

    // --- Mouse Demonstration ---
    println!("\nTesting Mouse Control...");

    // 1. Basic relative movement
    println!("> Moving mouse right and down (100, 100)");
    kmbox.mouse_move(100, 100)?;
    thread::sleep(Duration::from_millis(500));

    println!("> Moving mouse left and up (-100, -100)");
    kmbox.mouse_move(-100, -100)?;
    thread::sleep(Duration::from_millis(500));

    // 2. Mouse Clicks
    println!("> Clicking left mouse button");
    kmbox.mouse_left(true)?; // Press
    thread::sleep(Duration::from_millis(50));
    kmbox.mouse_left(false)?; // Release
    thread::sleep(Duration::from_millis(500));

    // 3. Firmware automated movement (smoother)
    println!("> Auto-move: 300px right over 1000ms");
    kmbox.mouse_move_auto(300, 0, 1000)?;
    // Wait for movement to finish
    thread::sleep(Duration::from_millis(1200));

    // 4. Encrypted Mouse Movement (anti-detection)
    println!("> Encrypted move: 300px left");
    kmbox.enc_mouse_move(-300, 0)?;
    thread::sleep(Duration::from_millis(500));

    // --- Keyboard Demonstration ---
    println!("\nTesting Keyboard Control...");
    println!("> Typing 'Hello'...");

    // Helper closure for typing
    let type_key =
        |client: &mut KmBoxNetClient, key: u8| -> Result<(), Box<dyn std::error::Error>> {
            client.keypress(key as i32, 50)?; // Press for 50ms
            thread::sleep(Duration::from_millis(50)); // Wait between keys
            Ok(())
        };

    // Shift + h for 'H'
    kmbox.keydown(keys::KEY_LEFTSHIFT as i32)?;
    type_key(&mut kmbox, keys::KEY_H as u8)?;
    kmbox.keyup(keys::KEY_LEFTSHIFT as i32)?;

    // 'e', 'l', 'l', 'o'
    type_key(&mut kmbox, keys::KEY_E as u8)?;
    type_key(&mut kmbox, keys::KEY_L as u8)?;
    type_key(&mut kmbox, keys::KEY_L as u8)?;
    type_key(&mut kmbox, keys::KEY_O as u8)?;

    // --- Monitor Demonstration ---
    println!("\nStarting Monitor...");

    // 1. Start local listener using the independent KmBoxMonitor class
    // This runs in a separate thread and calls the callback when data arrives
    println!("> Starting local monitor listener on port {}", monitor_port);
    let _monitor = KmBoxNetMonitor::start(monitor_port, |mouse, keyboard| {
        // Check for any mouse activity
        if mouse.buttons != 0 || mouse.x != 0 || mouse.y != 0 || mouse.wheel != 0 {
            println!(
                "[Monitor] Mouse    | Buttons: 0x{:02X} | X: {:4} | Y: {:4} | Wheel: {:4}",
                mouse.buttons, mouse.x, mouse.y, mouse.wheel
            );
        }

        // Check for any keyboard activity
        let mut pressed_keys = Vec::new();
        for &k in keyboard.data.iter() {
            if k != 0 {
                pressed_keys.push(k);
            }
        }

        if keyboard.buttons != 0 || !pressed_keys.is_empty() {
            println!(
                "[Monitor] Keyboard | Modifiers: 0x{:02X} | Keys: {:?}",
                keyboard.buttons, pressed_keys
            );
        }
    })?;

    // 2. Enable monitoring on device side (tell device to stream to our port)
    println!("> Enabling monitoring on device...");
    kmbox.monitor(monitor_port)?;

    println!("> Please use your PHYSICAL mouse and keyboard now.");
    println!("> Monitoring enabled. Press Ctrl+C to stop.");

    // Keep the main thread alive indefinitely to allow monitoring to continue
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
