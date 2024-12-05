use clap::{Arg, Command};

pub fn main() {
    let matches = Command::new("My Test Program")
        .version("0.1.0")
        .author("Shynnnnnn")
        .about("Teaches argument parsing")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .help("Your name"),
        )
        .get_matches();

    let name = matches.get_one::<String>("name").unwrap();
    println!("Hello {}", name);
}
