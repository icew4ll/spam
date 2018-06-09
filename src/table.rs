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
// }}}
// types {{{
use quicli::prelude::*;
// }}}
// structs {{{
/// IceSync
#[derive(Debug, StructOpt)]
struct Cli {
    // Add a CLI argument `--count`/-n` that defaults to 3, and has this help text:
    /// Who to ban on webdearc
    #[structopt(long = "web", short = "w")]
    name: String,
    /// Who to ban on webdearc
    #[structopt(long = "d", short = "d")]
    name2: String,
    // Quick and easy logging setup you get for free with quicli
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: u8,
}
main!(|args: Cli, log_level: verbosity| {
    println!("{}", &args.name);
    println!("{}", &args.name2);
    table();
});

fn table() {
    let table = ptable!(
        ["ABC", "DEFG", "HIJKLMN"],
        ["foobar", "bar", "foo"],
        ["foobar2", "bar2", "foo2"]
    );
}
// }}}
