# myka

A culture library & cultivation log for tracking the expansion of mycelia. 

Rust CLI tool.

_[ WIP ]._

### Installation

Note: Requires Rust compiler (install with [rustup](https://rustup.rs)).

`git clone https://github.com/mycognosist/myka`  
`cd myka`  
`cargo build --release`  

### Usage

`./target/release/myka`

```
myka 0.1.0
glyph | mycognosist <gnomad@cryptolab.net>

USAGE:
    myka <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add     Add culture to Library
    help    Prints this message or the help of the given subcommand(s)
    list    List cultures in Library
```

### Vision

`myka` has begun life as a local storage and query tool for interacting with a library of fungal cultures. It will grow to allow the storage and querying of culture lines, with each line forming an append-only log of cultivation events (e.g. agar -> grain -> woodchips).

Additional (imagined) capabilities will include tracking yields and environmental conditions to allow for optimization based on generated statistics.

The eventual goal is to make `myka` truly mycelial by leveraging p2p sociotechnology...

**Hypercore + Hyperswarm**

Back-up your virtual library and cultivation logs with peers.
Share your virtual library with peers to facilitate trade. 
Share media and substrate recipes with peers.
