use crate::soon_to_be_lib::spec::Spec;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "anon-csv", about = "A CSV-file anonymizer")]
pub struct Args {
    /// If set, print all available faker specifications
    #[structopt(long = "all-specs")]
    pub print_specs: bool,
    /// If set, no additional output will be produced on stderr
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,
    /// If set, the CSV file is expected to have one header line. It will be reproduced in the output
    #[structopt(long = "header")]
    pub header: bool,
    /// The delimiter of the input CSV file. The same delimiter will be used for output.
    #[structopt(short = "d", long = "delimiter", default_value = ",")]
    pub delimiter: char,
    /// The path to the CSV file to use as input. If '-', the file will be read from stdin.
    #[structopt(parse(from_os_str))]
    pub csv_file: PathBuf,
    /// One or more rewrite specifications. They look like '<column>:<type>', where <column> is
    /// a zero-based column indexed, separated from the <type> being the type of data to fake.
    /// Run this command with --all-specs and all required arguments to learn about all possible values.
    #[structopt(parse(try_from_str))]
    pub specs: Vec<Spec>,
}
