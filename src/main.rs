use std::{
    env::{self},
    fs,
};
mod reciever;
mod sender;

#[derive(Debug)]
enum Purpose {
    SEND,
    RECIEVE,
}

fn main() {
    let purpose: Purpose;
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Expected at least 3 args while we got {}", args.len());
        return;
    }
    if args[1].to_ascii_uppercase() == "SEND" {
        purpose = Purpose::SEND;
    } else if args[1].to_ascii_uppercase() == "RECIEVE" {
        purpose = Purpose::RECIEVE
    } else {
        println!(
            "Unexpected args!! Recieved {}, expecting SEND or RECIEVE",
            args[1]
        );
        return;
    }

    match purpose {
        Purpose::SEND => {
            let file_path = args.remove(2);
            let ip_addr = &args[2];
            if !fs::metadata(&file_path)
                .expect("file metadata expected")
                .is_file()
            {
                println!(
                    "Expected File!! Provided path is not belong to file {}",
                    file_path
                );
                return;
            }
            sender::sender(file_path);
        }
        Purpose::RECIEVE => {
            let ip_addr = &args[2];
            reciever::Reciever();
        }
    }
}
