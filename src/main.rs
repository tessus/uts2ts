use std::env;

fn help() {
    println!("usage: uts2ts SECONDS [utc|u]");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 | 3 => {
            let num = &args[1];

            let number: i64 = match num.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("error: argument not an integer");
                    help();
                    return;
                }
            };
            let ts = uts2ts::uts2ts(number);
            if args.len() == 2 {
                println!("{}", ts.as_string());
            } else if args.len() == 3 && (&args[2] == "utc" || &args[2] == "u") {
                println!("{}", ts.as_string_utc());
            } else {
                help();
            }
        }
        _ => {
            help();
        }
    }
}
