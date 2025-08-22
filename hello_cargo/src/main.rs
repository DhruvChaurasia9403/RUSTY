mod string_demo;
mod tupple_demo;
mod functions_demo;
mod ownership_inrust;
mod ownership_understanding;
mod ownership_dealing;

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



    println!("Functions Started");
    println!("--------------START----------------");
    functions_demo::fun();
    println!("----------------END----------------");
    println!();
    println!();



    println!("Ownership in Rust Started");
    println!("--------------START----------------");
    ownership_inrust::owner();
    println!("----------------END----------------");
    println!();
    println!();



    println!("Ownership in Rust Rule 2 : there can only be one owner at a time");
    println!("--------------START----------------");
    ownership_understanding::owner2();
    println!("if you will try to print s2 it will throw error because once the s2 was the owner of the \"world\"\n,but now its not and rust has thrown it in garbage!!");
    println!("----------------END----------------");
    println!();
    println!(); 



    println!("Ownership Avoidance");
    println!("--------------START----------------");
    println!("we need to avoid ownership sometimes \nwhy?\nsee the file and find out\nhow?\nit has 3 methods, see the file ownership_dealing");
    println!("Method 1 : making tupple\nMethod 2 : cloning\nMethod 3 : Borrowing - (Next File)");
    ownership_dealing::owner3();
    println!("----------------END----------------");
    println!();
    println!();
}

