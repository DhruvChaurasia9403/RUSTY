pub fn borrow(){
    let s1:String = String::from ("Hello");
    let length:usize = cal_length(& s1);                      //here we are passing the refrence of the s1 not the string s1 itself 
    println!("the size of String {} is : {}",s1,length);

    let mut s2:String = String::from("Hemmllo");
    let (new_string,new_length) = cal_and_append(&mut s2);
    println!("This is new String {} of size : {}",new_string,new_length);
}
fn cal_length(s:&String)->usize{
    let length = s.len();
    return length;
}


// PROBLEM : But there is a problem with passing the refrence beacuse only the read operations are allowed to the refrence variable 
// SOLUTION : using  --------->>>   &mut s1 instead of s1 itself that way we are telling that we can use the refrence varible for the write operations too.



fn cal_and_append(s:&mut String)->(&mut String , usize){
    s.push_str("_World");
    let new_length:usize = s.len();
    return (s,new_length);
}