/// Can one day be a separate library
///
pub mod spec;

mod anon {
    use crate::soon_to_be_lib::spec::Spec;
    use csv::ReaderBuilder;
    use csv::StringRecord;
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
        let mut csv = ReaderBuilder::new()
            .has_headers(false)
            .delimiter(delimiter)
            .from_reader(input);
        let mut out_csv = WriterBuilder::new()
            .delimiter(delimiter)
            .from_writer(output);
        let mut info = RewriteInfo::default();
        for record in csv.records() {
            let record = record?;
            info.rows += 1;
            let mut anon_record =
                StringRecord::with_capacity(record.as_slice().as_bytes().len(), record.len());
            let mut last_cell: Option<usize> = None;
            let push_fields = |target: &mut StringRecord, from: Option<usize>, to| {
                for index in (from.map(|index| index + 1).unwrap_or(0))..to {
                    target.push_field(&record[index])
                }
            };
            for spec in specs {
                info.cells += 1;
                let cell = record.get(spec.column).ok_or_else(|| {
                    format_err!(
                        "Invalid column index {} - rows have no more than {} columns",
                        spec.column,
                        record.len()
                    )
                })?;
                push_fields(&mut anon_record, last_cell, spec.column);
                anon_record.push_field(&spec.kind.fake());
                last_cell = Some(spec.column);
            }
            push_fields(&mut anon_record, last_cell, record.len());
            out_csv.write_record(&anon_record)?;
        }
        Ok(info)
    }
}

pub use self::anon::anonymise;
