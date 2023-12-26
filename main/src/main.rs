use anyhow::{bail, Result};
use clap::Parser;
use encryptor::password::generate_password;
#[derive(Parser, Debug)]
#[clap(version,author,about,long_about=None)]
struct Args {
    #[clap(short, long)]
    seed: String,

    #[clap(short, long, default_value = "16")]
    length: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    if args.seed.len() < 4 {
        bail!("seed {} length must >=4", &args.seed);
    }
    let (seed, length) = (args.seed, args.length);
    let passwd = generate_password(&seed[..], length);
    match passwd {
        Ok(val) => println!("{}", val),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}
