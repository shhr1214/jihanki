use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct VendingMachine {
    stocks: HashMap<String, Stock>,
}

impl VendingMachine {
    fn sell(&mut self, drink_name: String, money: usize) -> Result<Drink, String> {
        let stock = match self.stocks.get_mut(&drink_name) {
            Some(stock) => stock,
            None => return Err("そんな銘柄ない".into()),
        };

        if money < stock.price {
            return Err("金額がたりない".into());
        }

        let drink = stock.get()?;
        Ok(drink)
    }
}

#[derive(Debug, PartialEq)]
struct VendingMachineBuilder {
    stocks: HashMap<String, Stock>,
}

impl VendingMachineBuilder {
    fn new() -> Self {
        Self {
            stocks: HashMap::new(),
        }
    }

    fn add(mut self, drink: Drink, price: usize, num: usize) -> Self {
        self.stocks.insert(
            drink.name.clone(),
            Stock {
                drink_name: drink.name,
                price: price,
                num: num,
            },
        );
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

impl Stock {
    fn get(&mut self) -> Result<Drink, String> {
        if self.num <= 0 {
            return Err("在庫ないって".into());
        }

        self.num -= 1;

        Ok(Drink::new(self.drink_name.clone()))
    }
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

        let mut stocks = HashMap::new();
        stocks.insert(
            "コーラ".into(),
            Stock {
                drink_name: "コーラ".into(),
                price: 120,
                num: 10,
            },
        );
        stocks.insert(
            "カルピス".into(),
            Stock {
                drink_name: "カルピス".into(),
                price: 120,
                num: 10,
            },
        );
        stocks.insert(
            "コーヒー".into(),
            Stock {
                drink_name: "コーヒー".into(),
                price: 120,
                num: 10,
            },
        );

        assert_eq!(vending_machine, VendingMachine { stocks: stocks });
    }
}
