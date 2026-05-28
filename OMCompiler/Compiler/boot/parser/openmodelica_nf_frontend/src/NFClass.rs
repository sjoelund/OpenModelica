// Auto-generated from MetaModelica source
/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-2026, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF AGPL VERSION 3 LICENSE OR
 * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.8.
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES
 * RECIPIENT'S ACCEPTANCE OF THE OSMC PUBLIC LICENSE OR THE GNU AGPL
 * VERSION 3, ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the OSMC (Open Source Modelica Consortium)
 * Public License (OSMC-PL) are obtained from OSMC, either from the above
 * address, from the URLs:
 * http://www.openmodelica.org or
 * https://github.com/OpenModelica/ or
 * http://www.ida.liu.se/projects/OpenModelica,
 * and in the OpenModelica distribution.
 *
 * GNU AGPL version 3 is obtained from:
 * https://www.gnu.org/licenses/licenses.html#GPL
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::BaseModelica;
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFInstNode::InstNode;
use crate::NFModifier::Modifier;
use crate::NFRecord as Record;
use crate::NFRestriction as Restriction;
use crate::NFSections as Sections;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode::Element;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::IOStream;
use openmodelica_util::System;
use openmodelica_util::Util;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NFClass {
    NOT_INSTANTIATED,
    PARTIAL_CLASS {
        elements: Arc<ClassTree::ClassTree>,
        modifier: Arc<Modifier::Modifier>,
        ccMod: Arc<Modifier::Modifier>,
        prefixes: Arc<Prefixes::Prefixes>,
    },
    PARTIAL_BUILTIN {
        ty: Arc<Type::NFType>,
        elements: Arc<ClassTree::ClassTree>,
        modifier: Arc<Modifier::Modifier>,
        prefixes: Arc<Prefixes::Prefixes>,
        restriction: Arc<Restriction::NFRestriction>,
    },
    EXPANDED_CLASS {
        elements: Arc<ClassTree::ClassTree>,
        modifier: Arc<Modifier::Modifier>,
        ccMod: Arc<Modifier::Modifier>,
        prefixes: Arc<Prefixes::Prefixes>,
        restriction: Arc<Restriction::NFRestriction>,
    },
    EXPANDED_DERIVED {
        baseClass: Arc<InstNode::InstNode>,
        modifier: Arc<Modifier::Modifier>,
        ccMod: Arc<Modifier::Modifier>,
        dims: metamodelica::Array<Arc<Dimension::NFDimension>>,
        prefixes: Arc<Prefixes::Prefixes>,
        attributes: Arc<Attributes::NFAttributes>,
        restriction: Arc<Restriction::NFRestriction>,
    },
    INSTANCED_CLASS {
        ty: Arc<Type::NFType>,
        elements: Arc<ClassTree::ClassTree>,
        sections: Arc<Sections::NFSections>,
        prefixes: Arc<Prefixes::Prefixes>,
        restriction: Arc<Restriction::NFRestriction>,
    },
    INSTANCED_BUILTIN {
        ty: Arc<Type::NFType>,
        elements: Arc<ClassTree::ClassTree>,
        restriction: Arc<Restriction::NFRestriction>,
    },
    TYPED_DERIVED {
        ty: Arc<Type::NFType>,
        baseClass: Arc<InstNode::InstNode>,
        restriction: Arc<Restriction::NFRestriction>,
    },
    DAE_TYPE {
        ty: Arc<DAE::Type>,
    },
}
pub use self::NFClass::{NOT_INSTANTIATED,PARTIAL_CLASS,PARTIAL_BUILTIN,EXPANDED_CLASS,EXPANDED_DERIVED,INSTANCED_CLASS,INSTANCED_BUILTIN,TYPED_DERIVED,DAE_TYPE};
pub static DEFAULT_PREFIXES: std::sync::LazyLock<Arc<Prefixes::Prefixes>> = std::sync::LazyLock::new(|| { Arc::new(Prefixes::Prefixes { encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, replaceablePrefix: Arc::new(openmodelica_frontend_types::SCode::Replaceable::NOT_REPLACEABLE) }) });

pub mod Prefixes {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Prefixes {
        pub encapsulatedPrefix: SCode::Encapsulated,
        pub partialPrefix: SCode::Partial,
        pub finalPrefix: SCode::Final,
        pub innerOuter: Absyn::InnerOuter,
        pub replaceablePrefix: Arc<SCode::Replaceable>,
    }

    impl Default for Prefixes {
        fn default() -> Self {
            Self {
                encapsulatedPrefix: Default::default(),
                partialPrefix: Default::default(),
                finalPrefix: Default::default(),
                innerOuter: Default::default(),
                replaceablePrefix: Default::default(),
            }
        }
    }

    pub type PREFIXES = Prefixes;

    pub fn isEqual(mut prefs1: Arc<Prefixes>, mut prefs2: Arc<Prefixes>) -> bool {
        let mut isEqual: bool = prefs1.clone() == prefs2.clone();
        isEqual
    }

    pub fn isPartial(mut prefs: Arc<Prefixes>) -> bool {
        let mut isPartial: bool = SCodeUtil::partialBool(prefs.partialPrefix.clone()).unwrap();
        isPartial
    }

