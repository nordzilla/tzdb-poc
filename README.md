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

America/Los_Angeles  Sun Nov 18 19:59:59 1883 UT = Sun Nov 18 12:07:01 1883 LMT isdst=0 gmtoff=-28378
America/Los_Angeles  Sun Nov 18 20:00:00 1883 UT = Sun Nov 18 12:00:00 1883 PST isdst=0 gmtoff=-28800
America/Los_Angeles  Sun Mar 31 09:59:59 1918 UT = Sun Mar 31 01:59:59 1918 PST isdst=0 gmtoff=-28800
America/Los_Angeles  Sun Mar 31 10:00:00 1918 UT = Sun Mar 31 03:00:00 1918 PDT isdst=1 gmtoff=-25200
America/Los_Angeles  Sun Oct 27 08:59:59 1918 UT = Sun Oct 27 01:59:59 1918 PDT isdst=1 gmtoff=-25200
America/Los_Angeles  Sun Oct 27 09:00:00 1918 UT = Sun Oct 27 01:00:00 1918 PST isdst=0 gmtoff=-28800
...
America/Los_Angeles  Sun Mar  8 09:59:59 2499 UT = Sun Mar  8 01:59:59 2499 PST isdst=0 gmtoff=-28800
America/Los_Angeles  Sun Mar  8 10:00:00 2499 UT = Sun Mar  8 03:00:00 2499 PDT isdst=1 gmtoff=-25200
America/Los_Angeles  Sun Nov  1 08:59:59 2499 UT = Sun Nov  1 01:59:59 2499 PDT isdst=1 gmtoff=-25200
America/Los_Angeles  Sun Nov  1 09:00:00 2499 UT = Sun Nov  1 01:00:00 2499 PST isdst=0 gmtoff=-28800
```

**Show DST transitions for an IANA time-zone ID on a given year**
```
cargo run -- <TZID> <YEAR>
```

e.g.
```
cargo run -- America/Los_Angeles 1918

America/Los_Angeles  Sun Mar 31 09:59:59 1918 UT = Sun Mar 31 01:59:59 1918 PST isdst=0 gmtoff=-28800
America/Los_Angeles  Sun Mar 31 10:00:00 1918 UT = Sun Mar 31 03:00:00 1918 PDT isdst=1 gmtoff=-25200
America/Los_Angeles  Sun Oct 27 08:59:59 1918 UT = Sun Oct 27 01:59:59 1918 PDT isdst=1 gmtoff=-25200
America/Los_Angeles  Sun Oct 27 09:00:00 1918 UT = Sun Oct 27 01:00:00 1918 PST isdst=0 gmtoff=-28800

```
