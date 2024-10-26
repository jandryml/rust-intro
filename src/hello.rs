fn hello_fn() {
    println!("Hello, world!");
}

pub(crate) fn do_something() {
    hello_fn();
}
