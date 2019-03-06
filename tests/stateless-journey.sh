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
    expect_run ${WITH_FAILURE} "$exe" --all-specs foo
  }
)
(with "a valid input"
  (with "no header"
    (with "and a rewrite spec being out of range"
      it "fails with a decent error message" && {
        WITH_SNAPSHOT="$snapshot/failure-column-out-of-range" \
        expect_run ${WITH_FAILURE} "$exe" "$fixtures/addresses.csv" 8:Internet.safe_email
      }
    )
    (with "a rewrite spec being in range"
      (with "two non-consecutive columns"
        it "succeeds and rewrites the output" && {
          WITH_SNAPSHOT="$snapshot/success-column-in-range-two-non-consecutive-columns" \
          SNAPSHOT_FILTER=tabular \
          expect_run ${SUCCESSFULLY} "$exe" -q "$fixtures/addresses.csv" 2:address.street_name 4:address.state_abbr
        }
      )
      (with "two consecutive columns"
        it "succeeds and rewrites the output" && {
          WITH_SNAPSHOT="$snapshot/success-column-in-range-to-consecutive-columns" \
          SNAPSHOT_FILTER=tabular \
          expect_run ${SUCCESSFULLY} "$exe" -q "$fixtures/addresses.csv" 2:address.street_name 3:address.city
        }
      )
      (with "the center column"
        it "succeeds and rewrites the output" && {
          WITH_SNAPSHOT="$snapshot/success-column-in-range-center-column" \
          SNAPSHOT_FILTER=tabular \
          expect_run ${SUCCESSFULLY} "$exe" -q "$fixtures/addresses.csv" 2:address.street_name
        }
      )
      (with "the first column"
        it "succeeds and rewrites the output" && {
          WITH_SNAPSHOT="$snapshot/success-column-in-range-first-column" \
          SNAPSHOT_FILTER=tabular \
          expect_run ${SUCCESSFULLY} "$exe" -q "$fixtures/addresses.csv" 0:name.name
        }
      )
      (with "the last column"
        it "succeeds and rewrites the output" && {
          WITH_SNAPSHOT="$snapshot/success-column-in-range-last-column" \
          SNAPSHOT_FILTER=tabular \
          expect_run ${SUCCESSFULLY} "$exe" -q "$fixtures/addresses.csv" 5:address.zip
        }
      )
    )
  )
)
