fn main() {
    let mut children = vec![];
    use std::thread;

    for i in 0..10 {
        children.push(
            thread::spawn(move || {
                println!("{:02}: Hello thread", i);
            })
        );
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
