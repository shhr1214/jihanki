#[derive(Debug, PartialEq)]
struct VendingMachine {
    stocks: Vec<Stock>,
}

#[derive(Debug, PartialEq)]
struct VendingMachineBuilder {
    stocks: Vec<Stock>,
}

impl VendingMachineBuilder {
    fn new() -> Self {
        Self { stocks: Vec::new() }
    }

    fn add(mut self, drink: Drink, price: usize, num: usize) -> Self {
        self.stocks.push(Stock {
            drink_name: drink.name,
            price: price,
            num: num,
        });
        self
    }

    fn build(self) -> VendingMachine {
        VendingMachine {
            stocks: self.stocks.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Stock {
    drink_name: String,
    price: usize,
    num: usize,
}

#[derive(Debug, PartialEq)]
struct Drink {
    name: String,
}

impl Drink {
    fn new(name: String) -> Self {
        Self { name: name }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vending_machine_initialize_test() {
        let vending_machine = VendingMachineBuilder::new()
            .add(Drink::new("コーラ".into()), 120, 10)
            .add(Drink::new("カルピス".into()), 120, 10)
            .add(Drink::new("コーヒー".into()), 120, 10)
            .build();

        assert_eq!(
            vending_machine,
            VendingMachine {
                stocks: vec![
                    Stock {
                        drink_name: "コーラ".into(),
                        price: 120,
                        num: 10,
                    },
                    Stock {
                        drink_name: "カルピス".into(),
                        price: 120,
                        num: 10,
                    },
                    Stock {
                        drink_name: "コーヒー".into(),
                        price: 120,
                        num: 10,
                    },
                ]
            }
        );
    }
}
