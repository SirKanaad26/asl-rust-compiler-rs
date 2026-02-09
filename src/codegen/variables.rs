use antlr_rust::tree::ParseTree;

use crate::codegen::emitter::CodeEmitter;
use crate::codegen::types::{map_type, default_value_for};
use crate::parser::aslparser::*;

pub fn generate_variable(emitter: &mut CodeEmitter, ctx: &DefVariableContext<'_>) {
    let type_spec = ctx.typeSpec().unwrap();
    let rust_type = map_type(&type_spec);
    let name = ctx.qualId().unwrap().get_text();
    let default = default_value_for(&rust_type);

    emitter.emit(&format!(
        "pub static {}: std::sync::Mutex<{}> = std::sync::Mutex::new({});",
        name, rust_type, default
    ));
}

pub fn generate_constant(emitter: &mut CodeEmitter, ctx: &DefConstantContext<'_>) {
    let name = ctx.id().unwrap().get_text();
    let type_spec = ctx.typeSpec().unwrap();
    let rust_type = map_type(&type_spec);
    let value = ctx.expr().unwrap().get_text();

    emitter.emit(&format!("pub const {}: {} = {};", name, rust_type, value));
}

pub fn generate_array(emitter: &mut CodeEmitter, ctx: &DefArrayContext<'_>) {
    let type_spec = ctx.typeSpec().unwrap();
    let rust_type = map_type(&type_spec);
    let name = ctx.id().unwrap().get_text();
    let ix = ctx.ixType().unwrap();

    match ix.as_ref() {
        IxTypeContextAll::IxTypeRangeContext(range_ctx) => {
            let begin: i64 = range_ctx.begin.as_ref().unwrap().get_text().parse().unwrap_or(0);
            let end: i64 = range_ctx.end.as_ref().unwrap().get_text().parse().unwrap_or(0);
            let size = (end - begin + 1).max(0);
            let default = default_value_for(&rust_type);

            emitter.emit(&format!(
                "pub static {}: std::sync::Mutex<[{}; {}]> = std::sync::Mutex::new([{}; {}]);",
                name, rust_type, size, default, size
            ));
        }
        IxTypeContextAll::IxTypeRefContext(ref_ctx) => {
            // Indexed by an enum type, e.g. array integer arr[MyEnum]
            let ix_type = ref_ctx.id().unwrap().get_text();
            emitter.emit(&format!(
                "// array {} {}[{}] — enum-indexed arrays not yet supported",
                rust_type, name, ix_type
            ));
        }
        _ => {
            emitter.emit(&format!("// TODO: array {}", name));
        }
    }
}
