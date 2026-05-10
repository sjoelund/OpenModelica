#![allow(non_snake_case, unused)]

use mmwinnow::Absyn;

pub type Info = Absyn::Info;

#[derive(Debug)]
pub enum Error {
    EquationsNotAllowed { info: Info },
    ArrayDimNotAllowed { info: Info },
    InitialAlgorithmsNotAllowed { info: Info },
    PderNotAllowed { info: Info },
    OverloadNotAllowed { info: Info },
    WithinNotAllowed { path: Absyn::Path },
    ConstraintsNotAllowed { info: Info },
    DefineUnitNotAllowed { info: Info },
    TextElementNotAllowed { info: Info },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EquationsNotAllowed { info } => write!(f, "{}:{}: equations are not allowed in MetaModelica", info.file_name, info.line_number_start),
            Error::ArrayDimNotAllowed { info } => write!(f, "{}:{}: array dimensions are not allowed in MetaModelica", info.file_name, info.line_number_start),
            Error::InitialAlgorithmsNotAllowed { info } => write!(f, "{}:{}: initial algorithm sections are not allowed in MetaModelica", info.file_name, info.line_number_start),
            Error::PderNotAllowed { info } => write!(f, "{}:{}: partial derivative (pder) is not allowed in MetaModelica", info.file_name, info.line_number_start),
            Error::OverloadNotAllowed { info } => write!(f, "{}:{}: overload is not allowed in MetaModelica", info.file_name, info.line_number_start),
            Error::WithinNotAllowed { path } => write!(f, "within {:?} is not allowed; only top-level programs are supported", path),
            Error::ConstraintsNotAllowed { info } => write!(f, "{}:{}: constraint sections (Optimica) are not allowed in MetaModelica", info.file_name, info.line_number_start),
            Error::DefineUnitNotAllowed { info } => write!(f, "{}:{}: defineunit is not allowed in MetaModelica", info.file_name, info.line_number_start),
            Error::TextElementNotAllowed { info } => write!(f, "{}:{}: TEXT element (parse error placeholder) is not allowed", info.file_name, info.line_number_start),
        }
    }
}

