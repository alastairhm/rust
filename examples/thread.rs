fn main () {
    use std::thread;

    let handle = thread::spawn(|| { println!("From a thread!"); });
    println!("Before a thread!");
    handle.join();
}
