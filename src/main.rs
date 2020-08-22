use home::home_dir;

mod store;
mod store_manager;

use clap::{App, AppSettings, Arg, SubCommand};
use std::path::PathBuf;
use store_manager::StoreManager;

// Test phase: list-stores as the default action.
fn main() {
    let app = App::new("CrypTRust")
        .version("0.1.0")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("add-store")
            .arg(Arg::with_name("name").required(true).takes_value(true))
            .arg(Arg::with_name("url").required(true).takes_value(true))
            .arg(Arg::with_name("initial-key")))
        .subcommand(
            SubCommand::with_name("list-stores")
            )
        .subcommand(
            SubCommand::with_name("select-store")
            .arg(Arg::with_name("name").required(true).takes_value(true))
            )
        .subcommand(
            SubCommand::with_name("show")
            )
        ;
    let matches = app.get_matches();

    let manager = StoreManager::new(home_dir().expect("User home not found!"));

    match matches.subcommand() {
        ("add-store",  Some(sub_matches)) => {
            let path = match sub_matches.value_of("initial-key") {
                Some(path_string) => Some(PathBuf::from(path_string)),
                None => None,
            };

            manager.add_store(
                sub_matches.value_of("name").expect("Failed getting name argument."),
                sub_matches.value_of("url").expect("Failed getting url argument."),
                // TODO: Implement use of initial-key.
                path.as_ref(),
            )
        },
        ("select-store",  Some(sub_matches)) => {
            let name = sub_matches.value_of("name").expect("Failed getting name argument.");
            manager.select_store(name)
        },
        ("list-stores",  _) => {
            manager.list_stores()
        },
        ("show",  _) => {
            manager.current_store().show()

        },
        ("edit",  Some(sub_matches)) => {},
        ("add-user",  Some(sub_matches)) => {},
        ("list-users",  Some(sub_matches)) => {},
        (any, Some(sub_matches)) => { println!("Unknown subcommand {} -> {:?}", any, sub_matches); },
        _ => {},
    }
}
