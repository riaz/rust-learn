use clap::Parser;
use anyhow::{Context, Result};
use indicatif::ProgressBar;
use log::{info};

#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box <dyn std::error::Error>> {
    env_logger::init();
    /*
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };*/

    let pb = ProgressBar::new(100);

    info!("Starting Up");

    let args =  Cli::parse();

    //let f = File::open(&args.path)
    //let mut reader = BufReader::new(f)

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    info!("Looking for matches");

    grep_rust::find_matches(&content, &args.pattern, &mut std::io::stdout());
    /*
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }*/

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    pb.finish_with_message("done!");

    Ok(())
}