pub type Program = Vec<Class>;

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: String,
    pub partial_prefix: bool,
    pub final_prefix: bool,
    pub encapsulated_prefix: bool,
    pub restriction: Absyn::Restriction,
    pub body: ClassDef,
    pub comments_before_class: Vec<String>,
    pub comments_before_end: Vec<String>,
    pub comments_after_end: Vec<String>,
    pub info: Info,
    /// Rust crate this class belongs to, derived from `__OpenModelica_Interface` annotation.
    /// Only set on top-level packages (excluding "OpenModelica").
    pub crate_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassDef {
    Parts {
        type_vars: Vec<String>,
        class_attrs: Vec<Absyn::NamedArg>,
        members: Vec<ClassMember>,
        algorithms: Vec<Absyn::AlgorithmItem>,
        external: Option<ExternalSection>,
        annotations: Vec<Absyn::Annotation>,
        comment: Option<String>,
    },
    Derived {
        type_spec: Absyn::TypeSpec,
        attributes: Absyn::ElementAttributes,
        arguments: Vec<Absyn::ElementArg>,
        comment: Option<Absyn::Comment>,
    },
    Enumeration {
        enum_literals: Absyn::EnumDef,
        comment: Option<Absyn::Comment>,
    },
    ClassExtends {
        base_class_name: String,
        modifications: Vec<Absyn::ElementArg>,
        comment: Option<String>,
        members: Vec<ClassMember>,
        algorithms: Vec<Absyn::AlgorithmItem>,
        annotations: Vec<Absyn::Annotation>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Protected,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassMember {
    Component(ComponentMember),
    ClassDef(ClassDefMember),
    Extends(ExtendsMember),
    Import(ImportMember),
    LexerComment(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentMember {
    pub visibility: Visibility,
    pub final_prefix: bool,
    pub redeclare_keywords: Option<Absyn::RedeclareKeywords>,
    pub inner_outer: Absyn::InnerOuter,
    pub info: Info,
    pub variability: Absyn::Variability,
    pub direction: Absyn::Direction,
    pub type_spec: Absyn::TypeSpec,
    pub name: String,
    pub modification: Option<Absyn::Modification>,
    pub condition: Option<Absyn::Exp>,
    pub comment: Option<Absyn::Comment>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDefMember {
    pub visibility: Visibility,
    pub final_prefix: bool,
    pub redeclare_keywords: Option<Absyn::RedeclareKeywords>,
    pub info: Info,
    pub replaceable: bool,
    pub class_def: Box<Class>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExtendsMember {
    pub visibility: Visibility,
    pub final_prefix: bool,
    pub redeclare_keywords: Option<Absyn::RedeclareKeywords>,
    pub info: Info,
    pub path: Absyn::Path,
    pub element_args: Vec<Absyn::ElementArg>,
    pub annotation: Option<Absyn::Annotation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportMember {
    pub visibility: Visibility,
    pub info: Info,
    pub import: Absyn::Import,
    pub comment: Option<Absyn::Comment>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExternalSection {
    pub decl: Absyn::ExternalDecl,
    pub annotation: Option<Absyn::Annotation>,
}

pub fn from_program(prog: &Absyn::Program) -> Result<Program, Error> {
    let Absyn::Program::PROGRAM { classes, within_ } = prog;
    if let Absyn::Within::WITHIN { path } = within_ {
        return Err(Error::WithinNotAllowed { path: path.clone() });
    }
    classes.into_iter().map(|class| {
        let is_nf_builtin = matches!(&class, Absyn::Class::CLASS { info, .. } if is_nf_builtin(info));
        let result = convert_class(class.clone());
        if is_nf_builtin { Ok(result.ok().flatten()) } else { result }
    }).filter_map(|r| r.transpose())
    .map(|r| r.map(|mut c| {
        if c.name != "OpenModelica" {
            c.crate_name = extract_crate_name(&c.body);
        }
        c
    }))
    .collect()
}

fn extract_crate_name(body: &ClassDef) -> Option<String> {
    let annotations = match body {
        ClassDef::Parts { annotations, .. } | ClassDef::ClassExtends { annotations, .. } => annotations,
        _ => return None,
    };
    for ann in annotations {
        let Absyn::Annotation::ANNOTATION { elementArgs } = ann;
        for arg in elementArgs {
            if let Absyn::ElementArg::MODIFICATION {
                path: Absyn::Path::IDENT { name },
                modification: Some(Absyn::Modification::CLASSMOD { eqMod: Absyn::EqMod::EQMOD { exp, .. }, .. }),
                ..
            } = arg.as_ref() {
                if name == "__OpenModelica_Interface" {
                    if let Absyn::Exp::STRING { value } = exp.as_ref() {
                        return interface_to_crate(value);
                    }
                }
            }
        }
    }
    None
}

fn interface_to_crate(interface: &str) -> Option<String> {
    match interface {
        "backend" => Some("openmodelica_backend".to_owned()),
        "nf_frontend" => Some("openmodelica_nf_frontend".to_owned()),
        "frontend" => Some("openmodelica_frontend".to_owned()),
        "susan" => Some("openmodelica_susan".to_owned()),
        "util" => Some("openmodelica_util".to_owned()),
        _ => None,
    }
}

fn convert_class(class: Absyn::Class) -> Result<Option<Class>, Error> {
    let Absyn::Class::CLASS {
        name,
        partialPrefix,
        finalPrefix,
        encapsulatedPrefix,
        restriction,
        body,
        commentsBeforeClass,
        commentsBeforeEnd,
        commentsAfterEnd,
        info,
    } = class;
    match restriction {
        Absyn::Restriction::R_MODEL{} => return Ok(None),
        _ => (),
    };
    let converted_body = convert_class_def(body.as_ref(), &info)?;
    Ok(Some(Class {
        name,
        partial_prefix: partialPrefix,
        final_prefix: finalPrefix,
        encapsulated_prefix: encapsulatedPrefix,
        restriction,
        body: converted_body,
        comments_before_class: commentsBeforeClass.into_iter().cloned().collect(),
        comments_before_end: commentsBeforeEnd.into_iter().cloned().collect(),
        comments_after_end: commentsAfterEnd.into_iter().cloned().collect(),
        info,
        crate_name: None,
    }))
}

fn convert_class_def(def: &Absyn::ClassDef, class_info: &Info) -> Result<ClassDef, Error> {
    match def {
        Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts, ann, comment } => {
            let mut members = Vec::new();
            let mut algorithms = Vec::new();
            let mut external = None;
            let lenient = is_nf_builtin(class_info);
            for part in classParts {
                let result = convert_class_part(part.clone(), class_info, &mut members, &mut algorithms, &mut external);
                if !lenient {
                    result?;
                }
            }
            Ok(ClassDef::Parts {
                type_vars: typeVars.into_iter().cloned().collect(),
                class_attrs: classAttrs.into_iter().cloned().collect(),
                members,
                algorithms,
                external,
                annotations: ann.into_iter().cloned().collect(),
                comment: comment.clone(),
            })
        }
        Absyn::ClassDef::DERIVED { typeSpec, attributes, arguments, comment } => {
            check_type_spec_array_dim(typeSpec, class_info)?;
            check_element_attributes_array_dim(attributes, class_info)?;
            Ok(ClassDef::Derived {
                type_spec: typeSpec.clone(),
                attributes: attributes.clone(),
                arguments: arguments.into_iter().map(|a| a.as_ref().clone()).collect(),
                comment: comment.clone(),
            })
        }
        Absyn::ClassDef::ENUMERATION { enumLiterals, comment } => {
            Ok(ClassDef::Enumeration {
                enum_literals: enumLiterals.clone(),
                comment: comment.clone(),
            })
        }
        Absyn::ClassDef::OVERLOAD { .. } => {
            Err(Error::OverloadNotAllowed { info: class_info.clone() })
        }
        Absyn::ClassDef::CLASS_EXTENDS { baseClassName, modifications, comment, parts, ann } => {
            let mut members = Vec::new();
            let mut algorithms = Vec::new();
            let mut external = None;
            for part in parts {
                convert_class_part(part.clone(), class_info, &mut members, &mut algorithms, &mut external)?;
            }
            Ok(ClassDef::ClassExtends {
                base_class_name: baseClassName.clone(),
                modifications: modifications.into_iter().map(|m| m.as_ref().clone()).collect(),
                comment: comment.clone(),
                members,
                algorithms,
                annotations: ann.into_iter().cloned().collect(),
            })
        }
        Absyn::ClassDef::PDER { .. } => {
            Err(Error::PderNotAllowed { info: class_info.clone() })
        }
    }
}

fn convert_class_part(
    part: Absyn::ClassPart,
    class_info: &Info,
    members: &mut Vec<ClassMember>,
    algorithms: &mut Vec<Absyn::AlgorithmItem>,
    external: &mut Option<ExternalSection>,
) -> Result<(), Error> {
    match part {
        Absyn::ClassPart::PUBLIC { contents } => {
            convert_element_items(contents, Visibility::Public, class_info, members)?;
        }
        Absyn::ClassPart::PROTECTED { contents } => {
            convert_element_items(contents, Visibility::Protected, class_info, members)?;
        }
        Absyn::ClassPart::ALGORITHMS { contents } => {
            algorithms.extend(contents.into_iter().cloned());
        }
        Absyn::ClassPart::INITIALALGORITHMS { .. } => {
            return Err(Error::InitialAlgorithmsNotAllowed { info: class_info.clone() });
        }
        Absyn::ClassPart::EQUATIONS { .. } | Absyn::ClassPart::INITIALEQUATIONS { .. } => {
            return Err(Error::EquationsNotAllowed { info: class_info.clone() });
        }
        Absyn::ClassPart::CONSTRAINTS { .. } => {
            return Err(Error::ConstraintsNotAllowed { info: class_info.clone() });
        }
        Absyn::ClassPart::EXTERNAL { externalDecl, annotation_ } => {
            *external = Some(ExternalSection { decl: externalDecl, annotation: annotation_ });
        }
    }
    Ok(())
}

fn is_nf_builtin(info: &Info) -> bool {
    info.file_name.ends_with("NFModelicaBuiltin.mo")
}

fn convert_element_items(
    items: mmwinnow::List<Absyn::ElementItem>,
    visibility: Visibility,
    class_info: &Info,
    members: &mut Vec<ClassMember>,
) -> Result<(), Error> {
    let lenient = is_nf_builtin(class_info);
    for item in &items {
        match item {
            Absyn::ElementItem::LEXER_COMMENT { comment } => {
                members.push(ClassMember::LexerComment(comment.clone()));
            }
            Absyn::ElementItem::ELEMENTITEM { element } => {
                let result = convert_element(element.clone(), visibility.clone(), class_info, members);
                if result.is_err() && !lenient {
                    return result;
                }
            }
        }
    }
    Ok(())
}

fn convert_element(
    element: Absyn::Element,
    visibility: Visibility,
    class_info: &Info,
    members: &mut Vec<ClassMember>,
) -> Result<(), Error> {
    match element {
        Absyn::Element::DEFINEUNIT { info, .. } => {
            return Err(Error::DefineUnitNotAllowed { info });
        }
        Absyn::Element::TEXT { info, .. } => {
            return Err(Error::TextElementNotAllowed { info });
        }
        Absyn::Element::ELEMENT { finalPrefix, redeclareKeywords, innerOuter, specification, info, constrainClass } => {
            match specification {
                Absyn::ElementSpec::CLASSDEF { replaceable_, class_ } => {
                    if let Some(converted) = convert_class(class_.as_ref().clone())? {
                        members.push(ClassMember::ClassDef(ClassDefMember {
                            visibility,
                            final_prefix: finalPrefix,
                            redeclare_keywords: redeclareKeywords,
                            info,
                            replaceable: replaceable_,
                            class_def: Box::new(converted),
                        }));
                    }
                }
                Absyn::ElementSpec::EXTENDS { path, elementArg, annotationOpt } => {
                    members.push(ClassMember::Extends(ExtendsMember {
                        visibility,
                        final_prefix: finalPrefix,
                        redeclare_keywords: redeclareKeywords,
                        info,
                        path,
                        element_args: elementArg.into_iter().map(|a| a.as_ref().clone()).collect(),
                        annotation: annotationOpt,
                    }));
                }
                Absyn::ElementSpec::IMPORT { import_, comment, info: import_info } => {
                    members.push(ClassMember::Import(ImportMember {
                        visibility,
                        info: import_info,
                        import: import_,
                        comment,
                    }));
                }
                Absyn::ElementSpec::COMPONENTS { attributes, typeSpec, components } => {
                    check_type_spec_array_dim(&typeSpec, &info)?;
                    check_element_attributes_array_dim(&attributes, &info)?;
                    let Absyn::ElementAttributes::ATTR {
                        variability,
                        direction,
                        ..
                    } = attributes;
                    for comp_item in &components {
                        let Absyn::ComponentItem::COMPONENTITEM { component, condition, comment } = comp_item.as_ref().clone();
                        let Absyn::Component::COMPONENT { name, arrayDim, modification } = component;
                        check_array_dim(&arrayDim, &info)?;
                        members.push(ClassMember::Component(ComponentMember {
                            visibility: visibility.clone(),
                            final_prefix: finalPrefix,
                            redeclare_keywords: redeclareKeywords.clone(),
                            inner_outer: innerOuter.clone(),
                            info: info.clone(),
                            variability: variability.clone(),
                            direction: direction.clone(),
                            type_spec: typeSpec.clone(),
                            name,
                            modification,
                            condition,
                            comment,
                        }));
                    }
                }
            }
        }
    }
    Ok(())
}

fn check_array_dim(dim: &Absyn::ArrayDim, info: &Info) -> Result<(), Error> {
    if !dim.is_empty() {
        return Err(Error::ArrayDimNotAllowed { info: info.clone() });
    }
    Ok(())
}

fn check_type_spec_array_dim(ts: &Absyn::TypeSpec, info: &Info) -> Result<(), Error> {
    match ts {
        Absyn::TypeSpec::TPATH { arrayDim: Some(_), .. } |
        Absyn::TypeSpec::TCOMPLEX { arrayDim: Some(_), .. } => {
            Err(Error::ArrayDimNotAllowed { info: info.clone() })
        }
        _ => Ok(()),
    }
}

fn check_element_attributes_array_dim(attr: &Absyn::ElementAttributes, info: &Info) -> Result<(), Error> {
    let Absyn::ElementAttributes::ATTR { arrayDim, .. } = attr;
    check_array_dim(arrayDim, info)
}
