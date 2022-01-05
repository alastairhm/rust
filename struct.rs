struct Foo {
    quax: i32,
    baz: String,
}

fn main() {
    let a = Foo {
        quax: 10,
        baz: String::from("Hello, world!"),
    };

    println!("Foo: quax: {} baz: {}", a.quax, a.baz);
}
