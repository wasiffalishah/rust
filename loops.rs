fn main() {
    
    let num : [i8;7] = [23,67,89,45,34,45,54];
    
    for &n in &num{
        println!("{}",n);
    }
    
    myforloop();
    mywhileloop();
}

fn myforloop() {
    for i in 0 .. 11 {
        println!("{}",i);
    }
}

fn mywhileloop(){
    let mut count = 0;

    while count <= 5 {
        println!("Count: {}", count);
        count += 1;
    }
}
