pub fn run(){
    let s = "Rust is a Low-level programming language";
    println!("String s is: {}", s);//immutable string s 
    println!();
    // Demonstrating mutable string
    let mut st1:String = String::from("Now this is the example of a mutable string");
    st1.push_str("as here I append another String in the st itself");
    println!("example of both mutable and immutable string in Rust: {}",st1);
}