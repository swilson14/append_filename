use std::env;
use std::process;
use std::fs;
use std::fs::{OpenOptions, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    if let Ok(lines) = read_lines(&config.filepath) {

        let filename = Path::new(&config.filepath).file_name().unwrap().to_str().unwrap();
        let extension = format!(".{}",Path::new(&config.filepath).extension().unwrap().to_str().unwrap());
        let output_file = &config.filepath.replace(&extension, &format!("_output{}",&extension));
        const FIELDNAME: &str = "filename";
        
        let mut first_iteration = true;
        for line in lines.map_while(Result::ok) {
            if first_iteration {
                let header = format!("{}{}{}\n",&line,&config.delimiter,FIELDNAME);
                match write_header(&header, &output_file) {
                    Ok(()) => println!("Wrote {} to file {}", header, output_file),
                    Err(e) => eprintln!("Failed to write to file {}", e),
                }
                first_iteration = false;
            }
            else {
                let output_line = format!("{}{}{}",line, &config.delimiter, &filename);
                match append_to_file(&output_line, &output_file) {
                    Ok(()) => println!("Wrote {} to file {}", output_line, output_file),
                    Err(e) => eprintln!("Failed to write to file {}", e),
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where  P:AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn append_to_file(line: &str, filepath: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filepath)?;

    if let Err(e) = writeln!(&mut file, "{}", line) {
        eprintln!("Couldn't write to file: {}", e)
    }

    Ok(())
}

fn write_header(header: &str, filepath: &str) -> io::Result<()> {
    fs::write(filepath, header)?;
    Ok(())
}

struct Config {
    filepath: String,
    delimiter: String,
}

impl Config{
    fn build (mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str> {
        args.next();   
        let filepath = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let delimiter = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a delimiter"),
        };

        let delimiter = delimiter;

        Ok(Config{ filepath, delimiter })
    }
}
