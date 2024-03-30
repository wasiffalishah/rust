fn main(){
    let mynamearray = ["wasif","mehmod", "ali", "shah"];
    let mynameslice = &mynamearray[1.. 3]; //first number is inclusive means included and 2 number is exlusive means excluded
    
    println!("{:?}", mynameslice);
    println!("{:?}", mynamearray);
}

