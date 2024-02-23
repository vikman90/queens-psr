# N-Queens problem using backtracking and CS

## Go implementation

### Set up (Ubuntu 22.04)

```shell
apt install golang
```

### Build

```shell
go build
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

[0] 338
[1] 591
(...)
[1022] 155
[1023] 790
Steps:       1051
Discards:    750108
Time:        43.254331ms
Performance: 24 steps/ms
```
