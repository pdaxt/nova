//! Code generation for Nova
//!
//! Generates WebAssembly from the IR.
//!
//! # TODO
//!
//! This module is a work in progress. Contributors welcome!
//!
//! See: https://github.com/nova-lang/nova/issues/5

#![allow(dead_code)]
#![allow(unused_variables)]

use crate::ir::{BasicBlock, Function, Instruction, InstructionKind, IrType, Module, Terminator};

/// Generate WebAssembly binary from IR
pub fn generate(module: &Module) -> Vec<u8> {
    let mut generator = WasmGenerator::new();
    generator.generate(module)
}

/// WebAssembly generator
struct WasmGenerator {
    /// Output buffer
    output: Vec<u8>,
}

impl WasmGenerator {
    fn new() -> Self {
        Self { output: Vec::new() }
    }

    fn generate(&mut self, module: &Module) -> Vec<u8> {
        // WASM magic number and version
        self.emit_bytes(&[0x00, 0x61, 0x73, 0x6D]); // \0asm
        self.emit_bytes(&[0x01, 0x00, 0x00, 0x00]); // version 1

        // Type section (1)
        self.emit_type_section(module);

        // Function section (3)
        self.emit_function_section(module);

        // Export section (7)
        self.emit_export_section(module);

        // Code section (10)
        self.emit_code_section(module);

        std::mem::take(&mut self.output)
    }

    /// Emit raw bytes
    fn emit_bytes(&mut self, bytes: &[u8]) {
        self.output.extend_from_slice(bytes);
    }

    /// Emit a byte
    fn emit_byte(&mut self, byte: u8) {
        self.output.push(byte);
    }

