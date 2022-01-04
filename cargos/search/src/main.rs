extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    println!("{:?}", ctx.get_contents());
    ctx.set_contents("some string".to_owned()).unwrap();
}