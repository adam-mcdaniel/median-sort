use sorting::*;
use clap::Parser;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    /// The file to write the array to. If not provided, the array will be printed to stdout
    #[clap(value_parser)]
    file: Option<String>,
    /// The number of elements in the array
    #[clap(short, long, default_value = "10000000")]
    n: usize,
    /// The min value for the random array
    #[clap(short, long, default_value = "0")]
    min: i32,
    /// The max value for the random array
    #[clap(short, long, default_value = "1000")]
    max: i32,
    /// The separator to use when writing the array to a file
    #[clap(short, long, default_value = "\n")]
    separator: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let arr = random_array(args.n, args.min..args.max);
    let output = arr.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(&args.separator);
    match args.file {
        None => {
            println!("{output}");
        },
        Some(file) => {
            std::fs::write(file, output)?;
        }
    }
    Ok(())
}