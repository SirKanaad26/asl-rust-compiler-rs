use std::rc::Rc;
use antlr_rust::tree::ParseTree;

use crate::codegen::emitter::CodeEmitter;
use crate::parser::aslparser::{
    RegisterContextAll,
    RegisterContextAttrs,
    RegisterFieldCommaListContextAttrs,
    RegisterFieldContextAttrs,
    ArrayRegisterContextAll,
    ArrayRegisterContextAttrs,
};

pub fn generate_register(emitter: &mut CodeEmitter, reg: &Rc<RegisterContextAll<'_>>) {
    let size: u32 = reg.NAT_LIT().unwrap().get_text().parse().unwrap();
    let name = reg.id().unwrap().get_text();
    let rust_type = size_to_rust_type(size);

    emitter.emit(&format!("pub struct {}({});", name, rust_type));

    generate_bitfield_impl(emitter, reg, &name, false);
}

pub fn generate_array_register(emitter: &mut CodeEmitter, arr: &Rc<ArrayRegisterContextAll<'_>>) {
    let lo: u32 = arr.NAT_LIT(0).unwrap().get_text().parse().unwrap();
    let hi: u32 = arr.NAT_LIT(1).unwrap().get_text().parse().unwrap();
    let count = hi - lo + 1;

    if let Some(reg) = arr.register() {
        let size: u32 = reg.NAT_LIT().unwrap().get_text().parse().unwrap();
        let name = reg.id().unwrap().get_text();
        let rust_type = size_to_rust_type(size);

        emitter.emit(&format!("pub struct {}([{}; {}]);", name, rust_type, count));

        generate_bitfield_impl(emitter, &reg, &name, true);
    }
}

fn generate_bitfield_impl(
    emitter: &mut CodeEmitter,
    reg: &Rc<RegisterContextAll<'_>>,
    name: &str,
    is_array: bool,
) {
    let reg_size: u32 = reg.NAT_LIT().unwrap().get_text().parse().unwrap();

    if let Some(field_list) = reg.registerFieldCommaList() {
        let fields = field_list.registerField_all();

        if !fields.is_empty() {
            emitter.emit("");
            emitter.emit(&format!("impl {} {{", name));
            emitter.indent();

            for field in &fields {
                generate_bitfield_getter(emitter, field, is_array, reg_size);
            }
            
            emitter.emit("");  // Blank line between getters and setters
            
            for field in &fields {
                generate_bitfield_setter(emitter, field, is_array, reg_size);
            }

            emitter.dedent();
            emitter.emit("}");
        }
    }
}

fn generate_bitfield_getter(
    emitter: &mut CodeEmitter,
    field: &Rc<crate::parser::aslparser::RegisterFieldContextAll<'_>>,
    is_array: bool,
    reg_size: u32,  // Add this parameter
) {
    let hi: u32 = field.NAT_LIT(0).unwrap().get_text().parse().unwrap();
    let lo: u32 = field.NAT_LIT(1).unwrap().get_text().parse().unwrap();

    if let Some(field_id) = field.id() {
        let field_name = field_id.get_text();
        let width = hi - lo + 1;
        let field_type = width_to_rust_type(width);

        // Optimize: full-width field needs no masking
        let accessor = if is_array { "self.0[idx]" } else { "self.0" };
        
        let body = if lo == 0 && width == reg_size {
            // Full width, no masking needed
            format!("{}", accessor)
        } else if lo == 0 {
            // No shift needed, just mask
            let mask = (1u64 << width) - 1;
            format!("({} & 0x{:X}) as {}", accessor, mask, field_type)
        } else {
            // Shift and mask
            let mask = (1u64 << width) - 1;
            format!("(({} >> {}) & 0x{:X}) as {}", accessor, lo, mask, field_type)
        };

        if is_array {
            emitter.emit(&format!(
                "pub fn {}(&self, idx: usize) -> {} {{ {} }}",
                field_name, field_type, body
            ));
        } else {
            emitter.emit(&format!(
                "pub fn {}(&self) -> {} {{ {} }}",
                field_name, field_type, body
            ));
        }
    }
}

fn generate_bitfield_setter(
    emitter: &mut CodeEmitter,
    field: &Rc<crate::parser::aslparser::RegisterFieldContextAll<'_>>,
    is_array: bool,
    reg_size: u32,
) {
    let hi: u32 = field.NAT_LIT(0).unwrap().get_text().parse().unwrap();
    let lo: u32 = field.NAT_LIT(1).unwrap().get_text().parse().unwrap();

    if let Some(field_id) = field.id() {
        let field_name = field_id.get_text();
        let width = hi - lo + 1;
        let field_type = width_to_rust_type(width);
        let reg_type = size_to_rust_type(reg_size);

        let accessor = if is_array { "self.0[idx]" } else { "self.0" };

        let body = if lo == 0 && width == reg_size {
            // Full width - direct assign
            "val".to_string()
        } else {
            let mask = (1u64 << width) - 1;
            if lo == 0 {
                // No shift needed
                format!("({} & 0x{:X}) | (val as {})", accessor, !(mask) as u64 & ((1u64 << reg_size) - 1), reg_type)
            } else {
                format!("({} & !(0x{:X} << {})) | ((val as {}) << {})", accessor, mask, lo, reg_type, lo)
            }
        };

        if is_array {
            emitter.emit(&format!(
                "pub fn set_{}(&mut self, idx: usize, val: {}) {{ {} = {}; }}",
                field_name, field_type, accessor, body
            ));
        } else {
            emitter.emit(&format!(
                "pub fn set_{}(&mut self, val: {}) {{ {} = {}; }}",
                field_name, field_type, accessor, body
            ));
        }
    }
}

fn size_to_rust_type(size: u32) -> &'static str {
    match size {
        8 => "u8",
        16 => "u16",
        32 => "u32",
        64 => "u64",
        128 => "u128",
        _ => "u64",
    }
}

fn width_to_rust_type(width: u32) -> &'static str {
    match width {
        1..=8 => "u8",
        9..=16 => "u16",
        17..=32 => "u32",
        _ => "u64",
    }
}
