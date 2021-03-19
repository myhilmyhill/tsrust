use std::env;
use std::fs::File;
use std::io::BufReader;

mod ts;
use crate::ts::TsReader;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("Must input path");
    let file = File::open(path)?;
    let reader = TsReader::new(file);
    for i in reader {
        println!("{:#?}", i);
    }
    Ok(())
}
