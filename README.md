A tool to anonymise CSV files, providing various options to substitute real data with plausable fake data.

## Installation

Using Cargo (which you can install with https://rustup.rs):

```
cargo install anon-csv-cli
```

## Usage

```
curl https://people.sc.fsu.edu/~jburkardt/data/csv/addresses.csv > addresses.csv
anon-csv ./addresses.csv  0:name.first_name 1:name.last_name 2:address.street_name 3:address.city 4:address.state_abbr 5:address.zip
```

Use the `--header` flag to ignore the first line, usually the header, and the `--delimiter <other delimiter>` flag
to use a different delimiter.

## Works well with XSV

XSV is a tool to perform various operations on CSV files. It can be used in conjunction with this one for additional
processing.

https://github.com/BurntSushi/xsv


## Available fake data types

```
Name.first_name
Name.last_name
Name.prefix
Name.suffix
Name.name
Name.name_with_middle
Name.title_descriptor
Name.title_level
Name.title_job
Name.title

Address.time_zone
Address.city_prefix
Address.city_suffix
Address.street_suffix
Address.state
Address.state_abbr
Address.city
Address.street_name
Address.building_number
Address.street_address
Address.secondary_address
Address.zip
Address.postcode
Address.latitude
Address.longitude

Company.suffix
Company.name
Company.buzzword
Company.catch_phrase
Company.bs
Company.profession
Company.industry

Lorem.word
Lorem.words
Lorem.sentence
Lorem.sentences
Lorem.paragraph
Lorem.paragraphs

Number.phone_number
Number.cell_number
Number.digit

Internet.free_email_provider
Internet.domain_suffix
Internet.user_name
Internet.free_email
Internet.safe_email
Internet.password
Internet.ip
Internet.ipv4
Internet.ipv6
Internet.color
Internet.user_agent

Boolean.simple
```
