pub fn get_balance() -> String{
    let balance = String::from("Balance:");
    balance
}

pub fn get_transactions(number: &str) -> String{
    let transactions = format!("Last {} transactions", number);
    transactions
}