#[macro_use]

extern crate clap;
extern crate banklib;

use clap::{Arg, App};
use banklib::get_transactions;
use banklib::get_balance;


fn main() {
    let matches = App::new("bankrs")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Check balance and transactions")
        .arg(Arg::with_name("balance")
            .short("b")
            .long("balance")
            .takes_value(false)
            .help("Check balance"))
        .arg(Arg::with_name("number of transactions")
            .short("t")
            .long("transactions")
            .help("Lists n recent transactions")
            .takes_value(true))
        .arg(Arg::with_name("bank"))
        .get_matches();

    if matches.is_present("balance"){
        println!("{}", get_balance());
    }

    if let Some(number_of_transactions) = matches.value_of("number of transactions") {
        println!("{}", get_transactions(number_of_transactions));
    }


    }