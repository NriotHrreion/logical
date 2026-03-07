# `logical`

A simple logical calculator written in Rust

## Usage

### Basic commands

- `vars`: Display all stored variables
- `default`: Switch to default mode
- `table`: Switch to truth table mode
- `exit` or `quit`: Exit the program

### Logic expression evaluation

We have the following logical operations:
- `!`: NOT operation (¬)
- `&`: AND operation (∧)
- `|`: OR operation (∨)
- `>`: Forward implication (→)
- `<`: Reverse implication (←)
- `-`: Bidirectional implication (↔)

And we use `0` or `1` as the logical values.

#### Example

```
logical@default> 1&0
1&0
=0
logical@default> 0|!1&1
0|!1&1
=0
logical@default> ((1&1)|0)&1
((1&1)|0)&1
=1
```

### Variables

We can create variable using `=` symbol. Note that the name length of variables must be 1.

#### Example

```
logical@default> p=1
p=1
logical@default> q=0
q=0
logical@default> p&(q|1&!p)
p&(q|1&!p)
=0
```

### Truth table

Under the truth table mode, when an expression is inputted, the program will print its truth table.

#### Example

```
logical@default> table
logical@table> (!p|q)&((p&r)>p)
(!p|q)&((p&r)>p)
| p | q | r | Result |
|---|---|---|--------|
| 0 | 0 | 0 | 1      |
| 1 | 0 | 0 | 0      |
| 0 | 1 | 0 | 1      |
| 1 | 1 | 0 | 1      |
| 0 | 0 | 1 | 1      |
| 1 | 0 | 1 | 0      |
| 0 | 1 | 1 | 1      |
| 1 | 1 | 1 | 1      |
```

## Build from source

```
git clone https://github.com/NriotHrreion/logical.git
cargo build
./target/debug/logical
```

## License

[MIT](./LICENSE)
