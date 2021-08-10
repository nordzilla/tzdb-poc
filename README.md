# tzdb-poc
A proof of concept for looking up DST transitions from the IANA TZDB.

## Usage

**Show all DST transitions for an IANA time-zone ID**
```
cargo run -- <TZID>
```

**Show DST transitions for an IANA time-zone ID on a given year**
```
cargo run -- <TZID> <YEAR>
```
