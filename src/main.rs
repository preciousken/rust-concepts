// fn main() {
//     let mut account: BankAccount = BankAccount{
//         owner:"Precious".to_string(),
//         balance:1000.00
//     };

//     // immutable borrow to check the balance
//     account.check_balance();
//     account.withdraw(50.75);
//     account.check_balance();
// }

// struct BankAccount {
//     owner: String,
//     balance:f64,
// }

// impl BankAccount {
//     fn withdraw(&mut self, amount: f64) {
//         println!(
//             "withdrawing {} from the account owned by {}",
//             amount, self.owner
//         );
//         self.balance -= amount;
//     }

//     fn check_balance(&self) {
//         println!("Account balance of {} is {}", self.owner, self.balance);
//     }
// }

// fn main(){
//     const age:i32 = 40;
//     if (age >= 18){
//         println!("you can drive a car", );

//     }else {
//         println!("You can't drive a car", );
//     }
// }

// fn main() {
//     struct Book {
//         title: String,
//         author: String,
//         pages: u32,
//         available: bool,
//     }

//     struct User {
//         active: bool,
//         username: String,
//         email: String,
//         sign_in_count: u64,
//     }

//     let mut   user1: User = User {
//         active: true,
//         username: String::from("ademola Kehinde"),
//         email:String::from("email@mail.com"),
//         sign_in_count: 1 ,
//     };

//     user1.email = String::from("newmail@mail.com");
//     println!("user email is {}",user1.email );
// }

// using the enum
// fn main() {
//     enum IpAddr {
//         V4(String),
//         V6(String),
//     }

//     let home:IpAddr = IpAddr::V4(String::from("127.0.0.1"));
// }


// error handling techniques
// fn main() {
//     #[derive(Debug)]

//     // approach 1
//     enum Option<T> {
//         Some(T),
//         None,
//     }

//     // approach 2
//     enum Result<T, E> {
//         Ok(T),
//         Err(E),
//     }
// }



