fn main(){
    let x = 67;                 //table you want
    let n = 10;                 //ending point
    
    let mut c : i128 = x; 
    let mut z = 1;      
    let f = 1;
    let m = x*100;
    
    while c <=m && z<=n{
        println!("{}x{}={}",x,z,c);
        c += x;
        z += f;
    }
}
