extern crate hypercore;

use hypercore::Feed;
use std::{path::PathBuf, str};

fn main() {
    let path = PathBuf::from("./cultures");
    let mut feed = Feed::open(&path).unwrap();

    feed.append(b"Hericium").unwrap();
    feed.append(b"erinaceus").unwrap();

    let genus = feed.get(0).unwrap().unwrap();
    let species = feed.get(1).unwrap().unwrap();

    println!(
        "{} {}",
        str::from_utf8(&genus).unwrap(),
        str::from_utf8(&species).unwrap()
    );
}
