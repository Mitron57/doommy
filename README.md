# Doommy - parser for configuration language

## Launch parameters

Usage: doommy `<FILE>`

Arguments: `<FILE>`

Options:
-h, --help Print help message

## Syntax

- Dictionaries: `$[ NAME: value, ...]`
- Lists: `(list value1 value2 value3)`
- Translation-time constants: `(def NAME value)`
- `value` can be either a dictionary or a list.
- Translation-time evaluation of expressions in infix form `^{expr}`
- `expr` can be pow(a, b) function or a + b, a - b, where `a` and `b` parameters can be either an expression, a numeric
  literal (integer) or a translation-time constant.

## Example
### Input
```doommy
(def PI $[
  AMOGUS: $[
    ABOBA: ^{pow(10, 2) + pow(10, 2)}
  ],
  KEKW: 10
]);
$[A: PI, B: 10];
(list PI 10 ^{pow(10, 2) + 12 - 1})
```
### Output
```yaml
A:
  AMOGUS:
    ABOBA: 200
  KEKW: 10
B: 10
List:
- AMOGUS:
    ABOBA: 200
    KEKW: 10
- 10
- 1111
```

# Requirements
 - Rust toolchain
