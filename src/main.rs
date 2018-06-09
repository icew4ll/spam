// imports {{{
#![allow(unused_must_use)]
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate quicli;
extern crate duct;
// #[macro_use]
// extern crate prettytable;
#[macro_use]
extern crate lazy_static;
extern crate regex;
// }}}
// types {{{
// use std::str;
use duct::cmd;
use quicli::prelude::*;
use regex::Regex;

lazy_static! {
    pub static ref REG: Regex = Regex::new(r#" (\d{1,}@)"#).unwrap();
}
// }}}
// structs {{{
/// IceSync
#[derive(Debug, StructOpt)]
struct Cli {
    /// gitvim, gitdot, gitrgit, pulldot, gitall
    command: String,
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: u8,
}

// }}}
// main {{{
main!(
    |args: Cli, log_level: verbosity| match args.command.as_ref() {
        "ao5" => ssh(
            dotenv!("U3").to_string(),
            dotenv!("P2").to_string(),
            r"216.230.254.45".to_string(),
        ),
        // "test" => table(),
        _ => println!("No such command"),
    }
);

fn ssh(user: String, pass: String, ip: String) {
    let command = format!("spawn ssh {}@{};expect \"password\";send \"{}\n\";expect \"root\";send \"mailq | grep \\$(date | awk \\'{{print \\$2}}\\') | awk \\'{{print \\$7}}\\' | sort | uniq -c | sort -n\\n\";expect \"root\";send \"exit\\n\";expect eof;exit", user, ip, pass);
    println!("{}", command);
    let args = &["-c", command.as_str()];
    let stdout = cmd("expect", args).read().unwrap();
    println!("{}", stdout);
    // Display bytes
    // println!("{:?}", stdout);
}

// fn table() {
//     let table = ptable!(
//         ["ABC", "DEFG", "HIJKLMN"],
//         ["foobar", "bar", "foo"],
//         ["foobar2", "bar2", "foo2"]
//     );
// }
// }}}