    pub fn isEncapsulated(mut prefs: Arc<Prefixes>) -> bool {
        let mut isEncapsulated: bool = SCodeUtil::encapsulatedBool(prefs.encapsulatedPrefix.clone()).unwrap();
        isEncapsulated
    }

}

pub fn fromSCode(mut elements: Arc<metamodelica::List<Arc<Element>>>, mut isClassExtends: bool, mut scope: Arc<InstNode::InstNode>, mut prefixes: Arc<Prefixes::Prefixes>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = Arc::new(NFClass::NOT_INSTANTIATED);
    let mut tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    tree = ClassTree::fromSCode(elements.clone(), isClassExtends.clone(), scope.clone())?;
    cls = Arc::new(NFClass::PARTIAL_CLASS { elements: tree.clone(), modifier: Arc::new(crate::NFModifier::Modifier::NOMOD), ccMod: Arc::new(crate::NFModifier::Modifier::NOMOD), prefixes: prefixes.clone() });
    Ok(cls)
}

pub fn initImports(mut cls: Arc<NFClass>, mut parent: Arc<InstNode::InstNode>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_CLASS; elements = ClassTree::initImports(var_field!((*cls).elements, NFClass::PARTIAL_CLASS).clone(), parent.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cls)
}

pub fn fromEnumeration(mut literals: Arc<metamodelica::List<Arc<SCode::Enum>>>, mut enumType: Arc<Type::NFType>, mut prefixes: Arc<Prefixes::Prefixes>, mut enumClass: Arc<InstNode::InstNode>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = Arc::new(NFClass::NOT_INSTANTIATED);
    let mut tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    tree = ClassTree::fromEnumeration(literals.clone(), enumType.clone(), enumClass.clone())?;
    cls = Arc::new(NFClass::PARTIAL_BUILTIN { ty: enumType.clone(), elements: tree.clone(), modifier: Arc::new(crate::NFModifier::Modifier::NOMOD), prefixes: prefixes.clone(), restriction: Arc::new(crate::NFRestriction::ENUMERATION) });
    Ok(cls)
}

pub fn makeRecordConstructor(mut fields: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut out: Arc<InstNode::InstNode>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = Arc::new(NFClass::NOT_INSTANTIATED);
    let mut tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    tree = ClassTree::fromRecordConstructor(fields.clone(), out.clone())?;
    cls = Arc::new(NFClass::INSTANCED_CLASS { ty: Arc::new(crate::NFType::UNKNOWN), elements: tree.clone(), sections: Arc::new(crate::NFSections::EMPTY), prefixes: DEFAULT_PREFIXES.clone(), restriction: Arc::new(crate::NFRestriction::RECORD_CONSTRUCTOR) });
    Ok(cls)
}

