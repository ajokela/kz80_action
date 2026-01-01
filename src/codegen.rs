// Z80 Code Generator for Action! language

use crate::ast::*;
use crate::error::{CompileError, Result};
use crate::runtime::RuntimeSymbols;
use std::collections::HashMap;

// Z80 opcodes (many reserved for future use)
#[allow(dead_code)]
mod opcodes {
    pub const NOP: u8 = 0x00;
    pub const LD_BC_NN: u8 = 0x01;
    pub const LD_DE_NN: u8 = 0x11;
    pub const LD_HL_NN: u8 = 0x21;
    pub const LD_SP_NN: u8 = 0x31;
    pub const LD_A_N: u8 = 0x3E;
    pub const LD_B_N: u8 = 0x06;
    pub const LD_C_N: u8 = 0x0E;
    pub const LD_D_N: u8 = 0x16;
    pub const LD_E_N: u8 = 0x1E;
    pub const LD_H_N: u8 = 0x26;
    pub const LD_L_N: u8 = 0x2E;

    pub const LD_A_HL: u8 = 0x7E;
    pub const LD_HL_A: u8 = 0x77;
    pub const LD_A_DE: u8 = 0x1A;
    pub const LD_DE_A: u8 = 0x12;
    pub const LD_A_BC: u8 = 0x0A;

    pub const LD_A_B: u8 = 0x78;
    pub const LD_A_C: u8 = 0x79;
    pub const LD_A_D: u8 = 0x7A;
    pub const LD_A_E: u8 = 0x7B;
    pub const LD_A_H: u8 = 0x7C;
    pub const LD_A_L: u8 = 0x7D;
    pub const LD_B_A: u8 = 0x47;
    pub const LD_C_A: u8 = 0x4F;
    pub const LD_D_A: u8 = 0x57;
    pub const LD_E_A: u8 = 0x5F;
    pub const LD_H_A: u8 = 0x67;
    pub const LD_L_A: u8 = 0x6F;
    pub const LD_D_H: u8 = 0x54;
    pub const LD_E_L: u8 = 0x5D;
    pub const LD_H_D: u8 = 0x62;
    pub const LD_L_E: u8 = 0x6B;

    pub const LD_NN_A: u8 = 0x32;
    pub const LD_A_NN: u8 = 0x3A;
    pub const LD_NN_HL: u8 = 0x22;
    pub const LD_HL_NN_IND: u8 = 0x2A;

    pub const PUSH_BC: u8 = 0xC5;
    pub const PUSH_DE: u8 = 0xD5;
    pub const PUSH_HL: u8 = 0xE5;
    pub const PUSH_AF: u8 = 0xF5;
    pub const POP_BC: u8 = 0xC1;
    pub const POP_DE: u8 = 0xD1;
    pub const POP_HL: u8 = 0xE1;
    pub const POP_AF: u8 = 0xF1;

    pub const ADD_A_N: u8 = 0xC6;
    pub const ADD_A_B: u8 = 0x80;
    pub const ADD_A_C: u8 = 0x81;
    pub const ADD_A_D: u8 = 0x82;
    pub const ADD_A_E: u8 = 0x83;
    pub const ADD_A_H: u8 = 0x84;
    pub const ADD_A_L: u8 = 0x85;
    pub const ADD_A_HL: u8 = 0x86;
    pub const ADD_HL_BC: u8 = 0x09;
    pub const ADD_HL_DE: u8 = 0x19;
    pub const ADD_HL_HL: u8 = 0x29;

    pub const SUB_N: u8 = 0xD6;
    pub const SUB_B: u8 = 0x90;
    pub const SUB_C: u8 = 0x91;
    pub const SUB_D: u8 = 0x92;
    pub const SUB_E: u8 = 0x93;
    pub const SUB_H: u8 = 0x94;
    pub const SUB_L: u8 = 0x95;

    pub const AND_N: u8 = 0xE6;
    pub const AND_A: u8 = 0xA7;
    pub const AND_B: u8 = 0xA0;
    pub const OR_N: u8 = 0xF6;
    pub const OR_A: u8 = 0xB7;
    pub const XOR_N: u8 = 0xEE;
    pub const XOR_A: u8 = 0xAF;

