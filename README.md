# kz80_action

A cross-compiler that translates Action! source code into Z80 machine code. Action! was a popular compiled programming language for Atari 8-bit computers, known for producing fast, efficient code. This compiler brings Action! to the Z80 platform.

## Features

- **Data Types**: BYTE (8-bit unsigned), CARD (16-bit unsigned), INT (16-bit signed), CHAR
- **Arrays**: BYTE ARRAY, CARD ARRAY with indexed access
- **Control Flow**: IF/THEN/ELSE/ELSEIF/FI, WHILE/DO/OD, FOR/TO/STEP/DO/OD, UNTIL/DO/OD
- **Procedures**: PROC (no return value) and FUNC (with return value)
- **Expressions**: Full arithmetic, comparison, and logical operators
- **Built-in Runtime**: PrintB, PrintC, PrintE, Print, PutD, GetD

## Building

Requires Rust toolchain (1.70+):

```bash
cargo build --release
```

The compiler binary will be at `target/release/kz80_action`.

## Usage

```bash
kz80_action -i <source.act> -o <output.bin> [options]
```

### Options

| Option | Description |
|--------|-------------|
| `-i, --input <FILE>` | Input Action! source file |
| `-o, --output <FILE>` | Output binary file (default: input with .bin extension) |
| `--org <ADDRESS>` | Origin address for code (default: 0x4200) |
| `-l, --listing` | Generate listing file (.lst) |
| `-v, --verbose` | Verbose output |

### Example

```bash
# Compile with origin at 0x0000 (for RetroShield emulator)
./target/release/kz80_action -i examples/simple.act -o simple.bin --org 0x0000 -l

# Run on RetroShield emulator
../emulator/retroshield -l simple.bin
```

## Language Reference

### Data Types

```action
BYTE b          ; 8-bit unsigned (0-255)
CARD c          ; 16-bit unsigned (0-65535)
INT i           ; 16-bit signed (-32768 to 32767)
CHAR ch         ; Character (same as BYTE)

BYTE ARRAY buf(100)   ; Array of 100 bytes
CARD ARRAY nums(50)   ; Array of 50 words
```

### Procedures and Functions

```action
; Procedure (no return value)
PROC PrintNumber(BYTE n)
  PrintB(n)
  PrintE()
RETURN

; Function (returns a value)
FUNC BYTE Double(BYTE n)
  RETURN(n * 2)
```

### Control Flow

```action
; IF statement
IF x > 10 THEN
  PrintB(x)
ELSEIF x > 5 THEN
  PrintB(5)
ELSE
  PrintB(0)
FI

; WHILE loop
WHILE i < 100
DO
  i = i + 1
OD

; FOR loop
FOR i = 1 TO 10 STEP 2
DO
  PrintB(i)
  PrintE()
OD
```

### Operators

| Category | Operators |
|----------|-----------|
| Arithmetic | `+`, `-`, `*`, `/`, `MOD` |
| Comparison | `=`, `<>`, `<`, `>`, `<=`, `>=` |
| Logical | `AND`, `OR`, `XOR`, `NOT` |
| Bitwise | `&`, `%`, `!` |
| Unary | `-` (negate), `^` (dereference), `@` (address-of) |

### Comments

```action
; This is a comment (semicolon to end of line)
```

## Built-in Runtime Library

| Function | Description |
|----------|-------------|
| `PrintB(BYTE n)` | Print byte as decimal number (0-255) |
| `PrintC(CARD n)` | Print card as decimal number (0-65535) |
| `PrintE()` | Print end of line (CR+LF) |
| `Print(STRING s)` | Print null-terminated string |
| `PutD(BYTE ch)` | Output a single character |
| `GetD()` | Read a character from input (blocking) |

## Example Programs

### Hello World (Print A-Z)

```action
PROC main()
  BYTE i
  
  i = 65  ; ASCII 'A'
  WHILE i <= 90
  DO
    PutD(i)
    i = i + 1
  OD
  PrintE()
RETURN
```

### Counting

```action
PROC main()
  BYTE i
  
  FOR i = 1 TO 10
  DO
    PrintB(i)
    PrintE()
  OD
RETURN
```

### Fibonacci Sequence

```action
PROC main()
  CARD a, b, temp, count
  
  a = 0
  b = 1
  count = 0
  
  WHILE count < 20
  DO
    PrintC(a)
    PrintE()
    
    temp = a + b
    a = b
    b = temp
    count = count + 1
  OD
RETURN
```

## Memory Layout

The compiler generates code with the following layout:

```
+------------------+ <- Origin (e.g., 0x0000)
| JP entry_point   | 3 bytes
+------------------+
| Runtime Library  | ~109 bytes
+------------------+
| CALL main        | 3 bytes
| HALT             | 1 byte
+------------------+
| User Code        | Variable
+------------------+
| ...              |
+------------------+ <- 0x2000
| Variables (RAM)  | 
+------------------+
```

- Code is placed starting at the origin address
- Variables are allocated starting at 0x2000 (RAM area)
- The first 8KB (0x0000-0x1FFF) is typically ROM on RetroShield

## Target Platform

This compiler targets Z80-based systems with:
- Console I/O on port 0x00 (data) and 0x01 (status)
- Compatible with RetroShield Z80 and similar systems

## License

BSD 3-Clause License. See [LICENSE](LICENSE) for details.

## References

- [Action! Language Reference](https://www.atariarchives.org/action/)
- [Z80 CPU User Manual](https://www.zilog.com/docs/z80/um0080.pdf)
- [RetroShield](https://www.8bitforce.com/projects/retroshield/)
