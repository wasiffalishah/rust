fn main(){
 println!("sum is {}",add_func(3,6));
 println!("subtraction is {}",sub_func(3,6));
 println!("multiplication is {}",mul_func(3,6));
 println!("division is {}",div_func(3,6));
}

pub fn add_func(num1:i8, num2:i8) -> i8{
    let x=num1+num2;
    x
}

pub fn sub_func(num1:i8, num2:i8) -> i8{
    let x=num1-num2;
    x
}

pub fn mul_func(num1:i8, num2:i8) -> i8{
    let x=num1*num2;
    x
}

pub fn div_func<N: Into<f64>>(num1: N, num2: N) -> f64 {
    let x = num1.into() / num2.into();
    x
}

// using rust generic to intake all data types flaot, int, uint
// they will be converted to float and then fucntion will be performed
