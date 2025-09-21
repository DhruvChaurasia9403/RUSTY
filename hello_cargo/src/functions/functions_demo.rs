pub fn fun(){
    println!("This is the demo of functions in RUST by using example of printing addition of two numbers");
    let num1:u8 = 10;
    let num2:u8 = 20;
    add(num1,num2);
    println!("Above sum was just function acalling no return type ");
    println!("below is the sum return but using ->return datatype");
    let sum:u8 = add1(num1,num2);
    println!("Value of sum of 10+20 is :{}",sum);
    println!();
}

//here we just call the function no return type
fn add(n1:u8 , n2:u8){
    println!("Value of sum of 10+20 is :{}",n1+n2);
}
 

fn add1(n1:u8 , n2:u8)->u8{
    return n1+n2;
}