    pub const CP_N: u8 = 0xFE;
    pub const CP_B: u8 = 0xB8;
    pub const CP_C: u8 = 0xB9;
    pub const CP_D: u8 = 0xBA;
    pub const CP_E: u8 = 0xBB;
    pub const CP_H: u8 = 0xBC;
    pub const CP_L: u8 = 0xBD;

    pub const INC_A: u8 = 0x3C;
    pub const INC_B: u8 = 0x04;
    pub const INC_C: u8 = 0x0C;
    pub const INC_D: u8 = 0x14;
    pub const INC_E: u8 = 0x1C;
    pub const INC_H: u8 = 0x24;
    pub const INC_L: u8 = 0x2C;
    pub const INC_BC: u8 = 0x03;
    pub const INC_DE: u8 = 0x13;
    pub const INC_HL: u8 = 0x23;

    pub const DEC_A: u8 = 0x3D;
    pub const DEC_B: u8 = 0x05;
    pub const DEC_C: u8 = 0x0D;
    pub const DEC_BC: u8 = 0x0B;
    pub const DEC_DE: u8 = 0x1B;
    pub const DEC_HL: u8 = 0x2B;

    pub const JP_NN: u8 = 0xC3;
    pub const JP_Z_NN: u8 = 0xCA;
    pub const JP_NZ_NN: u8 = 0xC2;
    pub const JP_C_NN: u8 = 0xDA;
    pub const JP_NC_NN: u8 = 0xD2;
    pub const JP_HL: u8 = 0xE9;

    pub const JR_N: u8 = 0x18;
    pub const JR_Z_N: u8 = 0x28;
    pub const JR_NZ_N: u8 = 0x20;
    pub const JR_C_N: u8 = 0x38;
    pub const JR_NC_N: u8 = 0x30;

    pub const CALL_NN: u8 = 0xCD;
    pub const RET: u8 = 0xC9;
    pub const RST_00: u8 = 0xC7;
    pub const RST_38: u8 = 0xFF;

    pub const HALT: u8 = 0x76;
    pub const DI: u8 = 0xF3;
    pub const EI: u8 = 0xFB;

    pub const EX_DE_HL: u8 = 0xEB;

    pub const SLA_A: [u8; 2] = [0xCB, 0x27];
    pub const SRA_A: [u8; 2] = [0xCB, 0x2F];
    pub const SRL_A: [u8; 2] = [0xCB, 0x3F];

    pub const CPL: u8 = 0x2F;
    pub const NEG: [u8; 2] = [0xED, 0x44];
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct SymbolInfo {
    address: u16,
    data_type: DataType,
    is_param: bool,
    stack_offset: Option<i16>,  // For local variables/params
}

#[derive(Debug)]
#[allow(dead_code)]
struct ListingEntry {
    address: u16,
    bytes: Vec<u8>,
    source: String,
}

#[allow(dead_code)]
pub struct CodeGenerator {
    origin: u16,
    code: Vec<u8>,
    pc: u16,
    globals: HashMap<String, SymbolInfo>,
    locals: HashMap<String, SymbolInfo>,
    procedures: HashMap<String, u16>,
    label_counter: usize,
    loop_stack: Vec<(u16, u16)>,  // (loop_start, loop_end)
    listing: Vec<ListingEntry>,
    data_section: Vec<u8>,
    data_offset: u16,
    runtime: Option<RuntimeSymbols>,
}

impl CodeGenerator {
    pub fn new(origin: u16) -> Self {
        CodeGenerator {
            origin,
            code: Vec::new(),
            pc: origin,
            globals: HashMap::new(),
            locals: HashMap::new(),
            procedures: HashMap::new(),
            label_counter: 0,
            loop_stack: Vec::new(),
            listing: Vec::new(),
            data_section: Vec::new(),
            data_offset: 0,
            runtime: None,
        }
    }

    pub fn set_runtime_symbols(&mut self, symbols: &RuntimeSymbols) {
        self.runtime = Some(symbols.clone());
    }

    fn emit(&mut self, byte: u8) {
        self.code.push(byte);
        self.pc += 1;
    }

