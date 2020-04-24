# Benchmark with criterion
```
     Running target/release/deps/my_benchmark-f084cbf0dc74af64
Gnuplot not found, disabling plotting
rust native merge sort  time:   [4.9806 us 5.0106 us 5.0521 us]                                      
                        change: [+56.030% +57.424% +59.041%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 17 outliers among 100 measurements (17.00%)
  1 (1.00%) high mild
  16 (16.00%) high severe

rust native quick sort  time:   [3.0379 us 3.0461 us 3.0565 us]                                       
                        change: [-1.0776% -0.4913% +0.2367%] (p = 0.15 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe

merge no recurse        time:   [100.96 us 101.28 us 101.68 us]                               
                        change: [-8.2138% -5.8903% -3.8256%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

merge recurse           time:   [580.18 us 581.25 us 582.65 us]                            
                        change: [-1.4469% -1.2477% -1.0629%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe

merge mine              time:   [1.3029 ms 1.3055 ms 1.3089 ms]                        
                        change: [-1.0443% -0.7673% -0.5089%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe

quick                   time:   [104.31 us 104.62 us 104.99 us]                    
                        change: [+1.0788% +1.7427% +2.4585%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) high mild
  8 (8.00%) high severe

insertion               time:   [7.3088 us 7.3337 us 7.3663 us]                         
                        change: [-25.922% -25.379% -24.888%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) high mild
  9 (9.00%) high severe

bubble                  time:   [28.188 ms 28.213 ms 28.241 ms]                    
                        change: [+0.1515% +0.3523% +0.5543%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  3 (3.00%) high severe
```
