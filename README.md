# parallel-process-sample

## Overview

This repository is for simple CPU-bound and I/O-bound processing in Rust in single-threaded, multi-threaded (rayon), and asynchronous (tokio) modes, and for comparing speeds.

See below for usage.

```bash
% cargo run
   Compiling app v0.1.0 (~/parallel-process-sample)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/app`
Generating large file...

Processing cpu bound task...
Time taken (Single Thread): 24.106800625s
Time taken (Parallel): 7.583718333s
Time taken (Async): 26.639566208s

Processing io bound task...
Time taken (Single Thread): 61.077112667s
Time taken (Parallel): 8.477685708s
Time taken (Async): 47.490232542s
```
