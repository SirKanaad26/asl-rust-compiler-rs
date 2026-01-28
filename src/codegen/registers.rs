use std::rc::Rc;
use antlr_rust::tree::ParseTree;

use crate::codegen::emitter::CodeEmitter;
use crate::parser::aslparser::{
    RegisterContextAll,
    RegisterContextAttrs,
    RegisterFieldCommaListContextAttrs,
    RegisterFieldContextAttrs,
};

pub fn generate_register(emitter: &mut CodeEmitter, reg: &Rc<RegisterContextAll<'_>>) {
    let size: u32 = reg.NAT_LIT().unwrap().get_text().parse().unwrap();
    let name = reg.id().unwrap().get_text();

    let rust_type = size_to_rust_type(size);

    emitter.emit(&format!("pub struct {}({});", name, rust_type));

    // Generate bitfield getters
    if let Some(field_list) = reg.registerFieldCommaList() {
        let fields = field_list.registerField_all();

        if !fields.is_empty() {
            emitter.emit("");
            emitter.emit(&format!("impl {} {{", name));
            emitter.indent();

            for field in fields {
                generate_bitfield_getter(emitter, &field);
            }

            emitter.dedent();
            emitter.emit("}");
        }
    }
}

fn generate_bitfield_getter(
    emitter: &mut CodeEmitter,
    field: &Rc<crate::parser::aslparser::RegisterFieldContextAll<'_>>,
) {
    let hi: u32 = field.NAT_LIT(0).unwrap().get_text().parse().unwrap();
    let lo: u32 = field.NAT_LIT(1).unwrap().get_text().parse().unwrap();

    if let Some(field_id) = field.id() {
        let field_name = field_id.get_text();
        let width = hi - lo + 1;
        let mask = (1u64 << width) - 1;
        let field_type = width_to_rust_type(width);

        emitter.emit(&format!(
            "pub fn {}(&self) -> {} {{ ((self.0 >> {}) & 0x{:X}) as {} }}",
            field_name, field_type, lo, mask, field_type
        ));
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
