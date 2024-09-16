use std::time::Duration;

#[derive(Debug)]
// The 'a is a lifetime parameter that ensures the &str reference in each variant of the enum
// is valid for at least as long as the lifetime 'a.
// This allows the ChannelMsg enum to carry references to strings that live outside of the enum itself,
// rather than owning the strings.
//
// By using &'a str, we can pass around string slices without taking ownership,
// making the code more efficient as it avoids unnecessary cloning. The lifetime 'a guarantees
// that the reference remains valid while the ChannelMsg instance is in use.
pub enum ChannelMsg<'a> {
    Increment { thread: &'a str, count: u32 },
    Decrement { thread: &'a str, count: u32 },
}

pub fn increment(thread: &str, count: u32) -> ChannelMsg {
    ChannelMsg::Increment { thread, count }
}

pub fn decrement(thread: &str, count: u32) -> ChannelMsg {
    ChannelMsg::Decrement { thread, count }
}

pub fn run(buffer_size: usize) {
    // Create a channel with a buffer size of buffer_size
    // This means that the channel can hold at most 1 message at a time
    // If the channel is full, the sender will block until the receiver consumes a message
    let (tx, rx) = std::sync::mpsc::sync_channel::<ChannelMsg>(buffer_size);

    // We need to clone the sender to be able to send it to multiple threads
    // Since each thread have the `move` keyword, they take ownership of the sender
    let tx_1 = tx.clone();

    // Spawn a thread that sends a message
    // We use `move` here to move ownership of `tx` into the thread
    std::thread::spawn(move || {
        let thread = "thread 1";

        std::thread::sleep(Duration::from_secs(1));
        tx_1.send(increment(thread, 20)).unwrap();

        std::thread::sleep(Duration::from_secs(1));
        tx_1.send(decrement(thread, 1)).unwrap();

        std::thread::sleep(Duration::from_secs(1));
        tx_1.send(increment(thread, 2)).unwrap();
        tx_1.send(increment(thread, 1)).unwrap();

        // Try to send a message without blocking
        // If the channel is full, this will return an error
        // Try increasing/decresing the buffer_size to see the difference
        if let Err(err) = tx_1.try_send(decrement(thread, 1)) {
            println!("{err:?}");
        }
    });

    let tx_2 = tx.clone();
    std::thread::spawn(move || {
        let thread = "thread 2";

        tx_2.send(increment(thread, 2)).unwrap();
        std::thread::sleep(Duration::from_secs(1));
    });

    // Send a message from the main thread
    tx.send(increment("main", 100)).unwrap();

    // We need to drop the last live sender to be able to receive messages
    // The program will not complete if we comment this out
    // **All** `tx` needs to be dropped for `rx` to have `Err`.
    // The tx_1 and tx_2 are dropped when the threads finish execution
    // The main thread will drop the last tx
    drop(tx);

    let mut counter = 0;
    while let Some(msg) = rx.recv().ok() {
        match msg {
            ChannelMsg::Increment { thread, count } => {
                println!("[{thread:?}] incremented {count:?}");
                counter += count;
            }
            ChannelMsg::Decrement { thread, count } => {
                println!("[{thread:?}] decremented {count:?}");
                counter -= count;
            }
        }
    }

    println!("Counter: {}", counter);
}
