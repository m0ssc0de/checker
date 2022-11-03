extern crate structopt;
extern crate checker;

use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "checker")]
pub struct Opt {
    /// If the data range is in filename
    #[structopt(short = "f")]
    filename: bool,

    /// Set the depth of the iteraton
    #[structopt(short = "d", default_value = "1")]
    depth: i32,

    /// Directory to start with
    #[structopt(name = "DIRECTORY", default_value = ".", parse(from_os_str))]
    directory: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let (a, b, c, d) = checker::gap_in_data(
        &opt.directory,
        opt.depth,
        opt.filename
    );
    println!("{:#?}, {:#?}, {:#?}, {:#?}", a, b, c, d);
}
