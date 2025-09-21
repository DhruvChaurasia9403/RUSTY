pub fn tup(){
    //this is the declaration of tupple 
    let emp_info:(&str , u8) = ("Dhruv Chaurasia",20);
    

    //now there are two ways to assign the tupple values to the local variables 
    // one->
    let emp_name = emp_info.0;
    let emp_age = emp_info.1;

    println!("name of the Employee is : {} , his age is {}",emp_name,emp_age);
    println!();
    //2
    let (_employee_name,_employee_age) = emp_info;
    println!("{} is the best employee, and he is just {}" , _employee_name,_employee_age);


}