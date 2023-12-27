# Apache Bench Report

## go

### before late limit

```
$ ab -n 10000 -c 100 'http://localhost:8080/'
This is ApacheBench, Version 2.3 <$Revision: 1903618 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 1000 requests
Completed 2000 requests
Completed 3000 requests
Completed 4000 requests
Completed 5000 requests
Completed 6000 requests
Completed 7000 requests
Completed 8000 requests
Completed 9000 requests
Completed 10000 requests
Finished 10000 requests


Server Software:        
Server Hostname:        localhost
Server Port:            8080

Document Path:          /
Document Length:        42 bytes

Concurrency Level:      100
Time taken for tests:   0.439 seconds
Complete requests:      10000
Failed requests:        0
Total transferred:      1430000 bytes
HTML transferred:       420000 bytes
Requests per second:    22767.94 [#/sec] (mean)
Time per request:       4.392 [ms] (mean)
Time per request:       0.044 [ms] (mean, across all concurrent requests)
Transfer rate:          3179.51 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    2   0.6      2       7
Processing:     1    2   0.6      2       7
Waiting:        0    2   0.6      2       6
Total:          3    4   1.1      4      13

Percentage of the requests served within a certain time (ms)
  50%      4
  66%      4
  75%      4
  80%      5
  90%      5
  95%      6
  98%      7
  99%     12
 100%     13 (longest request)
```

### after rate limit

```
❯ ab -n 100 -c 100 'http://localhost:8080/limited'
This is ApacheBench, Version 2.3 <$Revision: 1903618 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient).....done


Server Software:        
Server Hostname:        localhost
Server Port:            8080

Document Path:          /limited
Document Length:        2 bytes

Concurrency Level:      100
Time taken for tests:   0.016 seconds
Complete requests:      100
Failed requests:        69
   (Connect: 0, Receive: 0, Length: 69, Exceptions: 0)
Non-2xx responses:      69
Total transferred:      16285 bytes
HTML transferred:       1304 bytes
Requests per second:    6066.49 [#/sec] (mean)
Time per request:       16.484 [ms] (mean)
Time per request:       0.165 [ms] (mean, across all concurrent requests)
Transfer rate:          964.77 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    3   0.4      3       4
Processing:     2    4   1.9      4       9
Waiting:        2    4   2.0      4       9
Total:          5    7   1.9      7      12

Percentage of the requests served within a certain time (ms)
  50%      7
  66%      8
  75%      8
  80%      8
  90%     11
  95%     12
  98%     12
  99%     12
 100%     12 (longest request)
```

## rust

### before late limit

```
❯ ab -n 10000 -c 100 'http://localhost:8080/'
This is ApacheBench, Version 2.3 <$Revision: 1903618 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 1000 requests
Completed 2000 requests
Completed 3000 requests
Completed 4000 requests
Completed 5000 requests
Completed 6000 requests
Completed 7000 requests
Completed 8000 requests
Completed 9000 requests
Completed 10000 requests
Finished 10000 requests


Server Software:        
Server Hostname:        localhost
Server Port:            8080

Document Path:          /
Document Length:        42 bytes

Concurrency Level:      100
Time taken for tests:   0.430 seconds
Complete requests:      10000
Failed requests:        0
Total transferred:      1430000 bytes
HTML transferred:       420000 bytes
Requests per second:    23242.03 [#/sec] (mean)
Time per request:       4.303 [ms] (mean)
Time per request:       0.043 [ms] (mean, across all concurrent requests)
Transfer rate:          3245.71 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    2   0.7      2       8
Processing:     1    2   1.3      2      15
Waiting:        1    2   1.3      2      14
Total:          2    4   1.5      4      17

Percentage of the requests served within a certain time (ms)
  50%      4
  66%      4
  75%      4
  80%      4
  90%      5
  95%      6
  98%     10
  99%     12
 100%     17 (longest request)
```

### after rate limit

```
TODO
```