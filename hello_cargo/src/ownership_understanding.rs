pub fn owner2(){
    let s1:String = give_s1_value(); 
    println!("Value of s1 is : {}",s1);


    let s2:String = String::from("World");
    let s3:String = double_play(s2);
    println!("value of s3 is : {}",s3);
}
fn give_s1_value()->String{
    let s2:String = String::from("Hello");
    return s2;
}
fn double_play(s4:String)->String{
    return s4;
}