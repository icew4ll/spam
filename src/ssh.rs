// imports {{{
#![allow(unused_must_use)]
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate quicli;
#[macro_use]
extern crate duct;
#[macro_use]
extern crate prettytable;
extern crate ssh2;
#[macro_use] extern crate lazy_static;
extern crate regex;
// }}}
// types {{{
use quicli::prelude::*;
use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;
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
        "w0" => getmq(
            dotenv!("U3").to_string(),
            dotenv!("P1").to_string(),
            r"216.230.241.172".to_string(),
        ),
        "test" => table(),
        _ => println!("No such command"),
    }
);

fn getmq(user: String, pass: String, ip: String) {
    let ipformat = format!("{}:22", ip);
    // Connect to the local SSH server
    let tcp = TcpStream::connect(ipformat).unwrap();
    let mut sess = Session::new().unwrap();
    sess.handshake(&tcp).unwrap();
    sess.userauth_password(&user, &pass).unwrap();
    // run cmd
    let mut channel = sess.channel_session().unwrap();
    channel.exec("mailq | grep Jun | awk '{print $7}' | sort | uniq -c | sort -n").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    let out = s;
    channel.wait_close();
    println!("{}", out);
}

fn table() {
    let table = ptable!(
        ["ABC", "DEFG", "HIJKLMN"],
        ["foobar", "bar", "foo"],
        ["foobar2", "bar2", "foo2"]
    );
}
// }}}