    fn emit_bytes(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.emit(b);
        }
    }

    fn emit_word(&mut self, word: u16) {
        self.emit((word & 0xFF) as u8);
        self.emit((word >> 8) as u8);
    }

    fn current_address(&self) -> u16 {
        self.pc
    }

    #[allow(dead_code)]
    fn new_label(&mut self) -> usize {
        let label = self.label_counter;
        self.label_counter += 1;
        label
    }

    // Patch a 16-bit address at a given location
    fn patch_word(&mut self, addr: u16, value: u16) {
        let offset = (addr - self.origin) as usize;
        self.code[offset] = (value & 0xFF) as u8;
        self.code[offset + 1] = (value >> 8) as u8;
    }

    // Load a byte value into A
    fn emit_load_byte(&mut self, value: u8) {
        self.emit(opcodes::LD_A_N);
        self.emit(value);
    }

    // Load a 16-bit value into HL
    fn emit_load_word(&mut self, value: u16) {
        self.emit(opcodes::LD_HL_NN);
        self.emit_word(value);
    }

    // Load variable into A (byte) or HL (word)
    fn emit_load_var(&mut self, name: &str) -> Result<DataType> {
        if let Some(_info) = self.locals.get(name).cloned() {
            // Local variable - loaded from stack
            // TODO: Implement stack-relative addressing
            return Err(CompileError::CodeGenError {
                message: "Local variables not yet fully implemented".to_string(),
            });
        }

        if let Some(info) = self.globals.get(name).cloned() {
            if info.data_type.is_word() {
                // Load 16-bit value into HL
                self.emit(opcodes::LD_HL_NN_IND);
                self.emit_word(info.address);
            } else {
                // Load 8-bit value into A
                self.emit(opcodes::LD_A_NN);
                self.emit_word(info.address);
            }
            return Ok(info.data_type);
        }

        Err(CompileError::UndefinedVariable { name: name.to_string() })
    }

    // Store A (byte) or HL (word) to variable
    fn emit_store_var(&mut self, name: &str, is_word: bool) -> Result<()> {
        if let Some(info) = self.globals.get(name).cloned() {
            if is_word || info.data_type.is_word() {
                // Store HL to 16-bit variable
                self.emit(opcodes::LD_NN_HL);
                self.emit_word(info.address);
            } else {
                // Store A to 8-bit variable
                self.emit(opcodes::LD_NN_A);
                self.emit_word(info.address);
            }
            return Ok(());
        }

        Err(CompileError::UndefinedVariable { name: name.to_string() })
    }

