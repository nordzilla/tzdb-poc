# tzdb-poc
A proof of concept for looking up DST transitions from the IANA TZDB.

## Usage

**Show all DST transitions for an IANA time-zone ID**
```
cargo run -- <TZID>
```
e.g.
```
cargo run -- America/Los_Angeles
```

**Show DST transitions for an IANA time-zone ID on a given year**
```
cargo run -- <TZID> <YEAR>
```

e.g.
```
cargo run -- America/Los_Angeles 1918
```
