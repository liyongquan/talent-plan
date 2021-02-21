use clap::{App, AppSettings, Arg, SubCommand};
use std::process::exit;
use kvs::KvStore;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("VALUE")
                        .help("The string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .get_matches();

    let mut kv_store = KvStore::new();
    match matches.subcommand() {
        ("set", Some(_matches)) => {
            let key = _matches.value_of("KEY").unwrap();
            let value = _matches.value_of("VALUE").unwrap();
            kv_store.set(String::from(key), String::from(value));
            println!("set successful...key:{},value:{}", key, value);
            exit(0);
        }
        ("get", Some(_matches)) => {
            let key = _matches.value_of("KEY").unwrap();
            let value = kv_store.get(String::from(key));
            let res = match value {
                Some(v) => {
                    v
                }
                None => {
                    String::from("none")
                }
            };
            println!("get successful...value:{}", res);
            exit(0);
        }
        ("rm", Some(_matches)) => {
            let key = _matches.value_of("KEY").unwrap();
            kv_store.remove(String::from(key));
            println!("remove successful...key:{}", key);
            exit(0);
        }
        _ => unreachable!(),
    }
}