    /// Emit an unsigned LEB128 integer
    fn emit_u32(&mut self, mut value: u32) {
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            self.emit_byte(byte);
            if value == 0 {
                break;
            }
        }
    }

    /// Emit a signed LEB128 integer
    fn emit_i32(&mut self, mut value: i32) {
        loop {
            let byte = (value & 0x7F) as u8;
            value >>= 7;
            let done = (value == 0 && (byte & 0x40) == 0) || (value == -1 && (byte & 0x40) != 0);
            if done {
                self.emit_byte(byte);
                break;
            } else {
                self.emit_byte(byte | 0x80);
            }
        }
    }

    /// Emit a signed LEB128 i64
    fn emit_i64(&mut self, mut value: i64) {
        loop {
            let byte = (value & 0x7F) as u8;
            value >>= 7;
            let done = (value == 0 && (byte & 0x40) == 0) || (value == -1 && (byte & 0x40) != 0);
            if done {
                self.emit_byte(byte);
                break;
            } else {
                self.emit_byte(byte | 0x80);
            }
        }
    }

    /// Emit a section
    fn emit_section(&mut self, id: u8, contents: Vec<u8>) {
        self.emit_byte(id);
        self.emit_u32(contents.len() as u32);
        self.emit_bytes(&contents);
    }

    /// Emit a string (length-prefixed)
    fn emit_string(&mut self, s: &str) {
        self.emit_u32(s.len() as u32);
        self.emit_bytes(s.as_bytes());
    }

    /// Emit the type section
    fn emit_type_section(&mut self, module: &Module) {
        let mut contents = Vec::new();
        let mut gen = WasmGenerator { output: contents };

        // Number of types
        gen.emit_u32(module.functions.len() as u32);

        for func in &module.functions {
            // Function type marker
            gen.emit_byte(0x60);

            // Parameters
            gen.emit_u32(func.params.len() as u32);
            for (_, ty) in &func.params {
                gen.emit_byte(Self::ir_type_to_wasm(ty));
            }

            // Results
            if matches!(func.return_type, IrType::Void) {
                gen.emit_u32(0);
            } else {
                gen.emit_u32(1);
                gen.emit_byte(Self::ir_type_to_wasm(&func.return_type));
            }
        }

        contents = gen.output;
        self.emit_section(1, contents);
    }

    /// Emit the function section
    fn emit_function_section(&mut self, module: &Module) {
        let mut contents = Vec::new();
        let mut gen = WasmGenerator { output: contents };

        gen.emit_u32(module.functions.len() as u32);
        for (i, _) in module.functions.iter().enumerate() {
            gen.emit_u32(i as u32); // Type index
        }

        contents = gen.output;
        self.emit_section(3, contents);
    }

    /// Emit the export section
    fn emit_export_section(&mut self, module: &Module) {
        let mut contents = Vec::new();
        let mut gen = WasmGenerator { output: contents };

        // Export all functions
        gen.emit_u32(module.functions.len() as u32);
        for (i, func) in module.functions.iter().enumerate() {
            gen.emit_string(&func.name);
            gen.emit_byte(0x00); // Function export
            gen.emit_u32(i as u32); // Function index
        }

        contents = gen.output;
        self.emit_section(7, contents);
    }

    /// Emit the code section
    fn emit_code_section(&mut self, module: &Module) {
        let mut contents = Vec::new();
        let mut gen = WasmGenerator { output: contents };

        gen.emit_u32(module.functions.len() as u32);
        for func in &module.functions {
            let func_body = gen.emit_function(func);
            gen.emit_u32(func_body.len() as u32);
            gen.emit_bytes(&func_body);
        }

        contents = gen.output;
        self.emit_section(10, contents);
    }

    /// Emit a function body
    fn emit_function(&mut self, func: &Function) -> Vec<u8> {
        let body = Vec::new();
        let mut gen = WasmGenerator { output: body };

        // Local declarations (simplified: just count locals)
        // TODO: Proper local handling
        gen.emit_u32(0); // No additional locals for now

        // Emit instructions for each block
        for block in &func.blocks {
            gen.emit_block(block);
        }

        // End of function
        gen.emit_byte(0x0B);

        gen.output
    }

    /// Emit a basic block
    fn emit_block(&mut self, block: &BasicBlock) {
        for instr in &block.instructions {
            self.emit_instruction(instr);
        }

        match &block.terminator {
            Terminator::Return(_value) => {
                // Value should already be on stack from last instruction
                // Return is implicit at end of function
            }
            Terminator::Branch(_) => {
                // TODO: br instruction
            }
            Terminator::CondBranch(_, _, _) => {
                // TODO: br_if instruction
            }
            Terminator::Unreachable => {
                self.emit_byte(0x00); // unreachable
            }
        }
    }

    /// Emit an instruction
    fn emit_instruction(&mut self, instr: &Instruction) {
        match &instr.kind {
            InstructionKind::ConstInt(n) => {
                self.emit_byte(0x42); // i64.const
                self.emit_i64(*n);
            }
            InstructionKind::ConstFloat(n) => {
                self.emit_byte(0x44); // f64.const
                self.emit_bytes(&n.to_le_bytes());
            }
            InstructionKind::ConstBool(b) => {
                self.emit_byte(0x41); // i32.const
                self.emit_i32(if *b { 1 } else { 0 });
            }
            InstructionKind::ConstString(_) => {
                // TODO: String handling
                self.emit_byte(0x41); // i32.const (pointer placeholder)
                self.emit_i32(0);
            }
            InstructionKind::Add(_, _) => {
                self.emit_byte(0x7C); // i64.add
            }
            InstructionKind::Sub(_, _) => {
                self.emit_byte(0x7D); // i64.sub
            }
            InstructionKind::Mul(_, _) => {
                self.emit_byte(0x7E); // i64.mul
            }
            InstructionKind::Div(_, _) => {
                self.emit_byte(0x7F); // i64.div_s
            }
            InstructionKind::Rem(_, _) => {
                self.emit_byte(0x81); // i64.rem_s
            }
            InstructionKind::Eq(_, _) => {
                self.emit_byte(0x51); // i64.eq
            }
            InstructionKind::Ne(_, _) => {
                self.emit_byte(0x52); // i64.ne
            }
            InstructionKind::Lt(_, _) => {
                self.emit_byte(0x53); // i64.lt_s
            }
            InstructionKind::Le(_, _) => {
                self.emit_byte(0x57); // i64.le_s
            }
            InstructionKind::Gt(_, _) => {
                self.emit_byte(0x55); // i64.gt_s
            }
            InstructionKind::Ge(_, _) => {
                self.emit_byte(0x59); // i64.ge_s
            }
            InstructionKind::And(_, _) => {
                self.emit_byte(0x83); // i64.and
            }
            InstructionKind::Or(_, _) => {
                self.emit_byte(0x84); // i64.or
            }
            InstructionKind::BitAnd(_, _) => {
                self.emit_byte(0x83); // i64.and
            }
            InstructionKind::BitOr(_, _) => {
                self.emit_byte(0x84); // i64.or
            }
            InstructionKind::BitXor(_, _) => {
                self.emit_byte(0x85); // i64.xor
            }
            InstructionKind::Shl(_, _) => {
                self.emit_byte(0x86); // i64.shl
            }
            InstructionKind::Shr(_, _) => {
                self.emit_byte(0x87); // i64.shr_s
            }
            InstructionKind::Not(_) => {
                self.emit_byte(0x45); // i64.eqz (not really not, but close)
            }
            InstructionKind::Neg(_) => {
                // No direct neg, use 0 - x
                self.emit_byte(0x42); // i64.const 0
                self.emit_i64(0);
                // TODO: Need to handle operand ordering
                self.emit_byte(0x7D); // i64.sub
            }
            InstructionKind::Alloca(_) => {
                // TODO: Stack allocation
            }
            InstructionKind::Load(_) => {
                // TODO: Memory load
                self.emit_byte(0x29); // i64.load
                self.emit_u32(0); // align
                self.emit_u32(0); // offset
            }
            InstructionKind::Store(_, _) => {
                // TODO: Memory store
                self.emit_byte(0x37); // i64.store
                self.emit_u32(0); // align
                self.emit_u32(0); // offset
            }
            InstructionKind::Call(_name, _args) => {
                // TODO: Proper call handling
                self.emit_byte(0x10); // call
                self.emit_u32(0); // function index (placeholder)
            }
            InstructionKind::Phi(_) => {
                // Phi nodes are resolved during SSA construction
            }
            InstructionKind::GetParam(idx) => {
                self.emit_byte(0x20); // local.get
                self.emit_u32(*idx as u32);
            }
        }
    }

    /// Convert IR type to WASM type byte
    fn ir_type_to_wasm(ty: &IrType) -> u8 {
        match ty {
            IrType::I32 => 0x7F,
            IrType::I64 => 0x7E,
            IrType::F32 => 0x7D,
            IrType::F64 => 0x7C,
            IrType::Bool => 0x7F,   // i32
            IrType::Ptr(_) => 0x7F, // i32 (32-bit address space)
            IrType::Void => 0x40,   // empty (for block types)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::lower;
    use crate::lexer::lex;
    use crate::parser::parse;
    use crate::types::check;

    #[test]
    fn test_generate_simple() {
        let source = "fn main() { return 42; }";
        let tokens = lex(source).unwrap();
        let ast = parse(source, tokens).unwrap();
        let typed = check(&ast).unwrap();
        let ir = lower(&typed);
        let wasm = generate(&ir);

        // Check WASM magic number
        assert_eq!(&wasm[0..4], &[0x00, 0x61, 0x73, 0x6D]);
        // Check version
        assert_eq!(&wasm[4..8], &[0x01, 0x00, 0x00, 0x00]);
    }
}
