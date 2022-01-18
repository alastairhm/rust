use hello::say_hello;

mod hello {
    pub fn say_hello() {
        println!("Hello, world!")
    }
}

fn main() {
    say_hello()
}
