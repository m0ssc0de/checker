use std::error::Error;
use std::ops::Range;
use std::fs;
use std::path::Path;
use std::process;
extern crate structopt;
extern crate checker;

use chrono::Timelike;
use structopt::StructOpt;
use std::path::PathBuf;
use chrono::NaiveDateTime;
use std::collections::HashMap;

#[derive(StructOpt, Debug)]
#[structopt(name = "Partition")]
enum Partition {
    #[structopt(name = "index")]
    Index {
        #[structopt(short = "h",)]
        has_headers: bool,
        #[structopt(short = "b", default_value = "0")]
        block_number_column: usize,
        #[structopt(short = "t", default_value = "16")]
        timestamp_column: usize,
        #[structopt(short = "o", default_value = "./map.index")]
        output: PathBuf,
        #[structopt(name = "csv", parse(from_os_str))]
        csv_file: PathBuf,
    },
    #[structopt(name = "split")]
    Split {
        #[structopt(short = "h",)]
        has_headers: bool,
        #[structopt(short = "b", default_value = "0")]
        block_number_column: usize,
        #[structopt(short = "i")]
        index: PathBuf,
        #[structopt(name = "csv", parse(from_os_str))]
        csv_file: PathBuf,
    }
}

fn example() -> Result<(), Box<dyn Error>> {
    match Partition::from_args(){
        Partition::Split { has_headers ,block_number_column, index, csv_file } => {
            let mut rdr = csv::ReaderBuilder::new().has_headers(has_headers).from_path(csv_file)?;
            let index_str = fs::read_to_string(index).expect("can not read index file");
            let block2time :HashMap<String, Range<i64>>= serde_json::from_str(&index_str)?;
            for result in rdr.records() {
                let record = result?;
                let block_num = record.get(block_number_column).unwrap().parse::<i64>()?;
                let mut parent =String::from("");
                let mut range_start =0;
                let mut range_end = 0;
                for (p, r) in &block2time {
                    if r.contains(&block_num) {
                        parent = p.clone();
                        range_start = r.start;
                        range_end = r.end-1;
                        break;
                    }
                }
                fs::create_dir_all(&parent)?;
                let filename = format!("{:010}-{:010}.csv", range_start, range_end);
                let u = Path::new(&parent).join(filename);
                csv::Writer::from_path(u)?.write_record(&record)?;
            }
        }
        Partition::Index {has_headers, block_number_column, timestamp_column, output, csv_file } => {
            let mut rdr = csv::ReaderBuilder::new().has_headers(has_headers).from_path(csv_file)?;
            let mut block2time :HashMap<String, Range<i64>>= HashMap::new();
            for result in rdr.records() {
                let record = result?;
                let b = record.get(block_number_column).unwrap().parse::<i64>()?;
                let dit = record.get(timestamp_column).unwrap().parse::<i64>()?;
                let time = NaiveDateTime::from_timestamp_millis(dit*1000).unwrap();
                let p = format!("datatime={:?}/hour={:02}",time.date(), time.hour());
                match block2time.get_mut(&p) {
                    Some(mut block_range)=> {
                        if b < block_range.start {
                            block_range.start = b
                        }
                        if b > block_range.end {
                            block_range.end = b+1
                        }
                    }
                    None=> {
                        block2time.insert(p.clone(), Range{start: b, end: b+1});
                        ()
                    },
                }
            }
            fs::write(output, serde_json::to_string(&block2time)?).expect("Unable to write file");
        }
    };
    // Build the CSV reader and iterate over each record.
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}