#[macro_use]

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("bankrs")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Check balance and transactions")
        .arg(Arg::with_name("balance")
            .short("b")
            .long("balance")
            .help("Check balance"))
        .arg(Arg::with_name("number of transactions")
            .short("t")
            .long("transactions")
            .help("Lists recent transactions")
            .takes_value(true))
        .arg(Arg::with_name("bank").help("Specify which bank to check"))
        .get_matches();


    }

    // more program logic goes here...