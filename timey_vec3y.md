## with Vec3
```
$ make run
./target/x86_64-apple-darwin/release/nbody 50000000
initial energy: 7.582689449
Energy after 50000000 steps: 6.620290462
```
```
hyperfine "./target/x86_64-apple-darwin/release/nbody 50000000"
Benchmark #1: ./target/x86_64-apple-darwin/release/nbody 50000000
  Time (mean ± σ):      4.256 s ±  0.006 s    [User: 4.249 s, System: 0.004 s]
  Range (min … max):    4.248 s …  4.266 s    10 runs
```

## at 6-idiomatic
```sh
$ make run
./target/x86_64-apple-darwin/release/nbody 50000000
initial energy: 7.582689449
Energy after 50000000 steps: 6.620290462
```
```
hyperfine "./target/x86_64-apple-darwin/release/nbody 50000000"
Benchmark #1: ./target/x86_64-apple-darwin/release/nbody 50000000
  Time (mean ± σ):      3.530 s ±  0.006 s    [User: 3.524 s, System: 0.003 s]
  Range (min … max):    3.524 s …  3.546 s    10 runs
```

