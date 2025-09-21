pub fn arr(){
    let mut arr:[&str;3] = ["what", "are", "you"];
    write_arr(&mut arr);
    println!("{:?}",arr);
}
fn write_arr(arr:&mut [&str;3]){
    arr[0] = "how";
    println!("{:?}",arr);
}