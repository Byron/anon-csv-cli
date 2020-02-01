#[macro_use]
extern crate failure;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate strum_macros;

use failure::{Error, ResultExt};
use failure_tools::ok_or_exit;
use soon_to_be_lib::spec::FakerKind;
use std::{fs::File, io::stdin, io::stdout, io::Read, path::Path};
use structopt::StructOpt;

mod options;
mod soon_to_be_lib;

fn run() -> Result<(), Error> {
    let opt: options::Args = options::Args::from_args();
    if opt.print_specs {
        FakerKind::eprint_combinations();
        return Ok(());
    }
    let reader: Box<dyn Read> =
        if opt.csv_file == Path::new("-") {
            Box::new(stdin())
        } else {
            Box::new(File::open(&opt.csv_file).with_context(|_| {
                format!("Could not open '{}' for reading", opt.csv_file.display())
            })?)
        };
    let stdout = stdout();
    let stdout_lock = stdout.lock();
    let info = soon_to_be_lib::anonymise(
        reader,
        opt.delimiter as u32 as u8,
        opt.header,
        &opt.specs,
        stdout_lock,
    )
    .with_context(|_| format!("Anonymisation failed"))?;
    if !opt.quiet {
        eprintln!("{:?}", info);
    }
    Ok(())
}

fn main() {
    ok_or_exit(run())
}
