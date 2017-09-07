#[macro_use]
extern crate helix;

ruby! {
    class HelloHelix {
        def hello() {
            println!("Hello from hello_helix!");
        }
    }
}
