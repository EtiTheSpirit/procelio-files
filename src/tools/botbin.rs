use crate::files::robot::robot;
use std::io::{Read, Write};

fn botbin_usage() {
    println!("statbin path/to/json.json [path/to/bin]");
    println!("  converts a json robot file (at given path) to the binary representation");
    println!("  if no path/to/bin is supplied, defaults to 'path/to/json.robot'");
    println!("  either way, the resultant file is suitable for being served");
}

pub fn tool(mut args: std::env::Args) {
    let arg = args.next().unwrap_or("--help".to_owned());
    if arg == "--help" || arg == "-h" {
        botbin_usage();
        return;
    }

    let source = std::path::Path::new(&arg);
    let dst = args.next();
    let destination = match dst {
        None => source.with_extension("robot"),
        Some(e) => std::path::PathBuf::from(e)
    };

    let file = std::fs::File::open(source);
    if let Err(e) = file {
        println!("Could not open {}: {}", source.display(), e);
        return;
    }
    let mut file = file.unwrap();
    let mut file_contents = std::vec::Vec::new();
    let nr = file.read_to_end(&mut file_contents);
    if let Err(e) = nr {
        println!("Could not read {}: {}", source.display(), e);
        return;
    }
    let botfile : robot::JsonRobot = serde_json::from_slice(&file_contents).unwrap();
    let binarybotfile : robot::Robot = botfile.into();
    let res = binarybotfile.compile();
    match res {
        Err(e) => { println!("Unable to compile robotfile: {}", e)},
        Ok(data) => {
            let ff = std::fs::File::create(destination);
            if ff.is_err() {
                println!("Unable to save robotfile: {}", ff.err().unwrap());
                return;
            }
            let written = ff.unwrap().write_all(&data);
            if let Err(e) = written {
                println!("Unable to save robotfile: {}", e);
                return;
            }
         }
    }
    println!("Robotfile successfully compiled");
}