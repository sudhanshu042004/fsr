use std::{
    env, fs,
    io::{Error, ErrorKind},
    os::unix::fs::MetadataExt,
};

const BLOCK_SIZE: u64 = 4000;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Provide the path of the file to send!!");
        Error::new(ErrorKind::InvalidFilename, "no file provided");
    }
    let md = fs::metadata(&args[1])?;
    if !md.is_file() {
        println!("File didn't exists");
        Error::new(ErrorKind::NotFound, "File didn't exists");
    }
    // get the number of blocks
    let BLOCKS = (md.size() + BLOCK_SIZE - 1) / BLOCK_SIZE;
    println!("{}", BLOCKS);
    Ok(())
}
