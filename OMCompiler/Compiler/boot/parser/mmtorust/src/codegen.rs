#![allow(unused)]

use std::collections::HashMap;
use std::fmt::Write;
use mmwinnow::Absyn;
use crate::MM;
use crate::hierarchy::{InstanceHierarchy, NameNode, NodeKind, Ty};

pub fn generate_all(hier: &InstanceHierarchy<'_>, output_dir: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(output_dir)?;
    for (name, node) in &hier.top_level {
        let content = generate_file(name, node);
        std::fs::write(format!("{output_dir}/{name}.rs"), content)?;
    }
    Ok(())
}

fn generate_file(top_name: &str, node: &NameNode<'_>) -> String {
    let mut out = String::new();
    writeln!(out, "// Auto-generated from MetaModelica source").unwrap();
    writeln!(out, "#![allow(non_camel_case_types, non_snake_case, dead_code)]").unwrap();
    writeln!(out).unwrap();
    emit_node(&mut out, top_name, node, "");
    out
}

fn emit_node(out: &mut String, name: &str, node: &NameNode<'_>, indent: &str) {
    let NodeKind::Class(c) = &node.kind else { return };
    use Absyn::Restriction::*;
    match &c.restriction {
        R_PACKAGE => {
            let mut children: Vec<_> = node.children.iter().collect();
            children.sort_by_key(|(n, _)| n.as_str());
            for (child_name, child_node) in children {
                emit_node(out, child_name, child_node, indent);
            }
        }
        R_UNIONTYPE => emit_uniontype(out, name, node, c, indent),
        R_TYPE => emit_type_item(out, name, node, c, indent),
        R_RECORD | R_METARECORD { .. } => emit_struct(out, name, node, c, indent),
        _ => {}
    }
}

fn emit_uniontype(out: &mut String, name: &str, node: &NameNode<'_>, c: &MM::Class, indent: &str) {
    match &node.ty {
        Ty::RustEnum(_) => {
            writeln!(out, "{indent}pub enum {name} {{").unwrap();
            for rec_name in &records_in_order(c) {
                let Some(rec_node) = node.children.get(rec_name) else { continue };
                let NodeKind::Class(rc) = &rec_node.kind else { continue };
                match &rec_node.ty {
                    Ty::RustUnitVariant => {
                        writeln!(out, "{indent}    {rec_name},").unwrap();
                    }
                    Ty::RustStruct(_) => {
                        let fields = component_fields(rc, &rec_node.children);
                        if fields.is_empty() {
                            writeln!(out, "{indent}    {rec_name},").unwrap();
                        } else {
                            writeln!(out, "{indent}    {rec_name} {{").unwrap();
                            for (fname, fty) in &fields {
                                writeln!(out, "{indent}        {fname}: {},", fmt_ty(fty)).unwrap();
                            }
                            writeln!(out, "{indent}    }},").unwrap();
                        }
                    }
                    _ => writeln!(out, "{indent}    {rec_name}, // unresolved").unwrap(),
                }
            }
            writeln!(out, "{indent}}}").unwrap();
            writeln!(out).unwrap();
        }
        Ty::AliasTo(rec_name) => {
            let rec_name = rec_name.clone();
            if let Some(rec_node) = node.children.get(&rec_name) {
                if let NodeKind::Class(rc) = &rec_node.kind {
                    emit_struct(out, &rec_name, rec_node, rc, indent);
                }
            }
            writeln!(out, "{indent}pub type {name} = {rec_name};").unwrap();
            writeln!(out).unwrap();
        }
        _ => {}
    }
}

fn emit_struct(out: &mut String, name: &str, node: &NameNode<'_>, c: &MM::Class, indent: &str) {
    let fields = component_fields(c, &node.children);
    if fields.is_empty() {
        writeln!(out, "{indent}pub struct {name};").unwrap();
    } else {
        writeln!(out, "{indent}pub struct {name} {{").unwrap();
        for (fname, fty) in &fields {
            writeln!(out, "{indent}    pub {fname}: {},", fmt_ty(fty)).unwrap();
        }
        writeln!(out, "{indent}}}").unwrap();
    }
    writeln!(out).unwrap();
}

