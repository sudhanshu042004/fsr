use std::{
    env, fs,
    io::{Error, ErrorKind},
    os::unix::fs::MetadataExt,
};

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
    println!("size {}", md.size());
    println!("permission {:?}", md.permissions());
    Ok(())
}
