// inside Collections/
mod collections {
    pub mod string_demo;
    pub mod tupple_demo;
    pub mod arrays;
    pub mod vector;
}

mod loops{
    pub mod while_loop;
    pub mod for_loop;
}

// inside Functions/
mod functions {
    pub mod functions_demo;
}

// inside Ownership/
mod ownership {
    pub mod ownership_inrust;
    pub mod ownership_understanding;
    pub mod ownership_dealing;
}

// inside Borrowing/
mod borrowing {
    pub mod borrowing;
    pub mod borrowing_rw;
}

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
    collections::string_demo::run();
    println!("----------------END----------------");
    println!();
    println!();

    println!("Tupple Started");
    println!("--------------START----------------");
    collections::tupple_demo::tup();
    println!("----------------END----------------");
    println!();
    println!();



    println!("Functions Started");
    println!("--------------START----------------");
    functions::functions_demo::fun();
    println!("----------------END----------------");
    println!();
    println!();



    println!("Ownership in Rust Started");
    println!("--------------START----------------");
    ownership::ownership_inrust::owner();
    println!("----------------END----------------");
    println!();
    println!();



    println!("Ownership in Rust Rule 2 : there can only be one owner at a time");
    println!("--------------START----------------");
    ownership::ownership_understanding::owner2();
    println!("if you will try to print s2 it will throw error because once the s2 was the owner of the \"world\"\n,but now its not and rust has thrown it in garbage!!");
    println!("----------------END----------------");
    println!();
    println!(); 



    println!("Ownership Avoidance");
    println!("--------------START----------------");
    println!("we need to avoid ownership sometimes \nwhy?\nsee the file and find out\nhow?\nit has 3 methods, see the file ownership_dealing");
    println!("Method 1 : making tupple\nMethod 2 : cloning\nMethod 3 : Borrowing - (Next File)");
    ownership::ownership_dealing::owner3();
    println!("----------------END----------------");
    println!();
    println!();



    println!("Borrowing in Rust Started");
    println!("--------------START----------------");
    println!("This is the concept of borrowing istead of cloning we use & (only allows the read operations) and &mut (allows the read and write operations both)");
    println!("read more in borrowing.rs");
    borrowing::borrowing::borrow();
    println!("----------------END----------------");
    println!();
    println!();




    println!("Borrowing Read Write operations");
    println!("--------------START----------------");
    println!("read more in borrowing_rw.rs");
    borrowing::borrowing_rw::borrow1();
    println!("----------------END----------------");
    println!();
    println!();



    println!("Arrays");
    println!("--------------START----------------");
    println!("read more in arrays.rs");
    collections::arrays::arr();
    println!("----------------END----------------");
    println!();
    println!();


    println!("Vector");
    println!("--------------START----------------");
    println!("read more in vector.rs");
    collections::vector::ve();
    println!("----------------END----------------");
    println!();
    println!();



    println!("Loops");
    println!("--------------START----------------");
    println!("read more in while_loop.rs");
    loops::while_loop::lp();
    println!("----------------END----------------");
    println!();
    println!();



    println!("FOR loop");
    println!("--------------START----------------");
    println!("read more in for_loop.rs");
    loops::for_loop::fr();
    println!("----------------END----------------");
    println!();
    println!();
}

