# N-Queens problem using backtracking and CS

## Java implementation

### Set up (Ubuntu 22.04)

```shell
apt install openjdk-19-jdk-headless
```

### Build

```shell
make
```

### Syntax

```
queens.jar [-test] [SIZE]
```

<dl>
  <dt>-test</dt>
  <dd>Enable test output.</dd>
  <dt>SIZE</dt>
  <dd>Length of the chess.</dd>
</dl>

### Example

```
# java -jar queens.jar 1024

Queen 1: square 492
Queen 2: square 294
(...)
Queen 1023: square 647
Queen 1024: square 955

Solved in 1142 steps. Time: 92 ms.
```
