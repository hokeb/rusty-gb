use std::fs::File;
use std::io::Read;
use rusty_gb::gameboy::GameBoy;


fn main() {
    let matches = clap::Command::new("rusty-gb")
        .version("0.1")
        .author("hokeb")
        .about("A GameBoy Emulator")
        .arg(clap::Arg::new("filepath")
            .long("filepath")
            .short('f')
            .help("Sets the cartridge by accessing the file at specified path")
            .takes_value(true)
            .required(true))
        .get_matches();

    let filename = matches.value_of("filepath").unwrap();
    let mut gb = GameBoy::new(std::path::PathBuf::from(filename));
    gb.run();
}
