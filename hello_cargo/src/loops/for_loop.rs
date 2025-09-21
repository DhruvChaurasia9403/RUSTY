pub fn fr(){
    //iteration in range
    println!("Iteration through range");
    for i in 1..5{
        print!("{}",i);
    }println!();


    //iteration in range reverse
    println!("Iteration through range reverse");
    for i in (1..5).rev(){
        print!("{}",i);
    }
    println!();


    fr1();
    println!();
    fr2();
    println!();
}
fn fr1(){

    // transfer of ownership
    println!("Iteration through arr:ownersip transfer");
    let arr:[usize;5] = [1,2,3,4,5];
    for i in arr {
        println!("{}",i);
    }
    println!();
    println!("Iteration through arr: No ownersip transfer");
    // No transfer of ownership 
    let arr:[char;5] = ['h','e','l' ,'l' ,'o'];
    for i in arr.iter(){
        print!("{} ",i);
    }
    println!();println!();

    //mutating the vector
    println!("Iteration through vector: Mutable Reference");
    let mut vc = vec!['c','h','a','n','g','e'];
    let mut x = String::new();
    for i in vc.iter_mut(){
        x.push(*i); // why use *i ? because vs.iter_mut() only gives the mutable refrence stored in the i so we use *i for the value stored at address (star of i)
        println!("{}",x);
    }
    println!();
}

fn fr2(){
    println!("Iteration of index and component of array");
    let names = ["abc","def","ghi"];
    for (index , name) in names.iter().enumerate(){
        println!("name:{},rank:{}" ,name,index);
    }
    println!();
}