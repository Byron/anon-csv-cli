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

(with "not enough arguments"
  it "fails with an error message" && {
    WITH_SNAPSHOT="$snapshot/failure-missing-arguments" \
    expect_run ${WITH_FAILURE} "$exe"
  }
)
(with "a valid input and a rewrite spec being out of range"
  it "fails with a decent error message" && {
    WITH_SNAPSHOT="$snapshot/failure-input-file-column-out-of-range" \
    expect_run ${WITH_FAILURE} "$exe" "$fixtures/addresses.csv" 8:Internet.safe_email
  }
)
