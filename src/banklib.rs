extern crate reqwest as req;

pub fn get_balance() -> Result<String, reqwest::Error>{
    let balance = String::from("Balance:");
    let foobar = req::get("https://httpbin.org/ip")?.text();
    foobar
}

pub fn get_transactions(number: &str) -> String{
    let transactions = format!("Last {} transactions", number);
    transactions
}