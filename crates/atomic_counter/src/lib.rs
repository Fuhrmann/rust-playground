pub fn run() {
    // Creates a new atomic counter with an initial value of 0
    // The counter is a thread-safe integer that can be shared between threads
    // The counter can be incremented, decremented, and read atomically
    // The counter is a wrapper around a `usize` that provides atomic operations
    // However since ours threads take ownership of the counter, we need to use Arc to share it
    let atomic_counter = std::sync::atomic::AtomicUsize::new(0);

    // Create a new Arc from the atomic counter
    // We need to use Arc to share the counter between threads
    // Arc means Atomic Reference Counted and is a thread-safe reference-counted smart pointer
    let arc_counter = std::sync::Arc::new(atomic_counter);

    // Clone the Arc to be able to send it to multiple threads
    // Since each thread have the `move` keyword, they take ownership of the Arc
    // The clone operation only increments the reference count of the Arc
    // Since Arc is a reference-counted smart pointer, it will be dropped when the last reference is dropped
    let counter_clone = arc_counter.clone();
    std::thread::spawn(move || {
        for _ in 0..10 {
            counter_clone.fetch_add(100, std::sync::atomic::Ordering::SeqCst);
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    });

    let counter_clone = arc_counter.clone();
    std::thread::spawn(move || {
        for _ in 0..10 {
            counter_clone.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    });

    let counter_clone = arc_counter.clone();
    std::thread::spawn(move || loop {
        let value = counter_clone.load(std::sync::atomic::Ordering::SeqCst);
        // print and clear stdout current value
        print!("\r Counter: {}", value);
    })
    // We need to wait here for the threads to finish
    // So the program doesn't exit before the threads finish
    .join()
    .unwrap();
}