pub fn initExpandedClass(mut cls: Arc<NFClass>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = cls;
    cls = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => Arc::new(NFClass::EXPANDED_CLASS { elements: var_field!((*cls).elements, NFClass::PARTIAL_CLASS).clone(), modifier: var_field!((*cls).modifier, NFClass::PARTIAL_CLASS).clone(), ccMod: var_field!((*cls).ccMod, NFClass::PARTIAL_CLASS).clone(), prefixes: var_field!((*cls).prefixes, NFClass::PARTIAL_CLASS).clone(), restriction: Arc::new(crate::NFRestriction::UNKNOWN) }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(cls)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getSections(mut cls: Arc<NFClass>) -> Result<Arc<Sections::NFSections>> {
    let mut sections: Arc<Sections::NFSections> = Arc::new(Sections::EMPTY);
    sections = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ INSTANCED_CLASS { .. } => var_field!((*cls).sections, NFClass::INSTANCED_CLASS).clone(),
        Deref @ TYPED_DERIVED { .. } => getSections(InstNode::getClass(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone())?)?,
        _ => Arc::new(crate::NFSections::EMPTY),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sections)
}

pub fn setSections(mut sections: Arc<Sections::NFSections>, mut cls: Arc<NFClass>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = cls;
    cls = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ INSTANCED_CLASS { .. } => Arc::new(NFClass::INSTANCED_CLASS { ty: var_field!((*cls).ty, NFClass::INSTANCED_CLASS).clone(), elements: var_field!((*cls).elements, NFClass::INSTANCED_CLASS).clone(), sections: sections.clone(), prefixes: var_field!((*cls).prefixes, NFClass::INSTANCED_CLASS).clone(), restriction: var_field!((*cls).restriction, NFClass::INSTANCED_CLASS).clone() }),
        Deref @ TYPED_DERIVED { .. } => {
            InstNode::classApply(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone(), (std::sync::Arc::new(setSections) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Sections::NFSections>, Arc<NFClass>) -> Result<Arc<NFClass>> + 'static>), sections.clone())?;
            cls.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cls)
}

pub fn lookupElement(mut name: ArcStr, mut cls: Arc<NFClass>) -> Result<(Arc<InstNode::InstNode>, bool)> {
    let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut isImport: bool = false;
    (node, isImport) = ClassTree::lookupElement((name.clone()).clone(), classTree(cls.clone())?)?;
    Ok((node, isImport))
}

pub fn tryLookupElement(mut name: ArcStr, mut cls: Arc<NFClass>) -> (Option<Arc<InstNode::InstNode>>, bool) {
    let mut node: Option<Arc<InstNode::InstNode>> = None;
    let mut isImport: bool = false;
    let mut n: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    match '__try0: {
        (n, isImport) = unwrap_break_err!(ClassTree::lookupElement((name.clone()).clone(), classTree(cls.clone()).unwrap()), '__try0);
        node = Some(n.clone());
        Ok::<_, anyhow::Error>((isImport.clone(), node.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            isImport = __try0_o0;
            node = __try0_o1;
        }
        Err(_) => {
            node = None;
            isImport = false;
        }
    }
    (node, isImport)
}

pub fn lookupComponentIndex(mut name: ArcStr, mut cls: Arc<NFClass>) -> Result<i32> {
    let mut index: i32 = 0;
    index = ClassTree::lookupComponentIndex((name.clone()).clone(), classTree(cls.clone())?)?;
    Ok(index)
}

pub fn nthComponent(mut index: i32, mut cls: Arc<NFClass>) -> Result<Arc<InstNode::InstNode>> {
    let mut component: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    component = ClassTree::nthComponent(index.clone(), classTree(cls.clone())?)?;
    Ok(component)
}

pub fn getComponents(mut cls: Arc<NFClass>) -> metamodelica::Array<Arc<InstNode::InstNode>> {
    let mut comps: metamodelica::Array<Arc<InstNode::InstNode>> = ClassTree::getComponents(classTree(cls.clone()).unwrap()).unwrap();
    comps
}

pub fn lookupAttributeBinding(mut name: ArcStr, mut cls: Arc<NFClass>) -> Arc<Binding::NFBinding> {
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut attr_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    match '__try0: {
        (attr_node, _) = unwrap_break_err!(ClassTree::lookupElement((name.clone()).clone(), classTree(cls.clone()).unwrap()), '__try0);
        binding = Component::getBinding(InstNode::component(attr_node.clone()).unwrap());
        Ok::<_, anyhow::Error>((binding.clone(),))
    } {
        Ok((__try0_o0,)) => {
            binding = __try0_o0;
        }
        Err(_) => {
            binding = Binding::EMPTY_BINDING().clone();
        }
    }
    binding
}

pub fn lookupAttributeValue(mut name: ArcStr, mut cls: Arc<NFClass>) -> Option<Arc<Expression::NFExpression>> {
    let mut value: Option<Arc<Expression::NFExpression>> = Binding::typedExp(lookupAttributeBinding((name.clone()).clone(), cls.clone()));
    value
}

pub fn isOnlyBuiltin(mut cls: Arc<NFClass>) -> bool {
    let mut builtin: bool = false;
    builtin = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_BUILTIN { .. } => true,
        Deref @ INSTANCED_BUILTIN { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    builtin
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isBuiltin(mut cls: Arc<NFClass>) -> Result<bool> {
    let mut isBuiltin: bool = false;
    isBuiltin = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_BUILTIN { .. } => true,
        Deref @ INSTANCED_BUILTIN { .. } => true,
        Deref @ EXPANDED_DERIVED { .. } => self::isBuiltin(InstNode::getClass(var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone())?)?,
        Deref @ TYPED_DERIVED { .. } => self::isBuiltin(InstNode::getClass(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone())?)?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isBuiltin)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn classTree(mut cls: Arc<NFClass>) -> Result<Arc<ClassTree::ClassTree>> {
    let mut tree: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
    tree = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => var_field!((*cls).elements, NFClass::PARTIAL_CLASS).clone(),
        Deref @ PARTIAL_BUILTIN { .. } => var_field!((*cls).elements, NFClass::PARTIAL_BUILTIN).clone(),
        Deref @ EXPANDED_CLASS { .. } => var_field!((*cls).elements, NFClass::EXPANDED_CLASS).clone(),
        Deref @ EXPANDED_DERIVED { .. } => classTree(InstNode::getClass(var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone())?)?,
        Deref @ INSTANCED_CLASS { .. } => var_field!((*cls).elements, NFClass::INSTANCED_CLASS).clone(),
        Deref @ INSTANCED_BUILTIN { .. } => var_field!((*cls).elements, NFClass::INSTANCED_BUILTIN).clone(),
        Deref @ TYPED_DERIVED { .. } => classTree(InstNode::getClass(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone())?)?,
        _ => Arc::new(crate::NFClassTree::ClassTree::EMPTY_TREE),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tree)
}

pub fn setClassTree(mut tree: Arc<ClassTree::ClassTree>, mut cls: Arc<NFClass>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_CLASS; elements = tree.clone());
            ()
        },
        Deref @ EXPANDED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_CLASS; elements = tree.clone());
            ()
        },
        Deref @ PARTIAL_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_BUILTIN; elements = tree.clone());
            ()
        },
        Deref @ INSTANCED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_CLASS; elements = tree.clone());
            ()
        },
        Deref @ INSTANCED_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_BUILTIN; elements = tree.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cls)
}

