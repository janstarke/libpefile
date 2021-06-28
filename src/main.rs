#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
mod pefile;

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
mod winnt;
mod utils;

use argparse::{ArgumentParser, Store};
use std::path::PathBuf;
use pefile::*;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Debug).init().unwrap();
    let mut pefile = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("analyses a PE file");
        ap.refer(&mut pefile).add_argument("pefile", Store, "name of the PE file").required();
        ap.parse_args_or_exit();
    }
    let pefile = match PEFile::new(PathBuf::from(pefile)) {
        Ok(file)    =>  file,
        Err(why)    =>  {log::error!("{}", why); std::process::exit(1); }
    };
    pefile.print_resources();
}
