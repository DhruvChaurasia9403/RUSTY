pub fn owner2(){
    let s1:String = give_s1_value();//           STEP 2 : s1 is owner and x is sent to garbage
    println!("Value of s1 is : {}",s1);


    let s2:String = String::from("World");//     STEP 3 : s3 is the owner of "world"
    let s3:String = double_play(s2);//       STEP 5 : s3 is the new owner and s4 is sent to grabage
    println!("value of s3 is : {}",s3);
}
fn give_s1_value()->String{//                    STEP 1 : x is the owner of "hello" 
    let x:String = String::from("Hello");
    return x;
}
fn double_play(s4:String)->String{//             STEP 4 : s4 is the new owner and s2 is sent to grabage
    return s4;
}