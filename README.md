# k-sized-subset

## running
you need cargo installed (`sudo apt-get install cargo`)

Then to run it with s=3 and k=2:
```bash
$ cargo run -- 3 2
s = 3, k = 2
n:  0 = [0, 1] == [0, 1]
n:  1 = [0, 2] == [0, 2]
n:  2 = [1, 2] == [1, 2]
```

Or to run the benchmark
```
$ cargo bench
...
iter f 148 17           time:   [9.5070 us 9.5433 us 9.5772 us]
...
```
