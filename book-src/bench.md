# Benchmark with criterion
```
     Running target/release/deps/my_benchmark-90cd95d3bbcaed14
rust native merge sort  time:   [4.5550 us 4.5661 us 4.5758 us]                                       
                        change: [+13.481% +13.929% +14.343%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

rust native quick sort  time:   [6.1192 us 6.1255 us 6.1335 us]                                      
                        change: [+10.848% +11.156% +11.452%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 17 outliers among 100 measurements (17.00%)
  2 (2.00%) low severe
  8 (8.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe

merge no recurse        time:   [166.88 us 167.14 us 167.43 us]                               
                        change: [+11.114% +11.482% +11.865%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

merge recurse           time:   [807.07 us 808.71 us 810.77 us]                            
                        change: [+5.6297% +6.0602% +6.4663%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe

merge mine              time:   [1.7124 ms 1.7144 ms 1.7169 ms]                        
                        change: [-0.6588% -0.3253% +0.0222%] (p = 0.06 > 0.05)
                        No change in performance detected.

quick                   time:   [127.85 us 128.08 us 128.33 us]                    
                        change: [+4.6607% +4.9541% +5.2417%] (p = 0.00 < 0.05)
                        Performance has regressed.

insertion               time:   [11.086 us 11.108 us 11.131 us]                         
                        change: [+9.4496% +10.173% +10.746%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 20 outliers among 100 measurements (20.00%)
  3 (3.00%) low severe
  6 (6.00%) low mild
  3 (3.00%) high mild
  8 (8.00%) high severe  
```
