# N-Queens problem using backtracking and CS

## C++ implementation

### Set up (Ubuntu 22.04)

```shell
apt install g++ make
```

### Build

```shell
make
```

### Syntax

```
queens [-test] [SIZE]
```

<dl>
  <dt>-test</dt>
  <dd>Enable test output.</dd>
  <dt>SIZE</dt>
  <dd>Length of the chess.</dd>
</dl>

### Example

```
# ./queens 1024

Queen 1: square 492
Queen 2: square 294
(...)
Queen 1023: square 873
Queen 1024: square 179

Steps:       19019
Discards:    906012
Time:        476.74 ms.
Performance: 39 steps/ms.
             1900 discards/ms.
```
