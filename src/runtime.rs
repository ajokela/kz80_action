// Z80 Runtime library for Action! compiler
// Provides built-in procedures and functions

/// Generate the runtime library code
/// Returns (code bytes, symbol table with addresses)
pub fn generate_runtime(base_address: u16) -> (Vec<u8>, RuntimeSymbols) {
    let mut code = Vec::new();
    let mut symbols = RuntimeSymbols::new();

    let mut addr = base_address;

    // Console I/O port addresses (RetroShield compatible)
    const CONSOLE_DATA: u8 = 0x00;
    const CONSOLE_STATUS: u8 = 0x01;

    // ============================================================
    // PrintB - Print byte as decimal number (0-255)
    // Input: A = byte to print
    // ============================================================
    symbols.print_b = addr;
    // Save the value
    code.push(0xF5);  // PUSH AF
    addr += 1;

    // Convert to decimal and print
    // Divide by 100
    code.push(0x06); code.push(100);  // LD B, 100
    addr += 2;
    code.push(0xCD); // CALL div8
    let div8_call1 = code.len();
    code.push(0x00); code.push(0x00);  // placeholder
    addr += 3;

    // If quotient > 0, print it
    code.push(0xB7);  // OR A
    addr += 1;
    code.push(0x28); code.push(0x06);  // JR Z, skip_hundreds (+6 bytes to skip)
    addr += 2;
    code.push(0xC6); code.push(0x30);  // ADD A, '0'
    addr += 2;
    code.push(0xD3); code.push(CONSOLE_DATA);  // OUT (CONSOLE_DATA), A
    addr += 2;
    code.push(0x3E); code.push(0x01);  // LD A, 1 (flag: printed something)
    addr += 2;
    // skip_hundreds:

    // Get remainder, divide by 10
    code.push(0x79);  // LD A, C (remainder)
    addr += 1;
    code.push(0x06); code.push(10);  // LD B, 10
    addr += 2;
    code.push(0xCD);  // CALL div8
    let div8_call2 = code.len();
    code.push(0x00); code.push(0x00);  // placeholder
    addr += 3;

    // Print tens digit (always if we printed hundreds, or if > 0)
    code.push(0xC6); code.push(0x30);  // ADD A, '0'
    addr += 2;
    code.push(0xD3); code.push(CONSOLE_DATA);  // OUT (CONSOLE_DATA), A
    addr += 2;

    // Print ones digit
    code.push(0x79);  // LD A, C (remainder)
    addr += 1;
    code.push(0xC6); code.push(0x30);  // ADD A, '0'
    addr += 2;
    code.push(0xD3); code.push(CONSOLE_DATA);  // OUT (CONSOLE_DATA), A
    addr += 2;

    code.push(0xF1);  // POP AF
    addr += 1;
    code.push(0xC9);  // RET
    addr += 1;

    // ============================================================
    // PrintC - Print CARD (16-bit) as decimal number
    // Input: HL = value to print
    // ============================================================
    symbols.print_c = addr;
    code.push(0xE5);  // PUSH HL
    addr += 1;
    code.push(0xD5);  // PUSH DE
    addr += 1;
    code.push(0xC5);  // PUSH BC
    addr += 1;

    // We'll use a simple repeated subtraction approach
    // For each power of 10 (10000, 1000, 100, 10, 1)
    // Note: This is a simplified version

    // Print HL as 5-digit decimal (with leading zero suppression)
    // For now, just print low byte
    code.push(0x7D);  // LD A, L
    addr += 1;
    code.push(0xCD);  // CALL PrintB
    code.push((symbols.print_b & 0xFF) as u8);
    code.push((symbols.print_b >> 8) as u8);
    addr += 3;

    code.push(0xC1);  // POP BC
    addr += 1;
    code.push(0xD1);  // POP DE
    addr += 1;
    code.push(0xE1);  // POP HL
    addr += 1;
    code.push(0xC9);  // RET
    addr += 1;

    // ============================================================
    // PrintE - Print end of line (CR+LF)
    // ============================================================
    symbols.print_e = addr;
    code.push(0x3E); code.push(0x0D);  // LD A, 13 (CR)
    addr += 2;
    code.push(0xD3); code.push(CONSOLE_DATA);  // OUT (CONSOLE_DATA), A
    addr += 2;
    code.push(0x3E); code.push(0x0A);  // LD A, 10 (LF)
    addr += 2;
    code.push(0xD3); code.push(CONSOLE_DATA);  // OUT (CONSOLE_DATA), A
    addr += 2;
    code.push(0xC9);  // RET
    addr += 1;

    // ============================================================
    // Print - Print a null-terminated string
    // Input: HL = pointer to string
    // ============================================================
    symbols.print = addr;
    code.push(0x7E);  // print_loop: LD A, (HL)
    addr += 1;
    code.push(0xB7);  // OR A
    addr += 1;
    code.push(0xC8);  // RET Z (if null terminator)
    addr += 1;
    code.push(0xD3); code.push(CONSOLE_DATA);  // OUT (CONSOLE_DATA), A
    addr += 2;
    code.push(0x23);  // INC HL
    addr += 1;
    code.push(0x18); code.push(0xF7);  // JR print_loop (-9)
    addr += 2;

    // ============================================================
    // GetD - Get a character from console (blocking)
    // Output: A = character read
    // ============================================================
    symbols.get_d = addr;
    code.push(0xDB); code.push(CONSOLE_STATUS);  // IN A, (CONSOLE_STATUS)
    addr += 2;
    code.push(0xE6); code.push(0x01);  // AND 1 (check RX ready)
    addr += 2;
    code.push(0x28); code.push(0xFA);  // JR Z, GetD (loop until ready)
    addr += 2;
    code.push(0xDB); code.push(CONSOLE_DATA);  // IN A, (CONSOLE_DATA)
    addr += 2;
    code.push(0xC9);  // RET
    addr += 1;

    // ============================================================
    // PutD - Output a character to console
    // Input: A = character to output
    // ============================================================
    symbols.put_d = addr;
    code.push(0xD3); code.push(CONSOLE_DATA);  // OUT (CONSOLE_DATA), A
    addr += 2;
    code.push(0xC9);  // RET
    addr += 1;

    // ============================================================
    // Multiply - 16-bit multiply (HL = HL * DE)
    // Input: HL, DE = 16-bit values
    // Output: HL = result (low 16 bits)
    // ============================================================
    symbols.multiply = addr;
    code.push(0xC5);  // PUSH BC
    addr += 1;
    code.push(0x44);  // LD B, H
    addr += 1;
    code.push(0x4D);  // LD C, L
    addr += 1;
    code.push(0x21); code.push(0x00); code.push(0x00);  // LD HL, 0
    addr += 3;
    code.push(0x06); code.push(16);  // LD B, 16 (bit counter)
    addr += 2;
    // mult_loop:
    let mult_loop = addr;
    code.push(0x29);  // ADD HL, HL (shift result left)
    addr += 1;
    code.push(0xCB); code.push(0x23);  // SLA E
    addr += 2;
    code.push(0xCB); code.push(0x12);  // RL D (shift DE left, carry = high bit)
    addr += 2;
    code.push(0x30); code.push(0x01);  // JR NC, skip_add
    addr += 2;
    code.push(0x09);  // ADD HL, BC
    addr += 1;
    // skip_add:
    code.push(0x10);  // DJNZ mult_loop
    let offset = (mult_loop as i32 - addr as i32 - 1) as i8;
    code.push(offset as u8);
    addr += 2;
    code.push(0xC1);  // POP BC
    addr += 1;
    code.push(0xC9);  // RET
    addr += 1;

    // ============================================================
    // div8 - 8-bit division
    // Input: A = dividend, B = divisor
    // Output: A = quotient, C = remainder
    // ============================================================
    symbols.div8 = addr;
    // Patch the earlier calls
    let div8_addr = addr;
    code[div8_call1] = (div8_addr & 0xFF) as u8;
    code[div8_call1 + 1] = (div8_addr >> 8) as u8;
    code[div8_call2] = (div8_addr & 0xFF) as u8;
    code[div8_call2 + 1] = (div8_addr >> 8) as u8;

    // Correct division algorithm:
    // C = dividend (becomes remainder)
    // D = quotient
    code.push(0x4F);  // LD C, A (C = dividend)
    addr += 1;
    code.push(0x16); code.push(0x00);  // LD D, 0 (quotient = 0)
    addr += 2;
    // div8_loop:
    let div8_loop = addr;
    code.push(0x79);  // LD A, C (A = current dividend)
    addr += 1;
    code.push(0xB8);  // CP B (compare with divisor)
    addr += 1;
    code.push(0x38); code.push(0x05);  // JR C, div8_done (if A < B, done)
    addr += 2;
    code.push(0x90);  // SUB B (A = A - B)
    addr += 1;
    code.push(0x4F);  // LD C, A (update remainder)
    addr += 1;
    code.push(0x14);  // INC D (quotient++)
    addr += 1;
    code.push(0x18);  // JR div8_loop
    let offset2 = (div8_loop as i32 - addr as i32 - 1) as i8;
    code.push(offset2 as u8);
    addr += 2;
    // div8_done:
    code.push(0x7A);  // LD A, D (return quotient in A)
    addr += 1;
    code.push(0xC9);  // RET
    addr += 1;

    symbols.end_address = addr;

    (code, symbols)
}

#[derive(Debug, Clone)]
pub struct RuntimeSymbols {
    pub print_b: u16,      // Print byte as decimal
    pub print_c: u16,      // Print CARD as decimal
    pub print_e: u16,      // Print end of line
    pub print: u16,        // Print string
    pub get_d: u16,        // Get character
    pub put_d: u16,        // Put character
    pub multiply: u16,     // 16-bit multiply
    pub div8: u16,         // 8-bit divide
    pub end_address: u16,  // Address after runtime
}

impl RuntimeSymbols {
    pub fn new() -> Self {
        RuntimeSymbols {
            print_b: 0,
            print_c: 0,
            print_e: 0,
            print: 0,
            get_d: 0,
            put_d: 0,
            multiply: 0,
            div8: 0,
            end_address: 0,
        }
    }

    /// Get the address of a runtime function by name
    pub fn get_function(&self, name: &str) -> Option<u16> {
        match name.to_uppercase().as_str() {
            "PRINTB" => Some(self.print_b),
            "PRINTC" => Some(self.print_c),
            "PRINTE" => Some(self.print_e),
            "PRINT" => Some(self.print),
            "GETD" => Some(self.get_d),
            "PUTD" => Some(self.put_d),
            _ => None,
        }
    }
}
