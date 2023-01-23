use clap::Parser;
use rand::{thread_rng, Rng, distributions::Alphanumeric};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    gen_key: bool
}

fn main() {
    let args = Args::parse();

    if args.gen_key {
            let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();
        println!("{}", rand_string);
    }
}
