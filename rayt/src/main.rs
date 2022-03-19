mod rayt;

mod sun;
mod notsun;

pub use std::fs::File;
pub use std::io::prelude::*;
pub use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    contents.retain(|c| c != ' ');
    let isSun =  contents.len() != contents.find("sun()").unwrap_or(contents.len());
    if isSun{
        run(2,contents);
    }else{
        run(1,contents);
    }
}

fn run(no: i32,contents:String) {
    match no {
        1 => sun::run(contents),
        2 => notsun::run(contents),
        _ => {}
    }
}