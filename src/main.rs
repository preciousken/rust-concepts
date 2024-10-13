fn main() {
    let account: BankAccount = BankAccount{
        owner:"Precious".to_string(),
        balance:1000.00
    };

    // immutable borrow to check the balance
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!(
            "withdrawing {} from the account owned by {}",
            amount, self.owner
        );
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account balance of {} is {}", self.owner, self.balance);
    }
}
