# N-Queens problem using backtracking and CS

## C# implementation

### Set up (Ubuntu 22.04)

```shell
apt install dotnet-sdk-8.0
```

### Build

```shell
dotnet build --configuration Release queens.csproj
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
# bin/Release/net8.0/queens 1024

Queen 1: square 492
Queen 2: square 294
(...)
Queen 1023: square 612
Queen 1024: square 498

Solved in 1024 steps. Time: 137 ms.
```
