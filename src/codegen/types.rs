use std::rc::Rc;
use antlr_rust::tree::ParseTree;

use crate::codegen::emitter::CodeEmitter;
use crate::codegen::expressions::generate_expr;
use crate::parser::aslparser::*;

/// Map an ASL type name to a Rust type
pub fn map_type(type_ctx: &Rc<TypeSpecContextAll<'_>>) -> String {
    match type_ctx.as_ref() {
        TypeSpecContextAll::TypeRefContext(ctx) => {
            let name = ctx.get_text();
            match name.as_str() {
                "integer" => "i128".to_string(),
                "boolean" => "bool".to_string(),
                "real" => "f64".to_string(),
                "string" => "String".to_string(),
                "bit" => "BitVec<1>".to_string(),
                other => other.to_string(),
            }
        }
        TypeSpecContextAll::TypeIndexedContext(ctx) => {
            // e.g., bits(32) → BitVec<32>
            let text = ctx.get_text();
            if text.starts_with("bits(") {
                let inner = text
                    .trim_start_matches("bits(")
                    .trim_end_matches(')');
                if let Ok(n) = inner.parse::<u32>() {
                    format!("BitVec<{}>", n)
                } else {
                    // Variable width (e.g. bits(N) where N is a parameter).
                    // Fall back to BitVec<64> until parametric types are supported.
                    format!("BitVec<64> /* TODO: variable width bits({}) */", inner)
                }
            } else {
                text
            }
        }
        TypeSpecContextAll::TypeArrayContext(ctx) => {
            let elem_type = map_type(&ctx.typeSpec().unwrap());
            match ctx.ixType().unwrap().as_ref() {
                IxTypeContextAll::IxTypeRangeContext(range) => {
                    let begin = generate_expr(range.begin.as_ref().unwrap());
                    let end = generate_expr(range.end.as_ref().unwrap());
                    format!("[{}; {} - {} + 1]", elem_type, end, begin)
                }
                IxTypeContextAll::IxTypeRefContext(ref_ctx) => {
                    format!("[{}; {}]", elem_type, ref_ctx.get_text())
                }
                _ => format!("[{}]", elem_type),
            }
        }
        TypeSpecContextAll::TypeOfContext(_) => {
            "todo!(\"typeof\")".to_string()
        }
        _ => type_ctx.get_text(),
    }
}

/// Return a const-compatible default value for a Rust type string
pub fn default_value_for(rust_type: &str) -> String {
    match rust_type {
        "i128" => "0".to_string(),
        "bool" => "false".to_string(),
        "f64" => "0.0".to_string(),
        "String" => "String::new()".to_string(),
        t if t.starts_with('u') && t[1..].parse::<u32>().is_ok() => "0".to_string(),
        t if t.starts_with("BitVec<") => "BitVec::zero()".to_string(),
        _ => format!("{}::default()", rust_type),
    }
}

pub fn generate_builtin_type(emitter: &mut CodeEmitter, ctx: &DefTypeBuiltinContext<'_>) {
    let name = ctx.id().unwrap().get_text();
    emitter.emit(&format!("// builtin: {}", name));
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
    emitter.emit(&format!("pub use {}::*;", name));
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
