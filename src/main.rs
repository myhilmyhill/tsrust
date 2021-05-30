use std::env;
use std::fs::File;
use std::io::BufReader;
mod es;
mod rawbytes;
mod ts;
use crate::es::EsStocker;
use crate::es::EsStockerConfig;
use crate::ts::TsReader;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("Must input path");
    let file = File::open(path)?;
    let reader = TsReader::new(file);
    let mut stocker = EsStocker::new(
        |pid, v| {
            println!("{:?}", v);
        },
        EsStockerConfig {
            taking_pids: vec![276],
        },
    );
    for i in reader {
        if let Err(err) = stocker.set(&i) {};
    }
    Ok(())
}
