use clap::{Parser, Subcommand, Args};
// Command Line Interface - Argument Parser

#[derive(Parser)]
#[command(author, version)]
#[command(about = "A Blazingly Fast 🦀 🚀 tool for Cosmic Date Tool", long_about = None)]
struct CosmicCli {
    #[command(subcommand)]
    /// test
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// Shows your name
    YourName(YourName),

    /// shows your age
    YourAge(YourAge),

    /// doubles the number you enter
    Double(Double),
}

#[derive(Args)]
struct YourName {
    name: Option<String>
}

#[derive(Args)]
struct YourAge {
    age: Option<String>,
}


#[derive(Args)]
struct Double {
    num: Option<f32>,
}

pub fn get_name(input: &String) -> String {
    return format!("Your name is {}", input);
}

pub fn get_age(input: &String) -> String {
    return format!("Your age is {}", input);
}

// if you don't put a reference then you have to deref it later.
pub fn double(num: &f32) -> f32 {
    return num * 2.0;
}

fn main() {
    let cosmicli = CosmicCli::parse();

    // for input not forced on different situations
    match &cosmicli.command {
        Some(Commands::YourName(name)) => {
            match name.name {
                Some(ref _name) => {
                    let boxed_name = &Box::new(_name);
                    println!("{}", get_name(boxed_name))
                },
                None => {
                    println!("Please input your name")
                }
            }
        },

        Some(Commands::YourAge(age)) => {
            match age.age {
                Some(ref _age) => {
                    let boxed_age = &Box::new(_age);
                    println!("{}", get_age(boxed_age))
                },
                None => {
                    println!("Please input your age")
                }
            }
        },

        Some(Commands::Double(num)) => {
            match num.num {
                Some(ref _num) => {
                    let boxed_num = &Box::new(_num);
                    println!("{} times 2 is {}", boxed_num, double(boxed_num))
                },
                None => {
                    println!("Please input your number")
                }
            }
        },

        None => {}
    }
}
