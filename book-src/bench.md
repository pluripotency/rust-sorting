# Benchmark with criterion
```
     Running target/release/deps/my_benchmark-9d644afdc652a87e
Gnuplot not found, disabling plotting
rust native merge sort  time:   [5.0220 us 5.0313 us 5.0425 us]                                      
                        change: [-1.5588% -0.8888% -0.2054%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe

merge native            time:   [3.2382 us 3.2466 us 3.2571 us]                             
                        change: [-2.3863% -1.6555% -0.9289%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 14 outliers among 100 measurements (14.00%)
  3 (3.00%) high mild
  11 (11.00%) high severe

merge insertion         time:   [119.06 us 119.55 us 120.19 us]                              
                        change: [+0.4254% +1.2821% +2.2263%] (p = 0.01 < 0.05)
                        Change within noise threshold.

insertion native        time:   [7.3418 us 7.3547 us 7.3715 us]                                
                        change: [-0.1983% +0.1002% +0.4605%] (p = 0.58 > 0.05)
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  7 (7.00%) high severe

insertion               time:   [9.7943 us 9.8181 us 9.8477 us]                         
                        change: [-0.1672% +0.2148% +0.5486%] (p = 0.25 > 0.05)
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe

merge no recurse        time:   [104.63 us 104.86 us 105.16 us]                               
                        change: [-1.3147% -0.7338% -0.2131%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

merge recurse           time:   [592.00 us 593.56 us 595.12 us]                            
                        change: [-1.5462% -1.2052% -0.8706%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe

rust native quick sort  time:   [4.9951 us 5.0192 us 5.0474 us]                                      
                        change: [+0.5483% +1.0532% +1.5536%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

quick                   time:   [129.53 us 129.83 us 130.19 us]                    
                        change: [-0.4525% -0.1205% +0.2271%] (p = 0.49 > 0.05)
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) low mild
  4 (4.00%) high mild
  4 (4.00%) high severe
```
