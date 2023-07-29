mod macros;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    println!("{}", longer_string!("hello", "world!"));
    println!("{}", longer_string!("hello", "world", "helloworld!"));
    //println!("{}", longer_string!(1, 2, 3));
    println!("{:?}", longer_string!(vec![1], vec![1,2], vec![1,2,3]));
    println!("Hello, world!");

    let greeting_file: File = File::open("hello.txt")?;
    Ok(())
}
