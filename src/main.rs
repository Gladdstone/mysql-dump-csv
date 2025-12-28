use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use csv::Writer;
use regex::Regex;

fn parse_header(line: &mut Vec<&str>, writer: &mut csv::Writer<File>) {
    if line.len() < 5 {
        return;
    }
    // get values between parentheses
    let header_end = line.len() - 1;
    let header_values = &mut line[3..header_end];
    for value in header_values.iter_mut() {
        *value = clean_record(value).unwrap();
    }
    let _ = writer.write_record(header_values);
    return;
}

fn parse_values(line: &mut Vec<&str>, writer: &mut Writer<File>) {
    for value in line.iter_mut() {
        *value = clean_record(value).unwrap();
    }

    let _ = writer.write_record(&mut *line);
}

fn clean_record(value: &str) -> Result<&str, regex::Error> {
    let re = Regex::new(r#"^[(']*([^',()]+)[',);]*$"#)?;

    let Some(capture) = re.captures(value) else {
        return Ok("");
    };

    let Some(clean_cap) = capture.get(1) else {
        return Ok("");
    };
    Ok(clean_cap.as_str())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut file_path = "dump.sql";
    if args.len() > 1 {
        file_path = &args[1];
    }

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Unable to open file {}: {}", file_path, error);
            return Err("Unable to open file".into());
        }
    };

    let writer = &mut Writer::from_path("dump.csv")?;
    let reader = BufReader::new(file);

    let mut insert_statement = false;
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let mut line_split: Vec<&str> = line.split(" ").collect();

        if line_split.len() > 1 {
            if line_split[0].to_owned()+line_split[1] == "INSERTINTO" {
                parse_header(&mut line_split, writer);
                insert_statement = true;
            } else if insert_statement {
                if line_split.last().expect("").chars().last().expect("") == ';' {
                    parse_values(&mut line_split, writer);
                    break;
                }
                parse_values(&mut line_split, writer);
            }
        }
    }

    let _ = writer.flush();

    return Ok(())
}
