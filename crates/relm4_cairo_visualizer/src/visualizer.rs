use std::{
    fs::File,
    io::{Read, Write},
    process::Stdio,
    sync::mpsc::Receiver,
};

pub struct Visualizer;

// This is defined as a macro so we can replace the `{0}` with the number of bars
// This way we can just call format!(cava_config!(), bars) and get the correct configuration
// You can play around with the configuration to get different effects
macro_rules! cava_config {
    () => {
        r#"
        [general]
        bars = {0}
        framerate = 60

        [output]
        method = raw
        channels = mono
        raw_target = /dev/stdout
        data_format = binary
        bit_format = 16bit

        [smoothing]
        integral = 70
        monstercat = 0
        waves = 1
        gravity = 100
"#
    };
}

impl Visualizer {
    pub fn new(bars: usize) -> Receiver<Vec<u16>> {
        // Create a new temporary configuration for cava and save it to
        // `/tmp/cava-config.conf` so we can pass it as a argument to cava
        let path = std::env::temp_dir().join("cava-config.conf");
        let config = format!(cava_config!(), bars);
        let mut temp = File::create(&path).unwrap();
        temp.write_all(config.as_bytes()).unwrap();
        temp.flush().unwrap();

        // Spawn the cava process with the configuration file
        let mut process = std::process::Command::new("cava")
            .arg("-p")
            .arg(&path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::null())
            .spawn()
            .unwrap();

        // Create a channel so we can send data from the cava process to the main thread
        // The `rx` will be returned to the UI so it can get the data
        let (tx, rx) = std::sync::mpsc::channel::<Vec<u16>>();
        std::thread::spawn(move || {
            // Initialize a buffer to receive raw data from cava
            // Buffer size is 2 * bars because:
            // - Each bar's data is represented by 2 bytes (16 bits)
            // - We need to accommodate data for all bars
            // Process:
            // 1. Read raw binary data into this buffer
            // 2. Convert buffer contents to Vec<u16>
            // 3. Send Vec<u16> to main thread for UI updates
            // This approach allows for efficient data transfer and easy iteration in the UI
            let mut buf = vec![0_u8; 2 * bars];

            // Get the stdout of the cava process so we can read its output
            let stdout = process.stdout.as_mut().unwrap();

            loop {
                // Initialize a vector to store data received from cava
                // Each element is a u16 (16-bit unsigned integer) representing the amplitude of a frequency bar
                // The vector is pre-filled with zeros and has a length equal to the number of bars
                // Rust's type inference allows us to use 0_u16 to specify the element type
                let mut data = vec![0_u16; bars];

                // Read the data from stdout into the buffer
                // We are reading the exact amount of bytes that we need
                let read_res = stdout.read_exact(&mut buf);
                if let Err(e) = read_res {
                    let stderr = process.stderr.as_mut().unwrap();
                    let mut stderr_contents = String::new();
                    stderr.read_to_string(&mut stderr_contents).unwrap();
                    panic!(
                        "cava proccess panicked: {:?} Error: {:?}",
                        stderr_contents, e
                    );
                }

                // Convert the raw binary buffer into a Vec<u16>
                // Each pair of bytes in the buffer represents one u16 value
                // This conversion is necessary because cava outputs data in binary format
                // (as specified in the cava configuration: `data_format = binary` and `bit_format = 16bit`)
                for i in 0..data.len() {
                    data[i] = u16::from_le_bytes([buf[2 * i], buf[2 * i + 1]]);
                }

                // Apply a simple moving average smoothing
                // This part makes the bars look smoother, like when we are drawing and you use ours finger to blend colors together
                let window_size = 3; // This is like how many bars we look at to make each bar smoother
                let mut smoothed_data = vec![0_u16; bars]; // We make a new list to put our smoother bars in
                for i in 0..bars {
                    // For each bar, we look at the bars next to it
                    let start = i.saturating_sub(window_size / 2); // We start looking a little bit before our bar
                    let end = (i + window_size / 2 + 1).min(bars); // We stop looking a little bit after our bar

                    // We add up the heights of all these bars
                    let sum: u32 = data[start..end].iter().map(|&x| x as u32).sum();

                    // Then we divide by how many bars we looked at to get an average
                    // This average becomes the new height of our bar
                    smoothed_data[i] = (sum / (end - start) as u32) as u16;
                }

                // And finally we send the smoothed data to the UI
                tx.send(smoothed_data).unwrap();
            }
        });

        rx
    }
}
