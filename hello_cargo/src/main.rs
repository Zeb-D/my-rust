use std::env;

// cargo run -- increase 10
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    match args.len() {
        // no arguments passed
        1 => {
            println!("My name is 'match_args'. Try passing some arguments!");
        }
        // one argument passed
        2 => match args[1].parse() {
            Ok(42) => println!("This is the answer!"),
            _ => println!("This is not the answer."),
        },
        // one command and one argument passed
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            // parse the number
            let number: i32 = match num.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    help();
                    return;
                }
            };
            // parse the command
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    eprintln!("error: invalid command");
                    help();
                }
            }
        }
        // all the other cases
        4 => {
            println!("hahaha four")
        }

        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}

fn increase(number: i32) {
    println!("111-{}", number + 1);
}

fn decrease(number: i32) {
    println!("222={}", number - 1);
}

fn help() {
    println!(
        "usage:
    match_args <string>
        Check whether given string is the answer.
    match_args {{increase|decrease}} <integer>
        Increase or decrease given integer by one."
    );
}
