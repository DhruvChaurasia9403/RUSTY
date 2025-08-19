mod string_demo;
mod tupple_demo;

fn main() {
    println!("Hello, world!");
    let num:u8 = 5;

    println!("the num stores an integer value equals : {}", num);
    println!();

    // Demonstrating string usage &str 
    let s = "Rust is a Low-level programming language";
    println!("String s is: {}",s);
    println!();

    
    // Demonstrating mutable string
    let mut s1 = String::from("Rust is a Low-level programming language");
    s1.push_str(" and it is also a High-level programming language");
    println!("Updated string s1 is: {}", s1);



    println!();
    println!();
    println!("String started");
    println!("--------------START----------------");
    string_demo::run();
    println!("----------------END----------------");
    println!();
    println!();

    println!("Tupple Started");
    println!("--------------START----------------");
    tupple_demo::tup();
    println!("----------------END----------------");
    println!();
    println!();
}