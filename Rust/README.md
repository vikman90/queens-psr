# N-Queens problem using backtracking and CS

## Rust implementation

### Set up

```sh
yum install rustc cargo
cargo build --release
```

### Syntax

```
queens [-h] [-p] [SIZE]
```

<dl>
  <dt>-h</dt>
  <dd>Show help.</dd>
  <dt>-p</dt>
  <dd>Enable parallel mode.</dd>
  <dt>SIZE</dt>
  <dd>Length of the chess.</dd>
</dl>

### Example

```
# cargo run --release -- -p 4000

Queen 1: square 1252
Queen 2: square 2295
(...)
Queen 3999: square 3509
Queen 4000: square 1655

Trials:      58765
Discards:    182326630
Time:        4539 ms.
Performance: 0.013 steps/μs.
             40.167 discards/μs.
```
