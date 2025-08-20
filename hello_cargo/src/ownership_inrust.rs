pub fn owner(){
    println!("The owner ship is explained here very exceptionally");
    let str1 = "This variable has stack memory allocation and its name is str1";
    let mut str2:String = String::from("This is heap/Dynamic memory allocation and its name is str2");
    println!("ABOUT str1 : {}",str1);
    println!("ABOUT str2 : {} \n the str2 can also be appended with other string too ",str2);

    //appending str2 with some random string
    str2.push_str(", and i have used .push_str to append this sentence, but sam i cannot do with the str1");
    println!("{}",str2);
    
    //what if i try to store the heap alloted str2 in some stack alloted string 
    // let str3:&str = str2;   
                     //^^^^ expected `&str`, found `String`
    
}