    // Generate code for expression, result in A (byte) or HL (word)
    fn gen_expression(&mut self, expr: &Expression) -> Result<bool> {
        match expr {
            Expression::Number(n) => {
                if *n >= 0 && *n <= 255 {
                    self.emit_load_byte(*n as u8);
                    Ok(false) // byte result
                } else {
                    self.emit_load_word(*n as u16);
                    Ok(true) // word result
                }
            }

            Expression::Char(c) => {
                self.emit_load_byte(*c as u8);
                Ok(false)
            }

            Expression::Variable(name) => {
                let dt = self.emit_load_var(name)?;
                Ok(dt.is_word())
            }

            Expression::Add(left, right) => {
                let left_word = self.gen_expression(left)?;

                if left_word {
                    // 16-bit addition
                    self.emit(opcodes::PUSH_HL);
                    let right_word = self.gen_expression(right)?;
                    if !right_word {
                        // Promote right to 16-bit
                        self.emit(opcodes::LD_L_A);
                        self.emit(opcodes::LD_H_N);
                        self.emit(0);
                    }
                    self.emit(opcodes::POP_DE);
                    self.emit(opcodes::ADD_HL_DE);
                    Ok(true)
                } else {
                    // 8-bit addition
                    self.emit(opcodes::LD_B_A);
                    let right_word = self.gen_expression(right)?;
                    if right_word {
                        // Promote to 16-bit
                        self.emit(opcodes::LD_C_A); // Save low byte
                        self.emit(opcodes::LD_A_B);
                        self.emit(opcodes::LD_L_A);
                        self.emit(opcodes::LD_H_N);
                        self.emit(0);
                        self.emit(opcodes::LD_D_N);
                        self.emit(0);
                        self.emit(opcodes::LD_E_A);
                        self.emit(opcodes::ADD_HL_DE);
                        Ok(true)
                    } else {
                        self.emit(opcodes::ADD_A_B);
                        Ok(false)
                    }
                }
            }

            Expression::Subtract(left, right) => {
                let left_word = self.gen_expression(left)?;

                if left_word {
                    // 16-bit subtraction using SBC or manual
                    self.emit(opcodes::PUSH_HL);
                    let _right_word = self.gen_expression(right)?;
                    // For simplicity, convert to 16-bit subtraction
                    self.emit(opcodes::LD_D_H);
                    self.emit(opcodes::LD_E_L);
                    self.emit(opcodes::POP_HL);
                    // HL = HL - DE (manual subtract)
                    self.emit(opcodes::AND_A); // Clear carry
                    self.emit(opcodes::LD_A_L);
                    self.emit(opcodes::SUB_E);
                    self.emit(opcodes::LD_L_A);
                    self.emit(opcodes::LD_A_H);
                    self.emit(0x9A); // SBC A, D
                    self.emit(opcodes::LD_H_A);
                    Ok(true)
                } else {
                    self.emit(opcodes::LD_B_A);
                    self.gen_expression(right)?;
                    self.emit(opcodes::LD_C_A);
                    self.emit(opcodes::LD_A_B);
                    self.emit(opcodes::SUB_C);
                    Ok(false)
                }
            }

            Expression::Multiply(left, right) => {
                // Simple 8-bit multiply using repeated addition
                // For 16-bit, would need a runtime routine
                self.gen_expression(left)?;
                self.emit(opcodes::LD_B_A);
                self.gen_expression(right)?;
                self.emit(opcodes::LD_C_A);
                // Call multiply routine
                self.emit(opcodes::CALL_NN);
                // Placeholder - needs runtime library
                self.emit_word(0x0000);
                Ok(false)
            }

            Expression::Equal(left, right) => {
                self.gen_expression(left)?;
                self.emit(opcodes::LD_B_A);
                self.gen_expression(right)?;
                self.emit(opcodes::CP_B);
                // Set A to 1 if equal, 0 otherwise
                self.emit(opcodes::LD_A_N);
                self.emit(0);
                self.emit(opcodes::JR_NZ_N);
                self.emit(1);
                self.emit(opcodes::INC_A);
                Ok(false)
            }

            Expression::NotEqual(left, right) => {
                self.gen_expression(left)?;
                self.emit(opcodes::LD_B_A);
                self.gen_expression(right)?;
                self.emit(opcodes::CP_B);
                // Set A to 1 if not equal, 0 otherwise
                self.emit(opcodes::LD_A_N);
                self.emit(0);
                self.emit(opcodes::JR_Z_N);
                self.emit(1);
                self.emit(opcodes::INC_A);
                Ok(false)
            }

            Expression::Less(left, right) => {
                self.gen_expression(left)?;
                self.emit(opcodes::LD_B_A);
                self.gen_expression(right)?;
                self.emit(opcodes::LD_C_A);
                self.emit(opcodes::LD_A_B);
                self.emit(opcodes::CP_C);
                // Set A to 1 if less (carry set), 0 otherwise
                self.emit(opcodes::LD_A_N);
                self.emit(0);
                self.emit(opcodes::JR_NC_N);
                self.emit(1);
                self.emit(opcodes::INC_A);
                Ok(false)
            }

            Expression::Greater(left, right) => {
                // a > b is the same as b < a
                self.gen_expression(right)?;
                self.emit(opcodes::LD_B_A);
                self.gen_expression(left)?;
                self.emit(opcodes::LD_C_A);
                self.emit(opcodes::LD_A_B);
                self.emit(opcodes::CP_C);
                self.emit(opcodes::LD_A_N);
                self.emit(0);
                self.emit(opcodes::JR_NC_N);
                self.emit(1);
                self.emit(opcodes::INC_A);
                Ok(false)
            }

            Expression::LessEqual(left, right) => {
                // a <= b is the same as !(a > b) = !(b < a) = b >= a
                // Or simpler: a <= b if a < b OR a == b
                self.gen_expression(left)?;
                self.emit(opcodes::LD_B_A);
                self.gen_expression(right)?;
                self.emit(opcodes::LD_C_A);
                self.emit(opcodes::LD_A_B);
                self.emit(opcodes::CP_C);
                // A <= C means carry set (A < C) or zero (A == C)
                self.emit(opcodes::LD_A_N);
                self.emit(1);  // Assume true
                self.emit(opcodes::JR_Z_N);  // If equal, skip JR C and XOR A
                self.emit(3);  // Skip 2 bytes (JR C) + 1 byte (XOR A)
                self.emit(opcodes::JR_C_N);  // If less, skip XOR A
                self.emit(1);
                self.emit(opcodes::XOR_A);  // Otherwise false
                Ok(false)
            }

            Expression::GreaterEqual(left, right) => {
                // a >= b if a > b OR a == b
                self.gen_expression(left)?;
                self.emit(opcodes::LD_B_A);
                self.gen_expression(right)?;
                self.emit(opcodes::LD_C_A);
                self.emit(opcodes::LD_A_B);
                self.emit(opcodes::CP_C);
                // A >= C means no carry (A >= C)
                self.emit(opcodes::LD_A_N);
                self.emit(0);
                self.emit(opcodes::JR_C_N);  // If carry (A < C), result is 0
                self.emit(1);
                self.emit(opcodes::INC_A);   // Otherwise 1
                Ok(false)
            }

            Expression::And(left, right) => {
                self.gen_expression(left)?;
                self.emit(opcodes::LD_B_A);
                self.gen_expression(right)?;
                self.emit(opcodes::AND_B);
                Ok(false)
            }

            Expression::Or(left, right) => {
                self.gen_expression(left)?;
                self.emit(opcodes::LD_B_A);
                self.gen_expression(right)?;
                self.emit(opcodes::OR_A);
                self.emit(opcodes::OR_N);
                self.emit(0); // OR with B would be: LD C,A; LD A,B; OR C
                // Actually need to fix this
                Ok(false)
            }

            Expression::BitAnd(left, right) => {
                self.gen_expression(left)?;
                self.emit(opcodes::LD_B_A);
                self.gen_expression(right)?;
                self.emit(opcodes::AND_B);
                Ok(false)
            }

            Expression::BitOr(left, right) => {
                self.gen_expression(left)?;
                self.emit(opcodes::LD_B_A);
                self.gen_expression(right)?;
                self.emit(opcodes::LD_C_A);
                self.emit(opcodes::LD_A_B);
                self.emit(0xB1); // OR C
                Ok(false)
            }

            Expression::BitXor(left, right) => {
                self.gen_expression(left)?;
                self.emit(opcodes::LD_B_A);
                self.gen_expression(right)?;
                self.emit(opcodes::LD_C_A);
                self.emit(opcodes::LD_A_B);
                self.emit(0xA9); // XOR C
                Ok(false)
            }

            Expression::Negate(inner) => {
                self.gen_expression(inner)?;
                self.emit_bytes(&opcodes::NEG);
                Ok(false)
            }

            Expression::Not(inner) => {
                self.gen_expression(inner)?;
                self.emit(opcodes::CPL);
                Ok(false)
            }

            Expression::FunctionCall { name, args } => {
                // Push arguments in reverse order
                for arg in args.iter().rev() {
                    self.gen_expression(arg)?;
                    self.emit(opcodes::PUSH_AF);
                }

                // Call the function
                if let Some(&addr) = self.procedures.get(name) {
                    self.emit(opcodes::CALL_NN);
                    self.emit_word(addr);
                } else {
                    // Forward reference - will need to patch
                    self.emit(opcodes::CALL_NN);
                    self.emit_word(0x0000); // Placeholder
                }

                // Clean up stack (caller cleanup)
                if !args.is_empty() {
                    let _cleanup = args.len() * 2;
                    for _ in 0..args.len() {
                        self.emit(opcodes::POP_BC);
                    }
                }

                Ok(false) // Assume byte return for now
            }

            Expression::AddressOf(name) => {
                if let Some(info) = self.globals.get(name) {
                    self.emit_load_word(info.address);
                    Ok(true)
                } else {
                    Err(CompileError::UndefinedVariable { name: name.clone() })
                }
            }

            Expression::ArrayAccess { array, index } => {
                // Get array base address
                let info = self.globals.get(array).cloned()
                    .ok_or_else(|| CompileError::UndefinedVariable { name: array.clone() })?;

                // Calculate address: base + index
                self.emit_load_word(info.address);
                self.emit(opcodes::PUSH_HL);
                self.gen_expression(index)?;
                self.emit(opcodes::LD_E_A);
                self.emit(opcodes::LD_D_N);
                self.emit(0);
                self.emit(opcodes::POP_HL);
                self.emit(opcodes::ADD_HL_DE);

                // Load value from (HL)
                self.emit(opcodes::LD_A_HL);
                Ok(false)
            }

            _ => Err(CompileError::CodeGenError {
                message: format!("Unsupported expression: {:?}", expr),
            }),
        }
    }