pub fn classTreeApply(mut cls: Arc<NFClass>, mut func: Arc<dyn ::std::ops::Fn(Arc<ClassTree::ClassTree>) -> Result<Arc<ClassTree::ClassTree>> + 'static>) -> Arc<NFClass> {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn(Arc<ClassTree::ClassTree>) -> Result<Arc<ClassTree::ClassTree>> + 'static>;

    let mut cls: Arc<NFClass> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_CLASS; elements = func(var_field!((*cls).elements, NFClass::PARTIAL_CLASS).clone()).unwrap());
            ()
        },
        Deref @ EXPANDED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_CLASS; elements = func(var_field!((*cls).elements, NFClass::EXPANDED_CLASS).clone()).unwrap());
            ()
        },
        Deref @ PARTIAL_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_BUILTIN; elements = func(var_field!((*cls).elements, NFClass::PARTIAL_BUILTIN).clone()).unwrap());
            ()
        },
        Deref @ INSTANCED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_CLASS; elements = func(var_field!((*cls).elements, NFClass::INSTANCED_CLASS).clone()).unwrap());
            ()
        },
        Deref @ INSTANCED_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_BUILTIN; elements = func(var_field!((*cls).elements, NFClass::INSTANCED_BUILTIN).clone()).unwrap());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cls
}

pub fn getModifier(mut cls: Arc<NFClass>) -> Arc<Modifier::Modifier> {
    let mut modifier: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    modifier = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => var_field!((*cls).modifier, NFClass::PARTIAL_CLASS).clone(),
        Deref @ EXPANDED_CLASS { .. } => var_field!((*cls).modifier, NFClass::EXPANDED_CLASS).clone(),
        Deref @ EXPANDED_DERIVED { .. } => var_field!((*cls).modifier, NFClass::EXPANDED_DERIVED).clone(),
        Deref @ PARTIAL_BUILTIN { .. } => var_field!((*cls).modifier, NFClass::PARTIAL_BUILTIN).clone(),
        _ => Arc::new(crate::NFModifier::Modifier::NOMOD),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    modifier
}

pub fn getCCModifier(mut cls: Arc<NFClass>) -> Arc<Modifier::Modifier> {
    let mut modifier: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    modifier = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => var_field!((*cls).ccMod, NFClass::PARTIAL_CLASS).clone(),
        Deref @ EXPANDED_CLASS { .. } => var_field!((*cls).ccMod, NFClass::EXPANDED_CLASS).clone(),
        Deref @ EXPANDED_DERIVED { .. } => var_field!((*cls).ccMod, NFClass::EXPANDED_DERIVED).clone(),
        _ => Arc::new(crate::NFModifier::Modifier::NOMOD),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    modifier
}

pub fn setModifier(mut modifier: Arc<Modifier::Modifier>, mut cls: Arc<NFClass>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_CLASS; modifier = modifier.clone());
            ()
        },
        Deref @ EXPANDED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_CLASS; modifier = modifier.clone());
            ()
        },
        Deref @ EXPANDED_DERIVED { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_DERIVED; modifier = modifier.clone());
            ()
        },
        Deref @ PARTIAL_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_BUILTIN; modifier = modifier.clone());
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFClass.setModifier")); __mm_s.push_str(&*literal!(" got non-modifiable class")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cls)
}

pub fn mergeModifier(mut modifier: Arc<Modifier::Modifier>, mut cls: Arc<NFClass>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_CLASS; modifier = Modifier::merge(modifier.clone(), var_field!((*cls).modifier, NFClass::PARTIAL_CLASS).clone(), (literal!("")).clone())?);
            ()
        },
        Deref @ EXPANDED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_CLASS; modifier = Modifier::merge(modifier.clone(), var_field!((*cls).modifier, NFClass::EXPANDED_CLASS).clone(), (literal!("")).clone())?);
            ()
        },
        Deref @ EXPANDED_DERIVED { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_DERIVED; modifier = Modifier::merge(modifier.clone(), var_field!((*cls).modifier, NFClass::EXPANDED_DERIVED).clone(), (literal!("")).clone())?);
            ()
        },
        Deref @ PARTIAL_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_BUILTIN; modifier = Modifier::merge(modifier.clone(), var_field!((*cls).modifier, NFClass::PARTIAL_BUILTIN).clone(), (literal!("")).clone())?);
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFClass.mergeModifier")); __mm_s.push_str(&*literal!(" got non-modifiable class")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cls)
}

