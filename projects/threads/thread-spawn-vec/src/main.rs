use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let vec_arc =  Arc::new(Mutex::new(Vec::<u32>::new()));

    let vec = Arc::clone(&vec_arc);
    let future1 = thread::spawn(move || {
        let mut v = vec.lock().unwrap();
        v.push(1);
        println!("thread A: {:?}", v);
    });

    let vec = Arc::clone(&vec_arc);
    let future2 = thread::spawn(move || {
        let mut v = vec.lock().unwrap();
        v.push(2);
        println!("thread B: {:?}", v);
    });

    future1.join().unwrap();
    future2.join().unwrap();

    let mut sum: u32 = 0;
    for n in vec_arc.lock().unwrap().iter() {
        sum += n;
    }
    println!("total={:?}", sum);
}
