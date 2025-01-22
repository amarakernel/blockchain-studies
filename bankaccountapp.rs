trait Account {
  fn deposit(&mut self, amount: f64);
  fn withdraw(&mut self, amount: f64);
  fn balance(&self) -> f64;
}

struct BankAccount {
  account_number: u32,
  holder_name: String,
  balance: f64,
}

impl Account for BankAccount {
  fn deposit(&mut self, amount: f64) {
      self.balance += amount;
      println!("Deposited {:.2} into account {}", amount, self.account_number);
  }
  
  fn withdraw(&mut self, amount: f64) {
      if self.balance >= amount {
          self.balance -= amount;
          println!("Withdrew {:.2} from account {}", amount, self.account_number);
      } else {
          println!("Insufficient funds in account {}", self.account_number);
      }
  }
  
  fn balance(&self) -> f64 {
      self.balance
  }
}

fn main() {
  let mut account1 = BankAccount {
      account_number: 101,
      holder_name: "Alice".to_string(),
      balance: 1000.0,
  };
  
  let mut account2 = BankAccount {
      account_number: 102,
      holder_name: "Bob".to_string(),
      balance: 500.0,
  };
  
  account1.deposit(200.0);
  account2.withdraw(100.0);
  
  println!("Account {} balance: {:.2}", account1.account_number, account1.balance());
  println!("Account {} balance: {:.2}", account2.account_number, account2.balance());
}
