use std::{thread,time};

fn main() {
    let pause = time::Duration::from_millis(20);
    let handel1 = thread::spawn(move ||{
        thread::sleep(pause);
    });
    let handel2 = thread::spawn(move ||{
        thread::sleep(pause);
    });
    handel1.join().unwrap();
    handel2.join().unwrap();
}