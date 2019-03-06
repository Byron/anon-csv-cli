#!/usr/bin/env bash
set -eu

exe=${1:?First argument must be the executable to test}

root="$(cd "${0%/*}" && pwd)"
# shellcheck disable=1090
source "$root/utilities.sh"
snapshot="$root/snapshots"
fixtures="$root/fixtures"

SUCCESSFULLY=0
WITH_FAILURE=1

function tabular() {
  xsv table
}

(with "not enough arguments"
  it "fails with an error message" && {
    WITH_SNAPSHOT="$snapshot/failure-missing-arguments" \
    expect_run ${WITH_FAILURE} "$exe"
  }
)
(with "the --all-specs argument"
  it "succeeds and prints all available faker specifications" && {
    WITH_SNAPSHOT="$snapshot/success-all-specs" \
    expect_run ${SUCCESSFULLY} "$exe" --all-specs foo
  }
)
(with "a valid input from stdin"
  (with "no column"
    it "succeeds" && {
      WITH_SNAPSHOT="$snapshot/success-noop-from-stdin" \
      expect_run ${SUCCESSFULLY} "$exe" -q --header - < "$fixtures/addresses-privatized-with-header.csv"
    }
  )
)
(with "a valid input"
  (with "header"
    (with "all columns"
      it "succeeds and removes all private data with plausible fake data, but keeps the header" && {
        WITH_SNAPSHOT="$snapshot/success-no-private-data-with-header" \
        expect_run_sh ${SUCCESSFULLY} "$exe -q --header $fixtures/addresses-privatized-with-header.csv 0:name.first_name 1:name.last_name 2:address.street_name 3:address.city 4:address.state_abbr 5:address.zip | grep private"
      }
    )
  )
  (with "no header"
    (with "and a duplicate spec"
      it "fails with a decent error message" && {
        WITH_SNAPSHOT="$snapshot/failure-duplicate-spec" \
        expect_run ${WITH_FAILURE} "$exe" "$fixtures/addresses.csv" 0:Internet.safe_email 0:Internet.safe_email
      }
    )
    (with "and a rewrite spec being out of range"
      it "fails with a decent error message" && {
        WITH_SNAPSHOT="$snapshot/failure-column-out-of-range" \
        expect_run ${WITH_FAILURE} "$exe" "$fixtures/addresses.csv" 8:Internet.safe_email
      }
    )
    (with "a rewrite spec being in range"
      (with "two non-consecutive columns"
        it "succeeds" && {
          expect_run ${SUCCESSFULLY} "$exe" -q "$fixtures/addresses.csv" 2:address.street_name 4:address.state_abbr
        }
      )
      (with "two consecutive columns"
        it "succeeds" && {
          expect_run ${SUCCESSFULLY} "$exe" -q "$fixtures/addresses.csv" 2:address.street_name 3:address.city
        }
      )
      (with "the center column"
        it "succeeds" && {
          expect_run ${SUCCESSFULLY} "$exe" -q "$fixtures/addresses.csv" 2:address.street_name
        }
      )
      (with "the first column"
        it "succeeds" && {
          expect_run ${SUCCESSFULLY} "$exe" -q "$fixtures/addresses.csv" 0:name.name
        }
      )
      (with "the last column"
        it "succeeds" && {
          expect_run ${SUCCESSFULLY} "$exe" -q "$fixtures/addresses.csv" 5:address.zip
        }
      )
      (with "all columns"
        it "succeeds and removes all private data with plausible fake data" && {
          WITH_SNAPSHOT="$snapshot/success-no-private-data" \
          expect_run_sh ${SUCCESSFULLY} "$exe -q $fixtures/addresses-privatized.csv 0:name.first_name 1:name.last_name 2:address.street_name 3:address.city 4:address.state_abbr 5:address.zip | grep private || :"
        }
      )
    )
  )
)
