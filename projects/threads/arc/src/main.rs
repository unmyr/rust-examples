struct MyBucket {
    data: String,
    count: i32,
}

fn main() {
    let mut children = vec![];
    use std::sync::Arc;
    use std::thread;
    
    // This variable declaration is where its value is specified.
    let my_bucket = MyBucket {
        data: String::from("apple"),
        count: 0
    };
    let my_bucket = Arc::new(
        my_bucket
    );
    
    for i in 0..10 {
        // Here there is no value specification as it is a pointer to a reference
        // in the memory heap.
        let my_bucket = Arc::clone(&my_bucket);
        children.push(
            thread::spawn(move || {
                // As Arc was used, threads can be spawned using the value allocated
                // in the Arc variable pointer's location.
                println!("{:02}: {:?} {}", i, my_bucket.data, my_bucket.count);
            })
        );
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
        