# jlc-convert
Automagic conversion of BOM and CPL CSV files saved from easyeda.com for jclpcb.com


This code can be used to massage the outputs of
the BOM CSV export and PickAndPlace CSV exports
from https://easyeda.com/ web UI into format that is
acceptable for https://jlcpcb.com/

The above statement is correct on 26 May 2020,
but I give no promises about maintaining this forever :-)

The way to run it is as follows:

```
cargo run /path/to/a/csv/file.csv >massaged_destination.csv
```

The code autodetects whether it is a BOM or a PickAndPlace coordinate file,
and will gracefully panic!() if it can not decide which it is :)





