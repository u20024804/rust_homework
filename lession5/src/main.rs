mod macros;

fn main() {
    println!("{}", longer_string!("hello", "world!"));
    println!("{}", longer_string!("hello", "world", "helloworld!"));
    println!("Hello, world!");
}
