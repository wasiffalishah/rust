#[derive(Debug)] // Add the Debug trait
struct Coin {
    name: String,
    ticker: String,
    total_Supply: u128,
    market_Cap: u128,
    initial_Price: u32,
    blockchain: String,
    is_live: bool,
}

fn main() {
    let coin1 = Coin {
        name: String::from("MOBILE NIKAL BSDK"),
        ticker: String::from("BSDK"),
        total_Supply: 1200000,
        market_Cap: 4800000,
        initial_Price: 4,
        blockchain: String::from("SOLANA"),
        is_live: true,
    };
    
    coin1.print_details();
}

impl Coin {
    fn print_details(&self) {
        println!("{:?}", self); // Implementation of struct to add function to it
    }
}
