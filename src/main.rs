mod args;
mod crypto;
mod store;

use crate::args::{Args, Commands};
use clap::Parser;

use crate::store::{Entry, Store};

use std::io::{self, Write};

fn ask_password_basic() -> Option<String> {
    // Ask for password
    let password = ask_param_if_none(
        None,
        "Please enter a password:\n>>> ",
        "Could not read line from stdin",
    );
    if password.is_empty() {
        println!("Empty password. aborting.");
        None
    } else {
        Some(password)
    }
}

fn ask_param_if_none(param: Option<String>, prompt: &str, err_msg: &str) -> String {
    param.unwrap_or_else(|| {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut param = String::new();
        io::stdin().read_line(&mut param).expect(err_msg);
        param.trim().to_string()
    })
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Create { store_name } => {
            // TODO: Check if the store already exists, show message: Sure you wanna overwrite?

            // If the store name is not given - Ask user for a name
            let store_name = ask_param_if_none(
                store_name,
                "Please enter a name for your store:\n>>> ",
                "Could not get store name!",
            );

            // Create initial store
            println!("Creating store..");
            let store = Store {
                name: store_name.to_string(),
                entries: Vec::new(),
            };

            // Ask user for key
            let pwd = ask_password_basic().unwrap();

            store.save_store(&args.store_path, &pwd).unwrap();
            println!("Store created with name: {:?}!", store.name);
        }
        Commands::Open { entry_name } => {
            // Ask user for key
            let pwd = ask_password_basic().unwrap();

            let store = Store::load_store(&args.store_path, &pwd);
            println!("Store opened: {:?}!", store.name);

            if let Some(entry_name) = entry_name {
                if let Some(entry) = store.entries.iter().find(|e| e.name == entry_name) {
                    println!(
                        r#"
                >>> Entry:
                -> Username: {:?}
                -> Password: {:?}
                "#,
                        entry.username.as_deref().unwrap_or(""),
                        entry.password.as_deref().unwrap_or(""),
                    )
                } else {
                    println!("Entry {} not found!", entry_name);
                }
            } else {
                // List all entries
                println!("\n>>> Entries in store:");
                store
                    .entries
                    .iter()
                    .for_each(|e| println!("-> {:?}", e.name));
                println!("-----")
            }
        }
        Commands::Add { entry_name } => {
            // Ask user for key
            let pwd = ask_password_basic().unwrap();

            let mut store = Store::load_store(&args.store_path, &pwd);

            // If the entry name is not given - Ask user for a name
            let entry_name = ask_param_if_none(
                entry_name,
                "Please enter a name for your entry:\n>>> ",
                "Could not get entry name!",
            );

            // Check if exists
            if store.entries.iter().any(|entry| entry.name == entry_name) {
                println!("Entry named '{}' already exists!", entry_name);
                return;
            }

            // Add a new entry
            // Prompt for username and password
            let username = ask_param_if_none(
                None,
                "Please enter a Username:\n>>> ",
                "Could not read line from stdin",
            );

            let password = ask_param_if_none(
                None,
                "Please enter a Password:\n>>> ",
                "Could not read line from stdin",
            );

            store.entries.push(Entry {
                name: entry_name.to_string(),
                username: if username.is_empty() {
                    None
                } else {
                    Some(username.to_string())
                },
                password: if password.is_empty() {
                    None
                } else {
                    Some(password.to_string())
                },
            });

            store.save_store(&args.store_path, &pwd).unwrap();
            println!("Added an entry to store: {:?}!", store.name);
        }
        Commands::Remove { entry_name } => {
            // Ask user for key
            let pwd = ask_password_basic().unwrap();

            let mut store = Store::load_store(&args.store_path, &pwd);

            // If the entry name is not given - Ask user for a name
            let entry_name = ask_param_if_none(
                entry_name,
                "Please enter a name for your entry:\n>>> ",
                "Could not get entry name!",
            );

            // Check if not exists
            if let Some(entry_pos) = store
                .entries
                .iter()
                .position(|entry| entry.name == entry_name)
            {
                store.entries.remove(entry_pos);
            } else {
                println!("Entry named '{}' does not exists!", entry_name);
                return;
            }

            store.save_store(&args.store_path, &pwd).unwrap();
            println!(
                "Removed entry {:?} from store: {:?}!",
                entry_name, store.name
            );
        }
    }
}
