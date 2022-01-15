struct MyBucket {
    data: String,
    count: i32,
}

impl Drop for MyBucket {
    fn drop(&mut self) {
        println!("Dropping MyBucket with data `{}`!", self.data);
    }
}

fn main() {
    let mut children = vec![];
    use std::sync::Arc;
    use std::thread;

    // This variable declaration is where its value is specified.
    let my_bucket = Arc::new(
        MyBucket {
            data: String::from("apple"),
            count: 0
        }
    );

    for i in 0..10 {
        // Here there is no value specification as it is a pointer to a reference
        // in the memory heap.
        let my_bucket = Arc::clone(&my_bucket);
        let handle = thread::spawn(move || {
            // As Arc was used, threads can be spawned using the value allocated
            // in the Arc variable pointer's location.
            println!("{:02}: {:?} {}", i, my_bucket.data, my_bucket.count);
        });
        children.push(handle);
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