    // Generate code for statement
    fn gen_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::VarDecl(_var) => {
                // Local variable - allocate on stack
                // For now, skip - handled during procedure setup
                Ok(())
            }

            Statement::Assignment { target, value } => {
                let is_word = self.gen_expression(value)?;
                if is_word {
                    self.emit_store_var(target, true)?;
                } else {
                    self.emit_store_var(target, false)?;
                }
                Ok(())
            }

            Statement::ArrayAssignment { array, index, value } => {
                // Calculate destination address
                let info = self.globals.get(array).cloned()
                    .ok_or_else(|| CompileError::UndefinedVariable { name: array.clone() })?;

                // Evaluate value first, save in B
                self.gen_expression(value)?;
                self.emit(opcodes::LD_B_A);

                // Calculate address
                self.emit_load_word(info.address);
                self.emit(opcodes::PUSH_HL);
                self.gen_expression(index)?;
                self.emit(opcodes::LD_E_A);
                self.emit(opcodes::LD_D_N);
                self.emit(0);
                self.emit(opcodes::POP_HL);
                self.emit(opcodes::ADD_HL_DE);

                // Store value
                self.emit(opcodes::LD_A_B);
                self.emit(opcodes::LD_HL_A);
                Ok(())
            }

            Statement::If { condition, then_block, else_block } => {
                self.gen_expression(condition)?;
                self.emit(opcodes::AND_A); // Set flags

                let else_jump = self.current_address();
                self.emit(opcodes::JP_Z_NN);
                self.emit_word(0x0000); // Placeholder

                // Then block
                for stmt in then_block {
                    self.gen_statement(stmt)?;
                }

                if let Some(else_stmts) = else_block {
                    let end_jump = self.current_address();
                    self.emit(opcodes::JP_NN);
                    self.emit_word(0x0000);

                    // Patch else jump
                    let else_addr = self.current_address();
                    self.patch_word(else_jump + 1, else_addr);

                    // Else block
                    for stmt in else_stmts {
                        self.gen_statement(stmt)?;
                    }

                    // Patch end jump
                    let end_addr = self.current_address();
                    self.patch_word(end_jump + 1, end_addr);
                } else {
                    // Patch else jump to end
                    let end_addr = self.current_address();
                    self.patch_word(else_jump + 1, end_addr);
                }

                Ok(())
            }

