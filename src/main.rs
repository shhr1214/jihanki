struct VendingMachine {
    stocks: Vec<Stock>,
}

struct Stock {
    drink_name: String,
    price: usize,
    drinks: Vec<Drink>,
    num: usize,
}

struct Drink {
    name: String,
}

fn main() {
    println!("Hello, world!");
}