pub fn isIdentical(mut cls1: Arc<NFClass>, mut cls2: Arc<NFClass>) -> bool {
    let mut identical: bool = false;
    if referenceEq(&cls1.clone(),&cls2.clone()) {
        identical = true;
    } else {
        identical = (::match_deref::match_deref! { match &((cls1.clone(), cls2.clone())) {
        (Deref @ EXPANDED_CLASS { .. }, Deref @ EXPANDED_CLASS { .. }) => Prefixes::isEqual(var_field!((*cls1).prefixes, NFClass::EXPANDED_CLASS).clone(), var_field!((*cls2).prefixes, NFClass::EXPANDED_CLASS).clone()) && ClassTree::isIdentical(var_field!((*cls1).elements, NFClass::EXPANDED_CLASS).clone(), var_field!((*cls2).elements, NFClass::EXPANDED_CLASS).clone()),
        (Deref @ INSTANCED_BUILTIN { .. }, Deref @ INSTANCED_BUILTIN { .. }) => {
            if !(Type::isEqual(var_field!((*cls1).ty, NFClass::INSTANCED_BUILTIN).clone(), var_field!((*cls2).ty, NFClass::INSTANCED_BUILTIN).clone())) {
                return identical.clone();
            }
            true
        },
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    identical
}

pub fn hasDimensions(mut cls: Arc<NFClass>) -> Result<bool> {
    let mut hasDims: bool = false;
    hasDims = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ EXPANDED_DERIVED { .. } => (var_field!((*cls).dims, NFClass::EXPANDED_DERIVED).clone().borrow().len() as i32) > 0 || hasDimensions(InstNode::getClass(var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone())?)?,
        Deref @ TYPED_DERIVED { .. } => Type::isArray(var_field!((*cls).ty, NFClass::TYPED_DERIVED).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hasDims)
}

pub fn getDimensions(mut cls: Arc<NFClass>) -> Arc<metamodelica::List<Arc<Dimension::NFDimension>>> {
    let mut dims: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
    dims = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ INSTANCED_CLASS { .. } => Type::arrayDims(var_field!((*cls).ty, NFClass::INSTANCED_CLASS).clone()),
        Deref @ INSTANCED_BUILTIN { .. } => Type::arrayDims(var_field!((*cls).ty, NFClass::INSTANCED_BUILTIN).clone()),
        Deref @ TYPED_DERIVED { .. } => Type::arrayDims(var_field!((*cls).ty, NFClass::TYPED_DERIVED).clone()),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    dims
}

pub fn dimensionCount(mut cls: Arc<NFClass>) -> i32 {
    let mut count: i32 = 0;
    count = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ EXPANDED_DERIVED { .. } => (var_field!((*cls).dims, NFClass::EXPANDED_DERIVED).clone().borrow().len() as i32),
        Deref @ INSTANCED_CLASS { .. } => Type::dimensionCount(var_field!((*cls).ty, NFClass::INSTANCED_CLASS).clone()),
        Deref @ INSTANCED_BUILTIN { .. } => Type::dimensionCount(var_field!((*cls).ty, NFClass::INSTANCED_BUILTIN).clone()),
        Deref @ TYPED_DERIVED { .. } => Type::dimensionCount(var_field!((*cls).ty, NFClass::TYPED_DERIVED).clone()),
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    count
}

pub fn getAttributes(mut cls: Arc<NFClass>) -> Arc<Attributes::NFAttributes> {
    let mut attr: Arc<Attributes::NFAttributes> = Arc::new(<Attributes::NFAttributes as ::std::default::Default>::default());
    attr = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ EXPANDED_DERIVED { .. } => var_field!((*cls).attributes, NFClass::EXPANDED_DERIVED).clone(),
        _ => Attributes::DEFAULT_ATTR().clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    attr
}

pub fn getTypeAttributes(mut cls: Arc<NFClass>) -> Arc<metamodelica::List<Arc<Modifier::Modifier>>> {
    let mut attributes: Arc<metamodelica::List<Arc<Modifier::Modifier>>> = metamodelica::nil();
    let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut r#mod: Arc<Modifier::Modifier> = Arc::new(Modifier::NOMOD);
    if '__try0: {
        comps = unwrap_break_err!(ClassTree::getComponents(classTree(cls.clone()).unwrap()), '__try0);
        let __range1 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut c in __range1 {
            r#mod = Component::getModifier(InstNode::component(c.clone()).unwrap());
            if !(Modifier::isEmpty(r#mod.clone())) {
                attributes = cons(r#mod.clone(), attributes.clone());
            }
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    attributes
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getType(mut cls: Arc<NFClass>, mut clsNode: Arc<InstNode::InstNode>) -> Result<Arc<Type::NFType>> {
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    ty = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_BUILTIN { .. } => var_field!((*cls).ty, NFClass::PARTIAL_BUILTIN).clone(),
        Deref @ EXPANDED_DERIVED { .. } => getType(InstNode::getClass(var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone())?, var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone())?,
        Deref @ INSTANCED_CLASS { .. } => var_field!((*cls).ty, NFClass::INSTANCED_CLASS).clone(),
        Deref @ INSTANCED_BUILTIN { .. } => var_field!((*cls).ty, NFClass::INSTANCED_BUILTIN).clone(),
        Deref @ TYPED_DERIVED { .. } => var_field!((*cls).ty, NFClass::TYPED_DERIVED).clone(),
        _ => Arc::new(crate::NFType::UNKNOWN),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

pub fn setType(mut ty: Arc<Type::NFType>, mut cls: Arc<NFClass>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_BUILTIN; ty = ty.clone());
            ()
        },
        Deref @ EXPANDED_DERIVED { .. } => {
            InstNode::classApply(var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone(), (std::sync::Arc::new(setType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Type::NFType>, Arc<NFClass>) -> Result<Arc<NFClass>> + 'static>), ty.clone())?;
            ()
        },
        Deref @ INSTANCED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_CLASS; ty = ty.clone());
            ()
        },
        Deref @ INSTANCED_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_BUILTIN; ty = ty.clone());
            ()
        },
        Deref @ TYPED_DERIVED { .. } => {
            assign_variant_field!(cls => NFClass::TYPED_DERIVED; ty = ty.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cls)
}

pub fn restriction(mut cls: Arc<NFClass>) -> Arc<Restriction::NFRestriction> {
    let mut res: Arc<Restriction::NFRestriction> = Arc::new(Restriction::BLOCK);
    res = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_BUILTIN { .. } => var_field!((*cls).restriction, NFClass::PARTIAL_BUILTIN).clone(),
        Deref @ EXPANDED_CLASS { .. } => var_field!((*cls).restriction, NFClass::EXPANDED_CLASS).clone(),
        Deref @ EXPANDED_DERIVED { .. } => var_field!((*cls).restriction, NFClass::EXPANDED_DERIVED).clone(),
        Deref @ INSTANCED_CLASS { .. } => var_field!((*cls).restriction, NFClass::INSTANCED_CLASS).clone(),
        Deref @ INSTANCED_BUILTIN { .. } => var_field!((*cls).restriction, NFClass::INSTANCED_BUILTIN).clone(),
        Deref @ TYPED_DERIVED { .. } => var_field!((*cls).restriction, NFClass::TYPED_DERIVED).clone(),
        _ => Arc::new(crate::NFRestriction::UNKNOWN),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn setRestriction(mut res: Arc<Restriction::NFRestriction>, mut cls: Arc<NFClass>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ EXPANDED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_CLASS; restriction = res.clone());
            ()
        },
        Deref @ EXPANDED_DERIVED { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_DERIVED; restriction = res.clone());
            ()
        },
        Deref @ INSTANCED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_CLASS; restriction = res.clone());
            ()
        },
        Deref @ INSTANCED_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_BUILTIN; restriction = res.clone());
            ()
        },
        Deref @ TYPED_DERIVED { .. } => {
            assign_variant_field!(cls => NFClass::TYPED_DERIVED; restriction = res.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cls)
}

pub fn isConnectorClass(mut cls: Arc<NFClass>) -> bool {
    let mut isConnector: bool = Restriction::isConnector(restriction(cls.clone()));
    isConnector
}

pub fn isNonexpandableConnectorClass(mut cls: Arc<NFClass>) -> bool {
    let mut isConnector: bool = Restriction::isNonexpandableConnector(restriction(cls.clone()));
    isConnector
}

pub fn isExpandableConnectorClass(mut cls: Arc<NFClass>) -> bool {
    let mut isConnector: bool = Restriction::isExpandableConnector(restriction(cls.clone()));
    isConnector
}

pub fn isExternalObject(mut cls: Arc<NFClass>) -> bool {
    let mut isExternalObject: bool = Restriction::isExternalObject(restriction(cls.clone()));
    isExternalObject
}

pub fn isFunction(mut cls: Arc<NFClass>) -> bool {
    let mut isFunction: bool = Restriction::isFunction(restriction(cls.clone()));
    isFunction
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isEnumeration(mut cls: Arc<NFClass>) -> Result<bool> {
    let mut isEnum: bool = false;
    isEnum = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_BUILTIN { ty: Deref @ Type::ENUMERATION { .. }, .. } => true,
        Deref @ INSTANCED_BUILTIN { ty: Deref @ Type::ENUMERATION { .. }, .. } => true,
        Deref @ EXPANDED_DERIVED { .. } => isEnumeration(InstNode::getClass(var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone())?)?,
        Deref @ TYPED_DERIVED { .. } => isEnumeration(InstNode::getClass(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone())?)?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEnum)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn isExternalFunction(mut cls: Arc<NFClass>) -> Result<bool> {
    let mut isExtFunc: bool = false;
    isExtFunc = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ EXPANDED_DERIVED { .. } => {
            isExternalFunction(InstNode::getClass(var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone())?)?
        },
        Deref @ INSTANCED_CLASS { sections: Deref @ Sections::EXTERNAL { language: lang, .. }, .. } => {
            lang.clone() != literal!("builtin")
        },
        Deref @ TYPED_DERIVED { .. } => {
            isExternalFunction(InstNode::getClass(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone())?)?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isExtFunc)
}

pub fn isOverdetermined(mut cls: Arc<NFClass>) -> bool {
    let mut isOverdetermined: bool = false;
    match '__try0: {
        unwrap_break_err!(lookupElement((literal!("equalityConstraint")).clone(), cls.clone()), '__try0);
        System::setHasOverconstrainedConnectors(true);
        isOverdetermined = true;
        Ok::<_, anyhow::Error>((isOverdetermined.clone(),))
    } {
        Ok((__try0_o0,)) => {
            isOverdetermined = __try0_o0;
        }
        Err(_) => {
            isOverdetermined = false;
        }
    }
    isOverdetermined
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getPrefixes(mut cls: Arc<NFClass>) -> Result<Arc<Prefixes::Prefixes>> {
    let mut prefs: Arc<Prefixes::Prefixes> = Arc::new(<Prefixes::Prefixes as ::std::default::Default>::default());
    prefs = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => var_field!((*cls).prefixes, NFClass::PARTIAL_CLASS).clone(),
        Deref @ PARTIAL_BUILTIN { .. } => var_field!((*cls).prefixes, NFClass::PARTIAL_BUILTIN).clone(),
        Deref @ EXPANDED_CLASS { .. } => var_field!((*cls).prefixes, NFClass::EXPANDED_CLASS).clone(),
        Deref @ EXPANDED_DERIVED { .. } => var_field!((*cls).prefixes, NFClass::EXPANDED_DERIVED).clone(),
        Deref @ INSTANCED_CLASS { .. } => var_field!((*cls).prefixes, NFClass::INSTANCED_CLASS).clone(),
        Deref @ TYPED_DERIVED { .. } => getPrefixes(InstNode::getClass(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone())?)?,
        _ => DEFAULT_PREFIXES.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(prefs)
}

pub fn setPrefixes(mut prefs: Arc<Prefixes::Prefixes>, mut cls: Arc<NFClass>) -> Result<Arc<NFClass>> {
    let mut cls: Arc<NFClass> = cls;
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ PARTIAL_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_CLASS; prefixes = prefs.clone());
            ()
        },
        Deref @ PARTIAL_BUILTIN { .. } => {
            assign_variant_field!(cls => NFClass::PARTIAL_BUILTIN; prefixes = prefs.clone());
            ()
        },
        Deref @ EXPANDED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_CLASS; prefixes = prefs.clone());
            ()
        },
        Deref @ EXPANDED_DERIVED { .. } => {
            assign_variant_field!(cls => NFClass::EXPANDED_DERIVED; prefixes = prefs.clone());
            ()
        },
        Deref @ INSTANCED_CLASS { .. } => {
            assign_variant_field!(cls => NFClass::INSTANCED_CLASS; prefixes = prefs.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cls)
}

pub fn isEncapsulated(mut cls: Arc<NFClass>) -> bool {
    let mut isEncapsulated: bool = Prefixes::isEncapsulated(getPrefixes(cls.clone()).unwrap());
    isEncapsulated
}

pub fn isPartial(mut cls: Arc<NFClass>) -> bool {
    let mut isPartial: bool = Prefixes::isPartial(getPrefixes(cls.clone()).unwrap());
    isPartial
}

pub fn lastBaseClass(mut node: Arc<InstNode::InstNode>) -> Arc<InstNode::InstNode> {
    let mut node: Arc<InstNode::InstNode> = node;
    let mut cls: Arc<NFClass> = InstNode::getClass(node.clone()).unwrap();
    node = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ EXPANDED_DERIVED { .. } => lastBaseClass(var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone()),
        Deref @ TYPED_DERIVED { .. } => lastBaseClass(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone()),
        _ => node.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    node
}

pub fn getDerivedComments(mut cls: Arc<NFClass>, mut cmts: Arc<metamodelica::List<Arc<SCode::Comment>>>) -> Arc<metamodelica::List<Arc<SCode::Comment>>> {
    let mut cmts: Arc<metamodelica::List<Arc<SCode::Comment>>> = cmts;
    cmts = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ EXPANDED_DERIVED { .. } => InstNode::getComments(var_field!((*cls).baseClass, NFClass::EXPANDED_DERIVED).clone(), cmts.clone()),
        Deref @ TYPED_DERIVED { .. } => InstNode::getComments(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone(), cmts.clone()),
        _ => {
            let __range0 = ClassTree::getExtends(classTree(cls.clone()).unwrap()).borrow().iter().cloned().collect::<Vec<_>>();
            for mut ext in __range0 {
                cmts = InstNode::getComments(ext.clone(), cmts.clone());
            }
            cmts.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cmts
}

pub fn constrainingClassPath(mut clsNode: Arc<InstNode::InstNode>) -> Result<Arc<Absyn::Path>> {
    let mut path: Arc<Absyn::Path>;
    let mut cls_node: Arc<InstNode::InstNode> = lastBaseClass(clsNode.clone());
    let mut prefs: Arc<Prefixes::Prefixes> = getPrefixes(InstNode::getClass(cls_node.clone())?)?;
    path = (::match_deref::match_deref! { match &(prefs.clone()) {
        Deref @ Prefixes::PREFIXES { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { constrainingClass: path, .. }) }, .. } => path.clone(),
        _ => InstNode::enclosingScopePath(cls_node.clone(), false, false)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(path)
}

pub fn hasOperator(mut name: ArcStr, mut cls: Arc<NFClass>) -> bool {
    let mut hasOperator: bool = false;
    let mut op_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    if Restriction::isOperatorRecord(restriction(cls.clone())) {
        match '__try0: {
            (op_node, _) = unwrap_break_err!(lookupElement((name.clone()).clone(), cls.clone()), '__try0);
            hasOperator = SCodeUtil::isOperator(InstNode::definition(op_node.clone()).unwrap());
            Ok::<_, anyhow::Error>((hasOperator.clone(),))
        } {
            Ok((__try0_o0,)) => {
                hasOperator = __try0_o0;
            }
            Err(_) => {
                hasOperator = false;
            }
        }
    } else {
        hasOperator = false;
    }
    hasOperator
}

pub fn makeRecordExp(mut clsNode: Arc<InstNode::InstNode>, mut scope: Arc<InstNode::InstNode>, mut typed: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut cls: Arc<NFClass> = Arc::new(NFClass::NOT_INSTANTIATED);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut fields: Arc<metamodelica::List<Arc<Record::Field::Field>>> = metamodelica::nil();
    let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    cls = InstNode::getClass(clsNode.clone())?;
    let (__pa1, __pa0) = ::match_deref::match_deref! { match &(getType(cls.clone(), clsNode.clone())?) {
        __pa1 @ Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: __pa0, .. }, .. } => (__pa1.clone(), __pa0.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty_node = __pa0.clone();
    ty = __pa1.clone();
    comps = ClassTree::getComponents(classTree(cls.clone())?)?;
    if typed.clone() {
        args = ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut c in (comps.clone()).borrow().iter() {
            let __x = Binding::getExp(Component::getImplicitBinding(InstNode::component(c.clone())?, scope.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        exp = Expression::makeRecord(InstNode::fullPath(ty_node.clone(), false), ty.clone(), args.clone());
    } else {
        args = metamodelica::nil();
        let __range3 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut c in __range3 {
            fields = Record::collectRecordField(c.clone(), metamodelica::nil())?;
            if !(fields.clone().is_empty()) && Record::Field::isInput(listHead(fields.clone())?) {
                args = cons(Binding::getExp(Component::getImplicitBinding(InstNode::component(c.clone())?, scope.clone()))?, args.clone());
            }
        }
        args = args.clone().reverse();
        exp = Arc::new(Expression::NFExpression::CALL { call: Arc::new(Call::NFCall::UNTYPED_CALL { r#ref: ComponentRef::fromNode(ty_node.clone(), ty.clone(), metamodelica::nil(), ComponentRef::Origin::CREF.clone()), arguments: args.clone(), named_args: metamodelica::nil(), call_scope: scope.clone() }) });
    }
    Ok(exp)
}

pub fn toFlatStream(mut cls: Arc<NFClass>, mut clsNode: Arc<InstNode::InstNode>, mut format: BaseModelica::OutputFormat, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut name: ArcStr = arcstr::literal!("");
    name = (Util::makeQuotedIdentifier((AbsynUtil::pathString(InstNode::scopePath(clsNode.clone(), InstNode::ScopeType::RELATIVE.clone(), false)?, (literal!(".")).clone(), true, false)?).clone())?).clone();
    s = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ INSTANCED_CLASS { .. } => {
            s = IOStream::append(s.clone(), (indent.clone()).clone())?;
            s = IOStream::append(s.clone(), (Restriction::toString(var_field!((*cls).restriction, NFClass::INSTANCED_CLASS).clone())).clone())?;
            s = IOStream::append(s.clone(), (literal!(" ")).clone())?;
            s = IOStream::append(s.clone(), (name.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!("\n")).clone())?;
            let __range0 = ClassTree::getComponents(var_field!((*cls).elements, NFClass::INSTANCED_CLASS).clone())?.borrow().iter().cloned().collect::<Vec<_>>();
            for mut comp in __range0 {
                s = IOStream::append(s.clone(), (InstNode::toFlatString(comp.clone(), format.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone())?).clone())?;
                s = IOStream::append(s.clone(), (literal!(";\n")).clone())?;
            }
            s = IOStream::append(s.clone(), (indent.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!("end ")).clone())?;
            s = IOStream::append(s.clone(), (name.clone()).clone())?;
            s.clone()
        },
        Deref @ INSTANCED_BUILTIN { .. } => {
            s = IOStream::append(s.clone(), (indent.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!("INSTANCED_BUILTIN(")).clone())?;
            s = IOStream::append(s.clone(), (name.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!(")")).clone())?;
            s.clone()
        },
        Deref @ TYPED_DERIVED { .. } => {
            s = IOStream::append(s.clone(), (indent.clone()).clone())?;
            s = IOStream::append(s.clone(), (Restriction::toString(var_field!((*cls).restriction, NFClass::TYPED_DERIVED).clone())).clone())?;
            s = IOStream::append(s.clone(), (literal!(" ")).clone())?;
            s = IOStream::append(s.clone(), (name.clone()).clone())?;
            s = IOStream::append(s.clone(), (literal!(" = ")).clone())?;
            s = IOStream::append(s.clone(), (Util::makeQuotedIdentifier((AbsynUtil::pathString(InstNode::scopePath(var_field!((*cls).baseClass, NFClass::TYPED_DERIVED).clone(), InstNode::ScopeType::RELATIVE.clone(), false)?, (literal!(".")).clone(), true, false)?).clone())?).clone())?;
            s.clone()
        },
        _ => IOStream::append(s.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("UNKNOWN_CLASS(")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(s)
}

pub fn toFlatString(mut cls: Arc<NFClass>, mut clsNode: Arc<InstNode::InstNode>, mut format: BaseModelica::OutputFormat, mut indent: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut s: IOStream::IOStream = <IOStream::IOStream as ::std::default::Default>::default();
    s = IOStream::create((literal!("NFClass.toFlatString")).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
    s = toFlatStream(cls.clone(), clsNode.clone(), format.clone(), (indent.clone()).clone(), s.clone())?;
    r#str = (IOStream::string(s.clone())?).clone();
    IOStream::delete(s.clone())?;
    Ok(r#str)
}


