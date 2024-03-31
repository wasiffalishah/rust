fn main(){
    
    let wasif = 789;
    
    match wasif {
        70 => println!("CD 7O"),
        125 => println!("CG 125"),
        150 => println!("SUKUZI 150"),
        200 => println!("HONDA 200"),
        500..=1200 => println!("500 se 1200 CC"),
        _ => println!("OTHER"),
    }
}
