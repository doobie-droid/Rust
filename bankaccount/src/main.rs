struct Account {
    id: u32,
    holder: String,
    amount: u64,
}

impl Account {
    fn new(id: u32, holder: String) -> Account {
        return Account {
            id,
            holder,
            amount: 0,
        };
    }
    fn deposit(&mut self, amount: u64) -> u64 {
        if amount > 0 {
            self.amount += amount;
        }
        self.amount
    }
    fn withdraw(&mut self, amount: u64) -> u64 {
        if self.amount > amount {
            self.amount -= amount;
        }
        return self.amount;
    }
    fn summary(&self) {
        println!("We got here");
        println!(
            "Bank with account ID: {} and with a holder of {} has a balance of {}",
            self.id, self.holder, self.amount
        )
    }
}
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Bank {
        return Bank { accounts: vec![] };
    }
    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
    fn get_total_account_balance(&self) -> u64 {
        let total_balance: u64 = self.accounts.iter().map(|account| account.amount).sum();
        println!("The total account balance is {}", total_balance);
        return total_balance;
    }
}

fn main() {
    let mut account: Account = Account::new(1, String::from("Leslie Douglas"));
    let mut account2: Account = Account::new(2, String::from("Nathan Godswill"));
    let mut bank: Bank = Bank::new();

    account.deposit(200);
    account.withdraw(50);
    account.summary();

    account2.deposit(300);
    account2.withdraw(10);
    account2.summary();

    bank.add_account(account);
    bank.add_account(account2);
    bank.get_total_account_balance();

}
