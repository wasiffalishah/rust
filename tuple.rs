fn main(){
    let mytuple : (u8, i8, f32) = (2,-1,4.5);
    let name = ("wasif", "mehmood", "ali", 125);
    
    println!("Print single element from tuple: {}, {}, {}", mytuple.0, mytuple.2, mytuple.1);
    println!("Print the whole tuple: {:?}", mytuple);
    println!("My name tuple: {:?}", name);
    
    
    //Destructuring a tuple
    
    let (w,x,y,z) = name;
    
    println!("mY FIRST NAME IS {}, MY MIDDLE NAME IS {}, MY LAST NAME IS {}, MY NUM IS {}",w,x,y,z)
    
}
