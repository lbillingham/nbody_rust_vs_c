# n-body simulation in Rust and C

Following along with Cliff Biffle's [learn Rust the dangerous way](http://cliffle.com/p/dangerust),
we'll be converting an unsafe, heavilly-optimized n-body simulation from C ([from the The Computer Language Benchmarks Game](https://benchmarksgame-team.pages.debian.net/benchmarksgame/performance/nbody.html)) to Rust.

## Running

There is a top level `makefile` next to this `README`.
If you `make` you should see the `C` and `Rust` implimentations
get built and run for the default number of timesteps.

e.g.

```sh
make
```

gives

```sh
vvvvvv results vvvvvv
/usr/bin/make -C ./nbody/ run
./nbody-blah.run 50000000
-0.169075164
-0.169059907
^^^^^^ results ^^^^^^
```

The top-level `makefile` defers to 2 sub-makefiles
next on the `c` and `rust` folders.
