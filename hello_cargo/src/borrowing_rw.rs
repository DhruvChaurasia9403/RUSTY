//rw here means read write 
pub fn borrow1(){
    let mut s1:String = String::from("This is string");
    let s2:String = String::from("this is string 2");
    //CASE-1 :the read read operation can be performed to the refrence variable anytime 
    
    let r1 = &s1;



    //CASE-2 :Read Write operation can also be performed with no issues 
    let mut s2:String = String::from("one");
    let r2 = &s2;
    println!("{}",r1);
    let w2 = &mut s2;
    w2.push_str(" two");
    println!("{}",w2);



    //CASE-3 :Write - Write here are not cross interfereing SEE CASE 4 FOR THE PROBLEM
    let mut s3:String = String::from("one");
    let w31 = &mut s3;
    w31.push_str(" two");
    println!("{}",w31);

    let w32 = &mut s3;
    w32.push_str(" three");
    println!("{}",w32);



    //CASE-4 :Write-Write problem 
    let mut s4:String = String::from("one");
    let w41 = &mut s4;
    w41.push_str(" two");
    let w42 = &mut s4;
    w42.push_str("three");
    // println!("{}",w41);
    println!("{}",w42);


}