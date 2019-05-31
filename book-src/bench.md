# Benchmark with criterion
```
 Running target/release/deps/my_benchmark-90cd95d3bbcaed14
 rust native merge sort  time:   [4.6193 us 4.6286 us 4.6373 us]
                         change: [+4.7729% +5.4420% +6.0999%] (p = 0.00 < 0.05)
                      Performance has regressed.

                      rust native quick sort  time:   [4.1546 us 4.1717 us 4.1947 us]
                                              change: [-4.2186% -3.2044% -1.8682%] (p = 0.00 < 0.05)
                      Performance has improved.
                      Found 8 outliers among 100 measurements (8.00%)
5 (5.00%) high mild
  3 (3.00%) high severe

  merge no recurse        time:   [156.38 us 156.57 us 156.79 us]
                          change: [-7.6287% -7.2171% -6.7943%] (p = 0.00 < 0.05)
                      Performance has improved.
                      Found 8 outliers among 100 measurements (8.00%)
5 (5.00%) high mild
  3 (3.00%) high severe

  merge recurse           time:   [769.43 us 771.09 us 772.84 us]
                          change: [-12.478% -11.985% -11.553%] (p = 0.00 < 0.05)
                      Performance has improved.
                      Found 5 outliers among 100 measurements (5.00%)
3 (3.00%) high mild
  2 (2.00%) high severe

  merge mine              time:   [1.6512 ms 1.6551 ms 1.6595 ms]
                          change: [-15.967% -15.637% -15.308%] (p = 0.00 < 0.05)
                      Performance has improved.
                      Found 4 outliers among 100 measurements (4.00%)
4 (4.00%) high mild

quick                   time:   [119.24 us 119.51 us 119.84 us]
                        change: [-15.304% -14.916% -14.497%] (p = 0.00 < 0.05)
                      Performance has improved.
                      Found 5 outliers among 100 measurements (5.00%)
4 (4.00%) high mild
  1 (1.00%) high severe

  insertion               time:   [14.442 us 14.505 us 14.581 us]
                          change: [-13.193% -12.349% -11.454%] (p = 0.00 < 0.05)
                      Performance has improved.
                      Found 4 outliers among 100 measurements (4.00%)
2 (2.00%) high mild
  2 (2.00%) high severe
```
