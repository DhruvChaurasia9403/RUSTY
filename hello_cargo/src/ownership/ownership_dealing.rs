// pub fn owner3(){
//     let s1:String = String::from("Hello");
//     let length:usize = find_lenght(s1);                           // HERE THE S2 TRANSFERED OWNERSHIP AND WAS DELETED
//     println!("the value of string {} is : {}",s1,length);         // THIS WILL SHOW ERROR AS THE S1 NO LONGER EXIST 
// }
// fn find_lenght(s:String)->usize{
//     let length:usize = s.len();
//     return length;
// }

//---------------------METHOD 1: make tuple--------------------------//

// pub fn owner3(){
//     let s1:String = String::from("Hello");
//     let (x,length) = find_lenght(s1);
//     println!("the value of string {} is : {}",s1,length);
// }
// fn find_lenght(s:String)->(String,usize){
//     let length:usize = s.len();
//     return (s,length);
// }





//-------------------------METHOD 2 : Cloning ------------------------//
pub fn owner3(){
    let s1:String = String::from("Hello");
    let length:usize = find_lenght(s1.clone());                       // INSTEAD OF SENDING THE S1 DIRECTLY PASS THE CLONE AS A ARGUMENT 
    println!("the value of string {} is : {}",s1,length);             // THE S1 IS NOT GETTING DELETED ITS JUST SENDING ANOTHER VARIABLE WITH SAME VALUE
}
fn find_lenght(s:String)->usize{
    let length:usize =  s.len();
    return length;
}