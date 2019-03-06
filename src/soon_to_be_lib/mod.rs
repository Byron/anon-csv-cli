/// Can one day be a separate library
///
pub mod spec;

mod anon {
    use crate::soon_to_be_lib::spec::Spec;
    use csv::ReaderBuilder;
    use csv::WriterBuilder;
    use std::io::Read;
    use std::io::Write;

    #[derive(Debug, Default)]
    pub struct RewriteInfo {
        pub rows: u64,
        pub cells: u64,
    }

    pub fn anonymise(
        input: impl Read,
        delimiter: u8,
        specs: &[Spec],
        output: impl Write,
    ) -> Result<RewriteInfo, failure::Error> {
        let mut csv = ReaderBuilder::new().delimiter(delimiter).from_reader(input);
        let mut out_csv = WriterBuilder::new()
            .delimiter(delimiter)
            .from_writer(output);
        let mut info = RewriteInfo::default();
        for record in csv.records() {
            let record = record?;
            info.rows += 1;
            for spec in specs {
                info.cells += 1;
                let cell = record.get(spec.column).ok_or_else(|| {
                    format_err!(
                        "Invalid column index {} - rows have no more than {} columns",
                        spec.column,
                        record.len()
                    )
                })?;
            }
            out_csv.write_record(&record)?;
        }
        Ok(info)
    }
}

pub use self::anon::anonymise;
