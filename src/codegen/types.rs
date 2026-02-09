use std::rc::Rc;
use antlr_rust::tree::ParseTree;

use crate::codegen::emitter::CodeEmitter;
use crate::parser::aslparser::*;

/// Map an ASL type name to a Rust type
pub fn map_type(type_ctx: &Rc<TypeSpecContextAll<'_>>) -> String {
    match type_ctx.as_ref() {
        TypeSpecContextAll::TypeRefContext(ctx) => {
            let name = ctx.get_text();
            match name.as_str() {
                "integer" => "i64".to_string(),
                "boolean" => "bool".to_string(),
                "real" => "f64".to_string(),
                "string" => "String".to_string(),
                "bit" => "u8".to_string(),
                other => other.to_string(),
            }
        }
        TypeSpecContextAll::TypeIndexedContext(ctx) => {
            // e.g., bits(32) → u32
            let text = ctx.get_text();
            if text.starts_with("bits(") {
                let n: u32 = text
                    .trim_start_matches("bits(")
                    .trim_end_matches(')')
                    .parse()
                    .unwrap_or(64);
                match n {
                    1..=8 => "u8".to_string(),
                    9..=16 => "u16".to_string(),
                    17..=32 => "u32".to_string(),
                    33..=64 => "u64".to_string(),
                    _ => format!("u{}", n),
                }
            } else {
                text
            }
        }
        _ => type_ctx.get_text(),
    }
}

/// Return a const-compatible default value for a Rust type string
fn default_value_for(rust_type: &str) -> String {
    match rust_type {
        "i64" => "0".to_string(),
        "bool" => "false".to_string(),
        "f64" => "0.0".to_string(),
        "String" => "String::new()".to_string(),
        t if t.starts_with('u') && t[1..].parse::<u32>().is_ok() => "0".to_string(),
        _ => format!("{}::default()", rust_type),
    }
}

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

pub fn generate_builtin_type(emitter: &mut CodeEmitter, ctx: &DefTypeBuiltinContext<'_>) {
    let name = ctx.id().unwrap().get_text();
    emitter.emit(&format!("// builtin: {}", name));
}

pub fn generate_constant(emitter: &mut CodeEmitter, ctx: &DefConstantContext<'_>) {
    let name = ctx.id().unwrap().get_text();
    let type_spec = ctx.typeSpec().unwrap();
    let rust_type = map_type(&type_spec);
    let value = ctx.expr().unwrap().get_text();

    emitter.emit(&format!("pub const {}: {} = {};", name, rust_type, value));
}

pub fn generate_abstract_type(emitter: &mut CodeEmitter, ctx: &DefTypeAbstractContext<'_>) {
    let name = ctx.id().unwrap().get_text();
    emitter.emit(&format!("pub struct {};", name));
}

pub fn generate_type_alias(emitter: &mut CodeEmitter, ctx: &DefTypeAliasContext<'_>) {
    let name = ctx.id().unwrap().get_text();
    let type_spec = ctx.typeSpec().unwrap();
    let rust_type = map_type(&type_spec);

    emitter.emit(&format!("pub type {} = {};", name, rust_type));
}

pub fn generate_enum(emitter: &mut CodeEmitter, ctx: &DefTypeEnumContext<'_>) {
    let name = ctx.id().unwrap().get_text();
    let variants = ctx.identifierCommaList0().unwrap();
    let ids = variants.id_all();

    emitter.emit("#[derive(Debug, Clone, Copy, PartialEq)]");
    let variant_list: Vec<String> = ids.iter().map(|id| id.get_text()).collect();
    emitter.emit(&format!("pub enum {} {{ {} }}", name, variant_list.join(", ")));
}

pub fn generate_struct(emitter: &mut CodeEmitter, ctx: &DefTypeStructContext<'_>) {
    let name = ctx.qualId().unwrap().get_text();
    let fields = ctx.symDeclCommaList().unwrap();
    let decls = fields.symDecl_all();

    emitter.emit("#[derive(Debug, Clone)]");
    emitter.emit(&format!("pub struct {} {{", name));
    emitter.indent();

    for decl in decls {
        let field_type = map_type(&decl.typeSpec().unwrap());
        let field_name = decl.id().unwrap().get_text();
        emitter.emit(&format!("pub {}: {},", field_name, field_type));
    }

    emitter.dedent();
    emitter.emit("}");
}