            Statement::While { condition, body } => {
                let loop_start = self.current_address();

                self.gen_expression(condition)?;
                self.emit(opcodes::AND_A);

                let exit_jump = self.current_address();
                self.emit(opcodes::JP_Z_NN);
                self.emit_word(0x0000);

                // Push loop context for EXIT
                self.loop_stack.push((loop_start, 0)); // End address TBD

                for stmt in body {
                    self.gen_statement(stmt)?;
                }

                // Jump back to start
                self.emit(opcodes::JP_NN);
                self.emit_word(loop_start);

                // Patch exit jump
                let loop_end = self.current_address();
                self.patch_word(exit_jump + 1, loop_end);

                self.loop_stack.pop();
                Ok(())
            }

            Statement::For { var, start, end, step, body } => {
                // Initialize loop variable
                self.gen_expression(start)?;
                self.emit_store_var(var, false)?;

                let loop_start = self.current_address();

                // Check condition: var <= end
                self.emit_load_var(var)?;
                self.emit(opcodes::LD_B_A);
                self.gen_expression(end)?;
                self.emit(opcodes::LD_C_A);
                self.emit(opcodes::LD_A_B);
                self.emit(opcodes::CP_C);

                // Exit if var > end
                let exit_jump = self.current_address();
                self.emit(opcodes::JP_Z_NN);  // Jump if equal (continue)
                self.emit_word(0x0000);
                self.emit(opcodes::JP_C_NN);  // Jump if less (continue)
                let exit_jump2 = self.current_address() - 3;
                self.emit_word(0x0000);

                // Exit point
                let _real_exit = self.current_address();
                self.emit(opcodes::JP_NN);
                self.emit_word(0x0000);
                let exit_patch = self.current_address() - 2;

                // Continue point
                let continue_addr = self.current_address();
                self.patch_word(exit_jump + 1, continue_addr);
                self.patch_word(exit_jump2, continue_addr);

                // Body
                for stmt in body {
                    self.gen_statement(stmt)?;
                }

                // Increment
                self.emit_load_var(var)?;
                if let Some(step_expr) = step {
                    self.emit(opcodes::LD_B_A);
                    self.gen_expression(step_expr)?;
                    self.emit(opcodes::ADD_A_B);
                } else {
                    self.emit(opcodes::INC_A);
                }
                self.emit_store_var(var, false)?;

                // Loop back
                self.emit(opcodes::JP_NN);
                self.emit_word(loop_start);

                // Patch exit
                let loop_end = self.current_address();
                self.patch_word(exit_patch, loop_end);

                Ok(())
            }

