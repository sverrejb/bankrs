#[macro_use]

extern crate clap;
extern crate banklib;

use banklib::get_balance;
use banklib::get_transactions;
use clap::{App, Arg};

fn main() {
    let matches = App::new("bankrs")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Check balance and transactions")
        .arg(
            Arg::with_name("balance")
                .short("b")
                .long("balance")
                .takes_value(false)
                .help("Check balance"),
        )
        .arg(
            Arg::with_name("n")
                .short("t")
                .long("transactions")
                .help("Lists n recent transactions")
                .takes_value(true),
        )
        .arg(Arg::with_name("bank"))
        .get_matches();

    if matches.is_present("balance") {
        let balance = match get_balance() {
            Ok(v) => v,
            Err(e) => panic!("Unable to get balance: {:?}", e)
        };
        println!("{}", balance)
    }

    if let Some(number_of_transactions) = matches.value_of("number of transactions") {
        println!("{}", get_transactions(number_of_transactions));
    }
}
