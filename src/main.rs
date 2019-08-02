extern crate hypercore;

use hypercore::Feed;
use structopt::StructOpt;

/*
// Add a culture to the library
#[derive(StructOpt)]
struct Culture {
    #[structopt(short = "a", long = "add")]
    genus: String, // Ganoderma
    #[structopt(short = "a", long = "add")]
    species: String, // lucidum
    #[structopt(short = "a", long = "add")]
    strain: String, // Aleon1
    #[structopt(short = "a", long = "add")]
    source: String, // FungiPerfecti
    #[structopt(short = "a", long = "add")]
    id: String, // GLAL001
}*/

// Application
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "add")]
    /// Add culture to Library
    Add { genus: String, species: String, strain: String, source: String },

    #[structopt(name = "list")]
    /// List cultures in Library
    List,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    
    let mut feed = Feed::open("./cultures")?;

    match args.command {
        Command::Add { genus, species, strain, source } => {
            let c = format!("{} {} {} {}", &genus, &species, &strain, &source);
            feed.append(&c.into_bytes())?;
        }
        Command::List => {
            for i in 0..feed.len() {
                println!("{:?}", feed.get(i))
            }
        }
    }

    Ok(())
}