            Statement::Exit => {
                if let Some(&(_, end)) = self.loop_stack.last() {
                    if end != 0 {
                        self.emit(opcodes::JP_NN);
                        self.emit_word(end);
                    } else {
                        // Need forward reference - not fully implemented
                        self.emit(opcodes::JP_NN);
                        self.emit_word(0x0000);
                    }
                }
                Ok(())
            }

            Statement::Return(value) => {
                if let Some(expr) = value {
                    self.gen_expression(expr)?;
                }
                self.emit(opcodes::RET);
                Ok(())
            }

            Statement::ProcCall { name, args } => {
                // Check if this is a runtime library function
                if let Some(ref runtime) = self.runtime {
                    if let Some(addr) = runtime.get_function(name) {
                        // Handle runtime functions specially
                        match name.to_uppercase().as_str() {
                            "PRINTB" => {
                                // PrintB expects byte in A
                                if !args.is_empty() {
                                    self.gen_expression(&args[0])?;
                                }
                                self.emit(opcodes::CALL_NN);
                                self.emit_word(addr);
                                return Ok(());
                            }
                            "PRINTC" => {
                                // PrintC expects CARD in HL
                                if !args.is_empty() {
                                    self.gen_expression(&args[0])?;
                                    // Move to HL if in A
                                    self.emit(opcodes::LD_L_A);
                                    self.emit(opcodes::LD_H_N);
                                    self.emit(0);
                                }
                                self.emit(opcodes::CALL_NN);
                                self.emit_word(addr);
                                return Ok(());
                            }
                            "PRINTE" | "GETD" => {
                                // No arguments
                                self.emit(opcodes::CALL_NN);
                                self.emit_word(addr);
                                return Ok(());
                            }
                            "PUTD" => {
                                // PutD expects character in A
                                if !args.is_empty() {
                                    self.gen_expression(&args[0])?;
                                }
                                self.emit(opcodes::CALL_NN);
                                self.emit_word(addr);
                                return Ok(());
                            }
                            "PRINT" => {
                                // Print expects string pointer in HL
                                if !args.is_empty() {
                                    // Generate address of string
                                    self.gen_expression(&args[0])?;
                                }
                                self.emit(opcodes::CALL_NN);
                                self.emit_word(addr);
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                }

                // Push arguments
                for arg in args.iter().rev() {
                    self.gen_expression(arg)?;
                    self.emit(opcodes::PUSH_AF);
                }

                if let Some(&addr) = self.procedures.get(name) {
                    self.emit(opcodes::CALL_NN);
                    self.emit_word(addr);
                } else {
                    // External or forward reference
                    self.emit(opcodes::CALL_NN);
                    self.emit_word(0x0000);
                }

                // Clean up stack
                for _ in 0..args.len() {
                    self.emit(opcodes::POP_BC);
                }

                Ok(())
            }

            Statement::Block(statements) => {
                for stmt in statements {
                    self.gen_statement(stmt)?;
                }
                Ok(())
            }

            _ => Ok(()), // Skip unimplemented statements
        }
    }

    fn gen_procedure(&mut self, proc: &Procedure) -> Result<()> {
        let proc_addr = self.current_address();
        self.procedures.insert(proc.name.clone(), proc_addr);

        // Clear locals
        self.locals.clear();

        // For now, allocate local variables as if they were globals
        // This is a simplification that won't work for recursion
        // but allows basic programs to work
        for local in &proc.locals {
            self.globals.insert(local.name.clone(), SymbolInfo {
                address: self.data_offset,
                data_type: local.data_type.clone(),
                is_param: false,
                stack_offset: None,
            });
            self.data_offset += local.data_type.size() as u16;
        }

        // Generate body
        for stmt in &proc.body {
            self.gen_statement(stmt)?;
        }

        // Ensure return at end
        self.emit(opcodes::RET);

        Ok(())
    }

    pub fn generate(&mut self, program: &Program) -> Result<Vec<u8>> {
        // First pass: allocate global variables
        // Variables start at 0x2000 (RAM starts here, first 8KB is ROM)
        let mut var_addr: u16 = 0x2000;

        for var in &program.globals {
            self.globals.insert(var.name.clone(), SymbolInfo {
                address: var_addr,
                data_type: var.data_type.clone(),
                is_param: false,
                stack_offset: None,
            });
            var_addr += var.data_type.size() as u16;
        }
        self.data_offset = var_addr;

        // Generate CALL to Main (or first procedure) followed by HALT
        let main_call = self.current_address();
        self.emit(opcodes::CALL_NN);
        self.emit_word(0x0000); // Will patch later
        self.emit(opcodes::HALT);

        // Generate procedures
        for proc in &program.procedures {
            self.gen_procedure(proc)?;
        }

        // Patch main call
        if let Some(&main_addr) = self.procedures.get("Main") {
            self.patch_word(main_call + 1, main_addr);
        } else if let Some(&main_addr) = self.procedures.get("main") {
            // Also check lowercase 'main'
            self.patch_word(main_call + 1, main_addr);
        } else {
            // No Main - call first procedure
            if let Some(proc) = program.procedures.first() {
                if let Some(&addr) = self.procedures.get(&proc.name) {
                    self.patch_word(main_call + 1, addr);
                }
            }
        }

        // Initialize global variables with values
        // (In a more complete implementation, this would be done at runtime startup)

        Ok(self.code.clone())
    }

    pub fn generate_listing(&self) -> String {
        let mut listing = String::new();
        listing.push_str("; Action! Compiler Output\n");
        listing.push_str(&format!("; Origin: ${:04X}\n", self.origin));
        listing.push_str(&format!("; Code size: {} bytes\n\n", self.code.len()));

        // Dump procedures
        listing.push_str("; Procedures:\n");
        for (name, addr) in &self.procedures {
            listing.push_str(&format!(";   {} = ${:04X}\n", name, addr));
        }

        // Dump globals
        listing.push_str("\n; Global variables:\n");
        for (name, info) in &self.globals {
            listing.push_str(&format!(";   {} = ${:04X} ({:?})\n", name, info.address, info.data_type));
        }

        // Hex dump
        listing.push_str("\n; Code:\n");
        for (i, chunk) in self.code.chunks(16).enumerate() {
            let addr = self.origin as usize + i * 16;
            listing.push_str(&format!("{:04X}: ", addr));
            for byte in chunk {
                listing.push_str(&format!("{:02X} ", byte));
            }
            listing.push('\n');
        }

        listing
    }
}
