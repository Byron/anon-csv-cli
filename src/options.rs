use crate::soon_to_be_lib::spec::Spec;
use std::path::PathBuf;

fn parse_spec(src: &str) -> Result<Spec, failure::Error> {
    src.parse()
}

#[derive(Debug, StructOpt)]
#[structopt(name = "anon-csv", about = "A CSV-file anonymizer")]
pub struct Args {
    /// The delimiter of the input CSV file. The same delimiter will be used for output.
    #[structopt(short = "d", long = "delimiter", default_value = ",")]
    pub delimiter: char,
    /// The path to the CSV file to use as input
    #[structopt(parse(from_os_str))]
    pub csv_file: PathBuf,
    /// One or more rewrite specifications. They look like '<column>:<type>', where <column> is
    /// a zero-based column indexed, separated from the <type> being the type of data to fake.
    /// Valid types are Internet.safe_email
    #[structopt(parse(try_from_str = "parse_spec"))]
    pub specs: Vec<Spec>,
}
