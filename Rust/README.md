# N-Queens problem using backtracking and CS

## Rust implementation

### Set up (Ubuntu 22.04)

```shell
apt install cargo
```

### Build

```shell
cargo build --release
```

### Syntax

```
queens [-h] [-p] [-test] [SIZE]
```

<dl>
  <dt>-h</dt>
  <dd>Print help.</dd>
  <dt>-p</dt>
  <dd>Enable parallel mode.</dd>
  <dt>-test</dt>
  <dd>Enable test output.</dd>
  <dt>SIZE</dt>
  <dd>Length of the chess.</dd>
</dl>

### Example

```
# cargo run --release -- -p 1024

Queen 1: square 313
Queen 2: square 3
(...)
Queen 1023: square 776
Queen 1024: square 549
Trials:      12290
Discards:    10912362
Time:        654 ms.
Performance: 0.019 steps/μs.
             16.678 discards/μs.
```
