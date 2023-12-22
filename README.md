# Native CRDT benchmark

| Tasks                   | automerge        | loro             | diamond-type    | y-octo           | yrs                |
| :---------------------- | :--------------- | :--------------- | :-------------- | :--------------- | :----------------- |
| automerge - apply       | 403.96 ± 1.25 ms | 59.92 ± 0.30 ms  | 16.20 ± 0.04 ms | 374.11 ± 1.29 ms | 3939.76 ± 14.43 ms |
| automerge - decode time | 393.88 ± 2.59 ms | 1.02 ± 0.00 ms   | 2.30 ± 0.01 ms  | 86.16 ± 0.34 ms  | 3.84 ± 0.01 ms     |
| automerge - encode time | 10.79 ± 0.07 ms  | 1.01 ± 0.01 ms   | 1.15 ± 0.01 ms  | 8.78 ± 0.18 ms   | 367.55 ± 1.35 us   |
| concurrent list inserts | 83.52 ± 1.14 ms  | 104.18 ± 0.29 ms | 60.32 ± 0.22 ms | 373.84 ± 1.50 us | 16.41 ± 0.12 ms    |
| list_random_insert_1k   | 295.41 ± 0.99 ms | 7.99 ± 0.01 ms   | 4.52 ± 0.01 ms  | 4.73 ± 0.01 ms   | 5.42 ± 0.01 ms     |
