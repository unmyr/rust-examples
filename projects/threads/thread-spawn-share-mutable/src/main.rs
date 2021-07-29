use std::sync::Mutex;

struct MyBucket {
    data: String,
    count: Mutex<i32>,
}

impl MyBucket {
    fn increment(&self) -> i32{
        let mut ref_count = self.count.lock().unwrap();
        *ref_count += 1;
        *ref_count
    }

    #[allow(dead_code)]
    fn decrement(&self) -> i32{
        let mut ref_count = self.count.lock().unwrap();
        *ref_count -= 1;
        *ref_count
    }
}

fn main() {
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    let mut children = vec![];
    
    // This variable declaration is where its value is specified.
    let my_bucket = Arc::new(
        MyBucket {
            data: String::from("apple"),
            count: Mutex::new(0)
        }
    );
    
    for i in 0..10 {
        // Here there is no value specification as it is a pointer to a reference
        // in the memory heap.
        let my_bucket = Arc::clone(&my_bucket);
        let handle = thread::spawn(move || {
            // As Arc was used, threads can be spawned using the value allocated
            // in the Arc variable pointer's location.
            let mut local_count: i32;
            local_count = my_bucket.increment();
            println!("{:02}: {:?} {:2}", i, my_bucket.data, local_count);

            thread::sleep(Duration::from_secs(2));

            {
                let mut count = my_bucket.count.lock().unwrap();
                *count -= 1;
                local_count = *count
            }
            println!("{:02}: {:?} {:2}", i, my_bucket.data, local_count);
        });
        children.push(handle);
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
