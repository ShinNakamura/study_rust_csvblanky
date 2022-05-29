use std::io::{self};
use csv;

type MyResult = Result<(), Box<dyn std::error::Error>>;

pub fn run() -> MyResult {
    let flds = get_flds();
    let input = io::stdin();
    let input = io::BufReader::new(input.lock());
    let mut rdr = csv::Reader::from_reader(input);
    let out = io::stdout();
    let mut wtr = csv::Writer::from_writer(out.lock());
    let header = rdr.headers()?.clone();
    let mut blanky_or_stay: Vec<bool> = Vec::new();
    for column in header.iter() {
        let mut is_blanky = true;
        for fld in flds.iter() {
            if fld == column {
                is_blanky = false;
                break;
            }
        }
        blanky_or_stay.push(is_blanky);
    }
    wtr.write_record(&header)?;
    for result in rdr.records() {
        let record_org = result?;
        let mut record: Vec<String> = Vec::new();
        for (i, is_blanky) in blanky_or_stay.iter().enumerate() {
            if *is_blanky {
                record.push("".to_string());
            } else {
                record.push(record_org[i].to_string());
            }
        }
        wtr.write_record(&record)?;
    }
    Ok(())
}

fn get_flds() -> Vec<String> {
    let mut flds: Vec<String> = Vec::new();
    let args = std::env::args();
    if args.len() < 2 {
        return flds;
    }
    let mut is_cmd_name = true;
    for arg in args {
        if is_cmd_name {
            is_cmd_name = false;
            continue;
        }
        flds.push(arg.to_string());
    }
    flds
}
