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
                stocks: vec![Stock {
                    drink_name: "コーラ".into(),
                    price: 120,
                    drinks: vec![Drink::new("コーラ")],
                    num: 10,
                }, Stock {
                    drink_name: "カルピス".into(),
                    price: 120,
                    drinks: vec![Drink::new("カルピス")],
                    num: 10,
                }, Stock {
                    drink_name: "コーヒー".into(),
                    price: 120,
                    drinks: vec![Drink::new("コーヒー")],
                    num: 10,
                }].
            }
        );
    }
}
