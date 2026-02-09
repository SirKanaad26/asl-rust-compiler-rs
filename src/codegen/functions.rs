use antlr_rust::tree::ParseTree;

use crate::codegen::emitter::CodeEmitter;
use crate::codegen::statements::generate_stmt;
use crate::codegen::types::map_type;
use crate::parser::aslparser::*;

/// Map a returnType node to a Rust return type string.
/// Returns "()" for procedures (no return type).
fn map_return_type(ret: Option<std::rc::Rc<ReturnTypeContextAll<'_>>>) -> String {
    match ret {
        None => "()".to_string(),
        Some(rt) => {
            let types: Vec<String> = rt.typeSpec_all()
                .iter()
                .map(|t| map_type(t))
                .collect();
            if types.len() == 1 {
                types[0].clone()
            } else {
                format!("({})", types.join(", "))
            }
        }
    }
}

pub fn generate_callable(emitter: &mut CodeEmitter, ctx: &DefCallableContext<'_>) {
    let name = ctx.qualId().unwrap().get_text();
    let rust_ret = map_return_type(ctx.returnType());

    // Parameters
    let params = ctx.symDeclCommaList().unwrap();
    let decls = params.symDecl_all();
    let param_list: Vec<String> = decls.iter().map(|d| {
        let ptype = map_type(&d.typeSpec().unwrap());
        let pname = d.id().unwrap().get_text();
        format!("{}: {}", pname, ptype)
    }).collect();

    let has_body = ctx.indentedBlock().is_some();

    if rust_ret == "()" {
        // Procedure
        emitter.emit(&format!("pub fn {}({}) {{", name, param_list.join(", ")));
    } else {
        emitter.emit(&format!("pub fn {}({}) -> {} {{", name, param_list.join(", "), rust_ret));
    }

    emitter.indent();

    if has_body {
        let block = ctx.indentedBlock().unwrap();
        let stmts = block.stmt_all();
        for stmt in &stmts {
            generate_stmt(emitter, &stmt);
        }
        if stmts.is_empty() && rust_ret != "()" {
            emitter.emit("todo!()");
        }
    } else {
        emitter.emit("todo!()");
    }

    emitter.dedent();
    emitter.emit("}");
}
