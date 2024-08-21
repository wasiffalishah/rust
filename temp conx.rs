use std::io;

fn main() {
    let mut knowconversion: String = String::new(); 
    println!("What conversion do you want:\nPress 1 for Celsius to Fahrenheit\nPress 2 for Fahrenheit to Celsius");
    io::stdin().read_line(&mut knowconversion).expect("Error in getting the conversion.");
    let knowcon: i32 = knowconversion.trim().parse().expect("Error in converting string to i32");

    if knowcon == 1 {
        let mut tempvalue: String = String::new(); 
        println!("Enter temperature in Celsius:");
        io::stdin().read_line(&mut tempvalue).expect("Error in getting the temperature.");
        let tempvalcel: f64 = tempvalue.trim().parse().expect("Error in converting string to f64");
        conctof(tempvalcel);
    } else if knowcon == 2 {
        let mut tempvalue: String = String::new(); 
        println!("Enter temperature in Fahrenheit:");
        io::stdin().read_line(&mut tempvalue).expect("Error in getting the temperature.");
        let tempvalfah: f64 = tempvalue.trim().parse().expect("Error in converting string to f64");
        conftoc(tempvalfah);
    } else {
        println!("Invalid option selected.");
    }
}

fn conctof(celsius: f64) {
    let resultingtempinf = (celsius * 9.0 / 5.0) + 32.0;
    println!("{:.2}°C = {:.2}°F", celsius, resultingtempinf);
}

fn conftoc(fahrenheit: f64) {
    let resultingtempinc = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{:.2}°F = {:.2}°C", fahrenheit, resultingtempinc);
}
