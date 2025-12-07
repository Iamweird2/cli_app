// use std::io;std::io::{self, Write};
use std::{
    collections::HashMap,
    // io,
    io::{self, Write},
};

// struct Contacts {
//     name: String,
//     number: String,
// }

// enum Commands {
//     Add,
//     List,
//     Remove,
//     Search,
//     Exit,
// }

fn main() {
    let mut logbook: HashMap<String, String> = HashMap::new();

    loop {
        //list users action
        println!(
            "Press
        add: to add contacts
        list: to list contacts
        remove: to remove contacts
        search: to search a contact
        exit: to exit
        "
        );
        print!("Enter a command: ");
        io::stdout().flush().unwrap();

        let mut command_input: String = String::new();
        io::stdin()
            .read_line(&mut command_input)
            .expect("Failed to read line");

        match command_input.trim() {
            "add" => {
                add_contact(&mut logbook);
            }
            "list" => {
                list_contact(&mut logbook);
            }
            "remove" => {
                remove_contact(&mut logbook);
            }
            "search" => {
                search_contact(&mut logbook);
            }
            "exit" => {
                break;
            }
            _ => {
                println!("invalid input")
            }
        }

        //define command functions
        fn add_contact(logbook: &mut HashMap<String, String>) {
            print!("Enter name: ");
            io::stdout().flush().unwrap();
            let mut name: String = String::new();

            io::stdin()
                .read_line(&mut name)
                .expect("Failed to read line");

            print!("Enter number: ");
            io::stdout().flush().unwrap();
            let mut number: String = String::new();

            io::stdin()
                .read_line(&mut number)
                .expect("Failed to read line");

            logbook.insert(name, number);
        }

        fn search_contact(logbook: &mut HashMap<String, String>) {
            let mut search_item: String = String::new();

            print!("What name in the logbook are you looking for");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut search_item)
                .expect("enter a valid name");

            match logbook.get(&search_item) {
                Some(number) => {
                    println!(
                        ">>{:?}'s number is {:?}",
                        search_item.trim_end(),
                        number.trim_end()
                    )
                }
                None => {
                    println!(">>{} doesn't exist in your contacts", search_item)
                }
            };
        }
        fn list_contact(logbook: &HashMap<String, String>) {
            println!("----------------------------------------");
            println!("{:<15} | {}", "NAME", "NUMBER");
            println!("----------------------------------------");

            for (name, number) in logbook {
                println!("{:<15} | {:?}", name.trim_end(), number.trim_end());
            }

            println!("----------------------------------------");
        }

        fn remove_contact(logbook: &mut HashMap<String, String>) {
            println!("Enter name to be removed");
            let mut name_to_be_removed: String = String::new();

            io::stdin()
                .read_line(&mut name_to_be_removed)
                .expect("Failed to read line");

            logbook.remove(&name_to_be_removed);
        }
        // fn exit_contact() {}
    }
}
