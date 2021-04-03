use argparse::{ArgumentParser, Store};
use std::path::PathBuf;

fn main() {
    let mut pefile = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("analyses a PE file");
        ap.refer(&mut pefile).add_argument("pefile", Store, "name of the PE file").required();
        ap.parse_args_or_exit();
    }
    let pefile = PathBuf::from(pefile);
}
