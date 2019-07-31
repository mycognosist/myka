#[macro_use]
extern crate clap;
extern crate hypercore;

use std::{path::PathBuf, str};

use clap::App;
use hypercore::Feed;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Culture {
    genus: String,
    species: String,
    strain: String,
    source: String,
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let path = PathBuf::from("./cultures.db");
    let mut feed = Feed::open(&path).unwrap();

    if let Some(matches) = matches.subcommand_matches("add") {
        let genus = matches.value_of("GENUS").unwrap();
        let species = matches.value_of("SPECIES").unwrap();
        let strain = matches.value_of("STRAIN").unwrap();
        let source = matches.value_of("SOURCE").unwrap();

        let culture = Culture {
            genus: genus.to_string(),
            species: species.to_string(),
            strain: strain.to_string(),
            source: source.to_string(),
        };

        let culture_json = serde_json::to_string(&culture).unwrap();
        let culture_bytes = culture_json.into_bytes();

        feed.append(&culture_bytes).unwrap();
    };

    let c = feed.get(0).unwrap().unwrap();
    let c_str = String::from_utf8(c);

    println!("{}", c_str.unwrap());

    /*
        if matches.is_present("list") {
            for i in 0..feed.len() {
                println!("{}: {:?}", i, feed.get(i).unwrap().unwrap());
            }
        };
    */
    /*let feed.get(0).unwrap().unwrap();
    let species = feed.get(1).unwrap().unwrap();

    println!(
        "{} {}",
        str::from_utf8(&genus).unwrap(),
        str::from_utf8(&species).unwrap()
    );*/
}
