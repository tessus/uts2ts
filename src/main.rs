use std::env;

fn help() {
    println!("usage: uts2ts SECONDS");
}

fn main () {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let num = &args[1];

            let number: i64 = match num.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    println!("error: argument not an integer");
                    help();
                    return;
                },
            };
            let ts = uts2ts::uts2ts(number);
            println!("{}", ts.as_string());
        },
        _ => {
            help();
        }
    }
}