fn emit_type_item(out: &mut String, name: &str, node: &NameNode<'_>, c: &MM::Class, indent: &str) {
    match &c.body {
        MM::ClassDef::Derived { .. } => {
            if !matches!(node.ty, Ty::Unknown) {
                writeln!(out, "{indent}pub type {name} = {};", fmt_ty(&node.ty)).unwrap();
                writeln!(out).unwrap();
            }
        }
        MM::ClassDef::Enumeration { enum_literals, .. } => {
            if let Absyn::EnumDef::ENUMLITERALS { enumLiterals } = enum_literals {
                writeln!(out, "{indent}pub enum {name} {{").unwrap();
                for lit in enumLiterals {
                    let Absyn::EnumLiteral::ENUMLITERAL { literal, .. } = lit;
                    writeln!(out, "{indent}    {literal},").unwrap();
                }
                writeln!(out, "{indent}}}").unwrap();
                writeln!(out).unwrap();
            }
        }
        _ => {}
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn records_in_order(c: &MM::Class) -> Vec<String> {
    let members = match &c.body {
        MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
        _ => return vec![],
    };
    members.iter()
        .filter_map(|m| {
            if let MM::ClassMember::ClassDef(cdm) = m {
                if matches!(cdm.class_def.restriction,
                    Absyn::Restriction::R_RECORD | Absyn::Restriction::R_METARECORD { .. })
                {
                    return Some(cdm.class_def.name.clone());
                }
            }
            None
        })
        .collect()
}

fn component_fields<'a>(c: &'a MM::Class, children: &'a HashMap<String, NameNode<'_>>) -> Vec<(&'a str, &'a Ty)> {
    let members: &[MM::ClassMember] = match &c.body {
        MM::ClassDef::Parts { members, .. } | MM::ClassDef::ClassExtends { members, .. } => members,
        _ => return vec![],
    };
    members.iter()
        .filter_map(|m| {
            if let MM::ClassMember::Component(comp) = m {
                let ty = children.get(&comp.name).map(|n| &n.ty)?;
                Some((comp.name.as_str(), ty))
            } else {
                None
            }
        })
        .collect()
}

fn fmt_ty(ty: &Ty) -> String {
    match ty {
        Ty::Unknown => "/* ? */".to_owned(),
        Ty::I32 => "i32".to_owned(),
        Ty::F64 => "f64".to_owned(),
        Ty::Bool => "bool".to_owned(),
        Ty::Str => "String".to_owned(),
        Ty::Unit => "()".to_owned(),
        Ty::TypeVar(name) => name.clone(),
        Ty::Enumeration(name) | Ty::RustEnum(name) | Ty::RustStruct(name) => name.replace('.', "::"),
        Ty::AliasTo(name) => name.clone(),
        Ty::RustUnitVariant => "()".to_owned(),
        Ty::Option(inner) => format!("Option<{}>", fmt_ty(inner)),
        Ty::List(inner) => format!("Vec<{}>", fmt_ty(inner)),
        Ty::Array(inner) => format!("Array<{}>", fmt_ty(inner)),
        Ty::Tuple(tys) => {
            format!("({})", tys.iter().map(fmt_ty).collect::<Vec<_>>().join(", "))
        }
        Ty::Function { type_vars, inputs, output } => {
            let tvs = if type_vars.is_empty() {
                String::new()
            } else {
                format!("<{}>", type_vars.join(", "))
            };
            let ins = inputs.iter().map(fmt_ty).collect::<Vec<_>>().join(", ");
            format!("{tvs}fn({ins}) -> {}", fmt_ty(output))
        }
        Ty::Generic(name, args) => {
            format!("{name}<{}>", args.iter().map(fmt_ty).collect::<Vec<_>>().join(", "))
        }
    }
}
