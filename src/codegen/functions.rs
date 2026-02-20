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
    let name = ctx.qualId().unwrap().get_text().replace('.', "_");
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

    // Shadow parameters as mutable (ASL params are mutable by default)
    for d in &decls {
        let pname = d.id().unwrap().get_text();
        emitter.emit(&format!("let mut {} = {};", pname, pname));
    }

    if has_body {
        let block = ctx.indentedBlock().unwrap();
        let stmts = block.stmt_all();
        for stmt in &stmts {
            let deferred = generate_stmt(emitter, &stmt);
            for d in deferred {
                generate_stmt(emitter, &d);
            }
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

pub fn generate_getter(emitter: &mut CodeEmitter, ctx: &DefGetterContext<'_>) {
    let qual_id = ctx.qualId().unwrap().get_text();
    let parts: Vec<&str> = qual_id.split('.').collect();

    if parts.len() != 2 {
        emitter.emit(&format!("// TODO: unsupported getter qualId format: {}", qual_id));
        return;
    }

    let type_name = parts[0];
    let prop_name = parts[1];
    let rust_ret = map_return_type(ctx.returnType());

    // Check if it's an indexed getter (has parameters)
    let indexed_params = if let Some(param_list) = ctx.symDeclCommaList() {
        let decls = param_list.symDecl_all();
        decls.iter().map(|d| {
            let ptype = map_type(&d.typeSpec().unwrap());
            let pname = d.id().unwrap().get_text();
            format!("{}: {}", pname, ptype)
        }).collect::<Vec<_>>()
    } else {
        vec![]
    };

    // Generate impl block
    emitter.emit(&format!("impl {} {{", type_name));
    emitter.indent();

    // Generate getter method signature
    if indexed_params.is_empty() {
        if rust_ret == "()" {
            emitter.emit(&format!("pub fn {}(&self) {{", prop_name));
        } else {
            emitter.emit(&format!("pub fn {}(&self) -> {} {{", prop_name, rust_ret));
        }
    } else {
        if rust_ret == "()" {
            emitter.emit(&format!("pub fn {}(&self, {}) {{", prop_name, indexed_params.join(", ")));
        } else {
            emitter.emit(&format!("pub fn {}(&self, {}) -> {} {{", prop_name, indexed_params.join(", "), rust_ret));
        }
    }

    emitter.indent();

    // Generate body
    let has_body = ctx.indentedBlock().is_some();
    if has_body {
        let block = ctx.indentedBlock().unwrap();
        let stmts = block.stmt_all();
        for stmt in &stmts {
            let deferred = generate_stmt(emitter, &stmt);
            for d in deferred {
                generate_stmt(emitter, &d);
            }
        }
        if stmts.is_empty() && rust_ret != "()" {
            emitter.emit("todo!()");
        }
    } else {
        emitter.emit("todo!()");
    }

    emitter.dedent();
    emitter.emit("}");

    emitter.dedent();
    emitter.emit("}");
}

pub fn generate_setter(emitter: &mut CodeEmitter, ctx: &DefSetterContext<'_>) {
    let qual_id = ctx.qualId().unwrap().get_text();
    let parts: Vec<&str> = qual_id.split('.').collect();

    if parts.len() != 2 {
        emitter.emit(&format!("// TODO: unsupported setter qualId format: {}", qual_id));
        return;
    }

    let type_name = parts[0];
    let prop_name = parts[1];

    // Get the value parameter from symDecl
    let val_decl = ctx.symDecl().unwrap();
    let val_type = map_type(&val_decl.typeSpec().unwrap());
    let val_name = val_decl.id().unwrap().get_text();

    // Check if it's an indexed setter (has additional parameters)
    let indexed_params = if let Some(param_list) = ctx.setterArgCommaList() {
        use crate::parser::aslparser::SetterArgContextAll;
        let args = param_list.setterArg_all();
        args.iter().map(|arg| {
            match &**arg {
                SetterArgContextAll::SetterRefArgContext(ctx) => {
                    let ptype = map_type(&ctx.typeSpec().unwrap());
                    let pname = ctx.id().unwrap().get_text();
                    format!("{}: &mut {}", pname, ptype)
                }
                SetterArgContextAll::SetterValArgContext(ctx) => {
                    let ptype = map_type(&ctx.typeSpec().unwrap());
                    let pname = ctx.id().unwrap().get_text();
                    format!("{}: {}", pname, ptype)
                }
                _ => "/* unknown setter arg */".to_string()
            }
        }).collect::<Vec<_>>()
    } else {
        vec![]
    };

    // Generate impl block
    emitter.emit(&format!("impl {} {{", type_name));
    emitter.indent();

    // Generate setter method signature
    if indexed_params.is_empty() {
        emitter.emit(&format!("pub fn set_{}(&mut self, {}: {}) {{", prop_name, val_name, val_type));
    } else {
        emitter.emit(&format!("pub fn set_{}(&mut self, {}, {}: {}) {{",
            prop_name, indexed_params.join(", "), val_name, val_type));
    }

    emitter.indent();

    // Shadow value parameter as mutable
    emitter.emit(&format!("let mut {} = {};", val_name, val_name));

    // Generate body
    let has_body = ctx.indentedBlock().is_some();
    if has_body {
        let block = ctx.indentedBlock().unwrap();
        let stmts = block.stmt_all();
        for stmt in &stmts {
            let deferred = generate_stmt(emitter, &stmt);
            for d in deferred {
                generate_stmt(emitter, &d);
            }
        }
        if stmts.is_empty() {
            emitter.emit("todo!()");
        }
    } else {
        emitter.emit("todo!()");
    }

    emitter.dedent();
    emitter.emit("}");

    emitter.dedent();
    emitter.emit("}");
}
