use clap::Parser;
use anyhow::{Context, Result};
use grrs::find_matches;

#[derive(Parser)]
struct Cli {
    /// The Pattern to search for
    pattern: String,
    /// Path to the file we want to search
    path: std::path::PathBuf,
}

fn main() -> Result<()>{
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    let args = Cli::parse();

    // println!("pattern: {:?}, path {:?}", args.pattern, args.path)

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())

}
