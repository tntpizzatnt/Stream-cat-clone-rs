// Thanks to everyone who provided ideas and help on stream
// Special thanks to silen_z (https://twitch.tv/silen_z) (https://github.com/silen-z) and Jdender (https://github.com/Jdender)

// Instead of returning Result use match statements

use std::fs::{File, metadata};
use std::io::{BufReader, BufRead, Error, ErrorKind};
use clap::{App, Arg};

fn main() -> Result<(), Error> {
    let matches = App::new("Read File") // Creates a new Clap App with the name "Read File"
        .version("1.0.0")
        .author("AUTHOR: https://twitch.tv/tntpizzatntea")
        .about("DESCRIPTION: A Simple Cat Clone With File Output and Line Numbers")
        .arg(Arg::with_name("path") // Specifies the first argument "path"
            .short("p")
            .long("path")
            .takes_value(true) // Takes a value 
            .help("Specifiy a path to file"))
        .arg(Arg::with_name("num")
            .short("n")
            .long("num")
            .takes_value(false)
            .help("Adds line numbers to the output"))
        .get_matches();
    
    let file = matches.value_of("path").unwrap(); // Take the value of path
    let metadata = metadata(file)?;

    if !metadata.is_file() { // if path doesn't lead to a file return error
        let fail = Error::new(ErrorKind::Other, "Path does not lead to a file");
        return Err(fail);
    }

    let input = File::open(file)?; // input = open file
    let buffered = BufReader::new(input); // Read each line of file
    let lines_enabled = matches.is_present("num"); // if -n or --num is passed lines enabled = true

    for (line_number, line) in buffered.lines().enumerate() { // line number and line start at 0 -> iterate through them
        let line = line?;
        if lines_enabled { // if lines are enabled print output with line number
            println!("{} | {}", line_number + 1, &line); // +1 bevause starts at 0
        } else {
            println!("{}", &line); // print each line of code
        }
    }

    Ok(()) // return Ok
}
