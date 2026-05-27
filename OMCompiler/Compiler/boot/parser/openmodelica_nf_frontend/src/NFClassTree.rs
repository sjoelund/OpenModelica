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

use crate::NFBuiltin;
use crate::NFClass as Class;
use crate::NFComponent as Component;
use crate::NFDuplicateTree as DuplicateTree;
use crate::NFImport as Import;
use crate::NFInst as Inst;
use crate::NFInstContext;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFLookup as Lookup;
use crate::NFModifier::Modifier;
use crate::NFRestriction as Restriction;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::NFLookupTree as LookupTree;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

thread_local! { static __EMPTY_TLS: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::ClassTree::PARTIAL_TREE { tree: Arc::new(openmodelica_util::NFLookupTree::Tree::EMPTY), classes: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), components: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), exts: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), imports: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), duplicates: Arc::new(crate::NFDuplicateTree::Tree::EMPTY) }); }
pub fn EMPTY() -> Arc<ClassTree::ClassTree> { __EMPTY_TLS.with(|__t| __t.clone()) }

thread_local! { static __EMPTY_FLAT_TLS: Arc<ClassTree::ClassTree> = Arc::new(ClassTree::ClassTree::FLAT_TREE { tree: Arc::new(openmodelica_util::NFLookupTree::Tree::EMPTY), classes: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), components: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), imports: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), duplicates: Arc::new(crate::NFDuplicateTree::Tree::EMPTY) }); }
pub fn EMPTY_FLAT() -> Arc<ClassTree::ClassTree> { __EMPTY_FLAT_TLS.with(|__t| __t.clone()) }

pub type LookupEntry = Arc<LookupTree::Entry::Entry>;

pub type LookupTable = Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<LookupTree::Entry::Entry>>>;

pub mod ClassTree {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum ClassTree {
        /// A partial tree allows lookup of local classes and imported elements.
        PARTIAL_TREE {
            tree: Arc<LookupTree::Tree>,
            classes: metamodelica::Array<Arc<InstNode::InstNode>>,
            components: metamodelica::Array<Arc<InstNode::InstNode>>,
            exts: metamodelica::Array<Arc<InstNode::InstNode>>,
            imports: metamodelica::Array<Arc<Import::NFImport>>,
            duplicates: Arc<DuplicateTree::Tree>,
        },
        /// Like partial tree, but the lookup tree is populated with all named
        ///       elements. The elements have not yet been added to the arrays though, so
        ///       lookup is still restricted to local classes and imported elements.
        EXPANDED_TREE {
            tree: Arc<LookupTree::Tree>,
            classes: metamodelica::Array<Arc<InstNode::InstNode>>,
            components: metamodelica::Array<Arc<InstNode::InstNode>>,
            exts: metamodelica::Array<Arc<InstNode::InstNode>>,
            imports: metamodelica::Array<Arc<Import::NFImport>>,
            duplicates: Arc<DuplicateTree::Tree>,
        },
        /// Allows lookup of both local and inherited elements.
        INSTANTIATED_TREE {
            tree: Arc<LookupTree::Tree>,
            classes: metamodelica::Array<Mutable::Mutable<Arc<InstNode::InstNode>>>,
            components: metamodelica::Array<Mutable::Mutable<Arc<InstNode::InstNode>>>,
            localComponents: Arc<metamodelica::List<i32>>,
            exts: metamodelica::Array<Arc<InstNode::InstNode>>,
            imports: metamodelica::Array<Arc<Import::NFImport>>,
            duplicates: Arc<DuplicateTree::Tree>,
        },
        /// A flattened version of an instantiated tree.
        FLAT_TREE {
            tree: Arc<LookupTree::Tree>,
            classes: metamodelica::Array<Arc<InstNode::InstNode>>,
            components: metamodelica::Array<Arc<InstNode::InstNode>>,
            imports: metamodelica::Array<Arc<Import::NFImport>>,
            duplicates: Arc<DuplicateTree::Tree>,
        },
        EMPTY_TREE,
    }
    pub use self::ClassTree::{PARTIAL_TREE,EXPANDED_TREE,INSTANTIATED_TREE,FLAT_TREE,EMPTY_TREE};
    pub fn fromSCode(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut isClassExtends: bool, mut parent: Arc<InstNode::InstNode>) -> Result<Arc<ClassTree>> {
        let mut tree: Arc<ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
        let mut ltree: Arc<LookupTree::Tree> = Arc::new(LookupTree::Tree::EMPTY);
        let mut lentry: Arc<LookupTree::Entry::Entry>;
        let mut clsc: i32 = 0;
        let mut compc: i32 = 0;
        let mut extc: i32 = 0;
        let mut i: i32 = 0;
        let mut clss: metamodelica::Array<Arc<InstNode::InstNode>>;
        let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
        let mut exts: metamodelica::Array<Arc<InstNode::InstNode>>;
        let mut cls_idx: i32 = 0;
        let mut ext_idx: i32 = 0;
        let mut comp_idx: i32 = 0;
        let mut dups: Arc<DuplicateTree::Tree> = Arc::new(DuplicateTree::Tree::EMPTY);
        let mut imps: Arc<metamodelica::List<Arc<Import::NFImport>>> = metamodelica::nil();
        let mut imps_arr: metamodelica::Array<Arc<Import::NFImport>>;
        let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
        ltree = LookupTree::new();
        (clsc, compc, extc) = countElements(elements.clone());
        if isClassExtends.clone() {
            extc = extc.clone() + 1;
        }
        clss = metamodelica::arrayCreate(clsc.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE));
        comps = metamodelica::arrayCreate(compc.clone() + extc.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE));
        exts = metamodelica::arrayCreate(extc.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE));
        dups = DuplicateTree::new();
        tree = Arc::new(ClassTree::PARTIAL_TREE { tree: ltree.clone(), classes: clss.clone(), components: comps.clone(), exts: exts.clone(), imports: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), duplicates: dups.clone() });
        if isClassExtends.clone() {
            {
                let __cell0 = Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE);
                unsafe { metamodelica::Dangerous::arrayInitSlot(exts.clone().clone(), 1, __cell0); }
            }
            {
                let __cell1 = Arc::new(InstNode::InstNode::REF_NODE { index: 1 });
                unsafe { metamodelica::Dangerous::arrayInitSlot(comps.clone().clone(), 1, __cell1); }
            }
            ext_idx = ext_idx.clone() + 1;
            comp_idx = comp_idx.clone() + 1;
        }
        for mut e in &*elements.clone() {
            let mut e = e.clone();
            let () = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ SCode::Element::CLASS { .. } => {
            cls_idx = cls_idx.clone() + 1;
            unsafe { metamodelica::Dangerous::arrayInitSlot(clss.clone(), cls_idx.clone(), InstNode::newClass(e.clone(), parent.clone(), Arc::new(crate::NFInstNode::InstNodeType::NORMAL_CLASS))?) };
            lentry = Arc::new(LookupTree::Entry::Entry::CLASS { index: cls_idx.clone() });
            ltree = addLocalElement((var_field!((*e).name, SCode::Element::CLASS).clone()).clone(), lentry.clone(), tree.clone(), ltree.clone())?;
            if SCodeUtil::isElementRedeclare(e.clone())? || SCodeUtil::isClassExtends(e.clone()) {
                dups = DuplicateTree::add(dups.clone(), (var_field!((*e).name, SCode::Element::CLASS).clone()).clone(), DuplicateTree::newRedeclare(lentry.clone()), (std::sync::Arc::new(DuplicateTree::addConflictDefault) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
            }
            ()
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            comp_idx = comp_idx.clone() + 1;
            unsafe { metamodelica::Dangerous::arrayInitSlot(comps.clone(), comp_idx.clone(), InstNode::newComponent(e.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE))?) };
            ()
        },
        Deref @ SCode::Element::EXTENDS { .. } => {
            ext_idx = ext_idx.clone() + 1;
            unsafe { metamodelica::Dangerous::arrayInitSlot(exts.clone(), ext_idx.clone(), InstNode::newExtends(e.clone(), parent.clone())?) };
            comp_idx = comp_idx.clone() + 1;
            unsafe { metamodelica::Dangerous::arrayInitSlot(comps.clone(), comp_idx.clone(), Arc::new(InstNode::InstNode::REF_NODE { index: ext_idx.clone() })) };
            ()
        },
        Deref @ SCode::Element::IMPORT { .. } => {
            imps = cons(Arc::new(Import::NFImport::UNRESOLVED_IMPORT { imp: var_field!((*e).imp, SCode::Element::IMPORT).clone(), scope: parent.clone(), info: var_field!((*e).info, SCode::Element::IMPORT).clone() }), imps.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
        }
        tree = Arc::new(ClassTree::PARTIAL_TREE { tree: ltree.clone(), classes: clss.clone(), components: comps.clone(), exts: exts.clone(), imports: metamodelica::arrayFromVec(imps.clone().into_iter().cloned().collect()), duplicates: dups.clone() });
        Ok(tree)
    }

    pub fn initImports(mut tree: Arc<ClassTree>, mut parent: Arc<InstNode::InstNode>) -> Result<Arc<ClassTree>> {
        let mut tree: Arc<ClassTree> = tree;
        let mut imports: metamodelica::Array<Arc<Import::NFImport>>;
        let mut init_imports: Arc<metamodelica::List<Arc<Import::NFImport>>> = metamodelica::nil();
        let mut imp: Arc<Import::NFImport>;
        let mut ltree: Arc<LookupTree::Tree> = Arc::new(LookupTree::Tree::EMPTY);
        let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ PARTIAL_TREE { imports, tree: ltree, .. } if (!(imports.clone().borrow().is_empty())) => {
            let mut imports = (*imports).clone();
            let mut ltree = (*ltree).clone();
            init_imports = metamodelica::nil();
            let __range0 = imports.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut imp in __range0 {
                init_imports = (::match_deref::match_deref! { match &(imp.clone()) {
        Deref @ Import::UNRESOLVED_IMPORT { imp: Absyn::Import::UNQUAL_IMPORT { .. }, .. } => Import::instUnqualified(imp.clone(), init_imports.clone())?,
        _ => cons(imp.clone(), init_imports.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            imports = metamodelica::arrayFromVec(init_imports.clone().into_iter().cloned().collect());
            let __range1 = (1..=(imports.clone().borrow().len() as i32)).rev();
            for mut i in __range1 {
                ltree = addImport(imports.borrow()[(i.clone()-1) as usize].clone(), i.clone(), ltree.clone(), imports.clone())?;
            }
            assign_variant_field!(tree => ClassTree::PARTIAL_TREE;
                imports = imports.clone(),
                tree = ltree.clone()
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(tree)
    }

    pub fn fromEnumeration(mut literals: Arc<metamodelica::List<Arc<SCode::Enum>>>, mut enumType: Arc<Type::NFType>, mut enumClass: Arc<InstNode::InstNode>) -> Result<Arc<ClassTree>> {
        let mut tree: Arc<ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
        let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
        let mut attr_count: i32 = 5;
        let mut i: i32 = 0;
        let mut comp: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut ltree: Arc<LookupTree::Tree> = Arc::new(LookupTree::Tree::EMPTY);
        let mut name: ArcStr = arcstr::literal!("");
        comps = metamodelica::arrayCreate((literals.clone().len() as i32) + attr_count.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE));
        ltree = NFBuiltin::ENUM_LOOKUP_TREE.clone();
        unsafe { metamodelica::Dangerous::arrayInitSlot(comps.clone(), 1, InstNode::fromComponent((literal!("quantity")).clone(), Arc::new(Component::NFComponent::TYPE_ATTRIBUTE { ty: Arc::new(crate::NFType::STRING), modifier: Arc::new(crate::NFModifier::Modifier::NOMOD) }), enumClass.clone())) };
        unsafe { metamodelica::Dangerous::arrayInitSlot(comps.clone(), 2, InstNode::fromComponent((literal!("min")).clone(), Arc::new(Component::NFComponent::TYPE_ATTRIBUTE { ty: enumType.clone(), modifier: Arc::new(crate::NFModifier::Modifier::NOMOD) }), enumClass.clone())) };
        unsafe { metamodelica::Dangerous::arrayInitSlot(comps.clone(), 3, InstNode::fromComponent((literal!("max")).clone(), Arc::new(Component::NFComponent::TYPE_ATTRIBUTE { ty: enumType.clone(), modifier: Arc::new(crate::NFModifier::Modifier::NOMOD) }), enumClass.clone())) };
        unsafe { metamodelica::Dangerous::arrayInitSlot(comps.clone(), 4, InstNode::fromComponent((literal!("start")).clone(), Arc::new(Component::NFComponent::TYPE_ATTRIBUTE { ty: enumType.clone(), modifier: Arc::new(crate::NFModifier::Modifier::NOMOD) }), enumClass.clone())) };
        unsafe { metamodelica::Dangerous::arrayInitSlot(comps.clone(), 5, InstNode::fromComponent((literal!("fixed")).clone(), Arc::new(Component::NFComponent::TYPE_ATTRIBUTE { ty: Arc::new(crate::NFType::BOOLEAN), modifier: Arc::new(crate::NFModifier::Modifier::NOMOD) }), enumClass.clone())) };
        for mut l in &*literals.clone() {
            let mut l = l.clone();
            name = (l.literal.clone()).clone();
            i = i.clone() + 1;
            comp = InstNode::fromComponent((name.clone()).clone(), Component::newEnum(enumType.clone(), (name.clone()).clone(), l.comment.clone(), i.clone()), enumClass.clone());
            unsafe { metamodelica::Dangerous::arrayInitSlot(comps.clone(), i.clone() + attr_count.clone(), comp.clone()) };
            ltree = LookupTree::add(ltree.clone(), (name.clone()).clone(), Arc::new(LookupTree::Entry::Entry::COMPONENT { index: i.clone() + attr_count.clone() }), Arc::new({ let __pe_b3 = comp.clone(); move |__pe_a0, __pe_a1, __pe_a2| addEnumConflict(__pe_a0, __pe_a1, __pe_a2, __pe_b3.clone()) }))?;
        }
        tree = Arc::new(ClassTree::FLAT_TREE { tree: ltree.clone(), classes: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), components: comps.clone(), imports: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), duplicates: Arc::new(crate::NFDuplicateTree::Tree::EMPTY) });
        Ok(tree)
    }

    pub fn addElementsToFlatTree(mut elements: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut tree: Arc<ClassTree>) -> Result<Arc<ClassTree>> {
        let mut tree: Arc<ClassTree> = tree;
        let mut ltree: Arc<LookupTree::Tree> = Arc::new(LookupTree::Tree::EMPTY);
        let mut cls_arr: metamodelica::Array<Arc<InstNode::InstNode>>;
        let mut comp_arr: metamodelica::Array<Arc<InstNode::InstNode>>;
        let mut cls_lst: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
        let mut comp_lst: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
        let mut imports: metamodelica::Array<Arc<Import::NFImport>>;
        let mut duplicates: Arc<DuplicateTree::Tree> = Arc::new(DuplicateTree::Tree::EMPTY);
        let mut cls_idx: i32 = 0;
        let mut comp_idx: i32 = 0;
        let mut lentry: Arc<LookupTree::Entry::Entry>;
        let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(tree.clone()) {
            Deref @ FLAT_TREE { tree: __pa0, classes: __pa1, components: __pa2, imports: __pa3, duplicates: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ltree = __pa0.clone();
        cls_arr = __pa1.clone();
        comp_arr = __pa2.clone();
        imports = __pa3.clone();
        duplicates = __pa4.clone();
        cls_idx = (cls_arr.clone().borrow().len() as i32);
        comp_idx = (comp_arr.clone().borrow().len() as i32);
        for mut e in &*elements.clone() {
            let mut e = e.clone();
            if InstNode::isComponent(e.clone()) {
                comp_idx = comp_idx.clone() + 1;
                lentry = Arc::new(LookupTree::Entry::Entry::COMPONENT { index: comp_idx.clone() });
                comp_lst = cons(e.clone(), comp_lst.clone());
            } else {
                cls_idx = cls_idx.clone() + 1;
                lentry = Arc::new(LookupTree::Entry::Entry::CLASS { index: cls_idx.clone() });
                cls_lst = cons(e.clone(), cls_lst.clone());
            }
            ltree = addLocalElement((InstNode::name(e.clone())?).clone(), lentry.clone(), tree.clone(), ltree.clone())?;
        }
        cls_arr = Array::appendList(cls_arr.clone(), cls_lst.clone().reverse())?;
        comp_arr = Array::appendList(comp_arr.clone(), comp_lst.clone().reverse())?;
        tree = Arc::new(ClassTree::FLAT_TREE { tree: ltree.clone(), classes: cls_arr.clone(), components: comp_arr.clone(), imports: imports.clone(), duplicates: duplicates.clone() });
        Ok(tree)
    }

    pub fn expand(mut tree: Arc<ClassTree>) -> Result<Arc<ClassTree>> {
        let mut tree: Arc<ClassTree> = tree;
        let mut ltree: Arc<LookupTree::Tree> = Arc::new(LookupTree::Tree::EMPTY);
        let mut lentry: Arc<LookupTree::Entry::Entry>;
        let mut exts: metamodelica::Array<Arc<InstNode::InstNode>>;
        let mut clss: metamodelica::Array<Arc<InstNode::InstNode>>;
        let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
        let mut imps: metamodelica::Array<Arc<Import::NFImport>>;
        let mut ext_idxs: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
        let mut ccount: i32 = 0;
        let mut cls_idx: i32 = 0;
        let mut comp_idx: i32 = 1;
        let mut dups: Arc<DuplicateTree::Tree> = Arc::new(DuplicateTree::Tree::EMPTY);
        let mut dups_ptr: Mutable::Mutable<Arc<DuplicateTree::Tree>>;
        let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(tree.clone()) {
            Deref @ PARTIAL_TREE { tree: __pa0, classes: __pa1, components: __pa2, exts: __pa3, imports: __pa4, duplicates: __pa5 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ltree = __pa0.clone();
        clss = __pa1.clone();
        comps = __pa2.clone();
        exts = __pa3.clone();
        imps = __pa4.clone();
        dups = __pa5.clone();
        cls_idx = (clss.clone().borrow().len() as i32) + 1;
        let __range6 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut c in __range6 {
            let () = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ InstNode::COMPONENT_NODE { .. } => {
            lentry = Arc::new(LookupTree::Entry::Entry::COMPONENT { index: comp_idx.clone() });
            ltree = addLocalElement((InstNode::name(c.clone())?).clone(), lentry.clone(), tree.clone(), ltree.clone())?;
            if InstNode::isRedeclare(c.clone())? {
                dups = DuplicateTree::add(dups.clone(), (var_field!((*c).name, InstNode::InstNode::COMPONENT_NODE).clone()).clone(), DuplicateTree::newRedeclare(lentry.clone()), (std::sync::Arc::new(DuplicateTree::addConflictDefault) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
            }
            comp_idx = comp_idx.clone() + 1;
            ()
        },
        Deref @ InstNode::REF_NODE { .. } => {
            ext_idxs = cons((cls_idx.clone() - 1, comp_idx.clone() - 1), ext_idxs.clone());
            (cls_idx, comp_idx) = countInheritedElements(exts.borrow()[(var_field!((*c).index, InstNode::InstNode::REF_NODE).clone()-1) as usize].clone(), cls_idx.clone(), comp_idx.clone())?;
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFClassTree.ClassTree.expand")); __mm_s.push_str(&*literal!(" got invalid component")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        dups_ptr = Mutable::create(dups.clone());
        if !(ext_idxs.clone().is_empty()) {
            ext_idxs = ext_idxs.clone().reverse();
            let __range7 = exts.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut ext in __range7 {
                let (__pa8, __pa9, __pa10) = ::match_deref::match_deref! { match &(ext_idxs.clone()) {
                    Deref @ metamodelica::List::Cons { head: (__pa8, __pa9), tail: __pa10 } => (__pa8.clone(), __pa9.clone(), __pa10.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                cls_idx = __pa8.clone();
                comp_idx = __pa9.clone();
                ext_idxs = __pa10.clone();
                ltree = expandExtends(ext.clone(), ltree.clone(), cls_idx.clone(), comp_idx.clone(), dups_ptr.clone())?;
            }
        }
        tree = Arc::new(ClassTree::EXPANDED_TREE { tree: ltree.clone(), classes: clss.clone(), components: comps.clone(), exts: exts.clone(), imports: imps.clone(), duplicates: Mutable::access(dups_ptr.clone()) });
        Ok(tree)
    }

    pub fn instantiate(mut clsNode: Arc<InstNode::InstNode>, mut instance: Arc<InstNode::InstNode>, mut scope: Arc<InstNode::InstNode>) -> Result<(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>, i32, i32)> {
        let mut clsNode: Arc<InstNode::InstNode> = clsNode;
        let mut instance: Arc<InstNode::InstNode> = instance;
        let mut classCount: i32 = 0;
        let mut compCount: i32 = 0;
        let mut cls: Arc<Class::NFClass> = Arc::new(Class::NOT_INSTANTIATED);
        let mut tree: Arc<ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
        let mut ext_tree: Arc<ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
        let mut ltree: Arc<LookupTree::Tree> = Arc::new(LookupTree::Tree::EMPTY);
        let mut exts: metamodelica::Array<Arc<InstNode::InstNode>>;
        let mut old_clss: metamodelica::Array<Arc<InstNode::InstNode>>;
        let mut old_comps: metamodelica::Array<Arc<InstNode::InstNode>>;
        let mut imps: metamodelica::Array<Arc<Import::NFImport>>;
        let mut clss: metamodelica::Array<Mutable::Mutable<Arc<InstNode::InstNode>>>;
        let mut comps: metamodelica::Array<Mutable::Mutable<Arc<InstNode::InstNode>>>;
        let mut ext_clss: metamodelica::Array<Mutable::Mutable<Arc<InstNode::InstNode>>>;
        let mut local_comps: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut break_indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut cls_idx: i32 = 1;
        let mut comp_idx: i32 = 1;
        let mut cls_count: i32 = 0;
        let mut comp_count: i32 = 0;
        let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut parent_scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut inner_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut inst_scope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut dups: Arc<DuplicateTree::Tree> = Arc::new(DuplicateTree::Tree::EMPTY);
        let mut comp: Arc<Component::NFComponent> = Arc::new(Component::WILD);
        let mut ext_def: Arc<SCode::Element>;
        let mut is_typish: bool = false;
        let mut inst_ty: Arc<InstNodeType> = Arc::new(InstNodeType::BUILTIN_CLASS);
        cls = InstNode::getClass(clsNode.clone())?;
        clsNode = InstNode::replaceClass(cls.clone(), clsNode.clone())?;
        let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Class::EXPANDED_CLASS { elements: Deref @ INSTANTIATED_TREE { .. }, .. } => (),
        Deref @ Class::EXPANDED_CLASS { .. } => {
            if InstNode::isEmpty(instance.clone()) {
                instance = clsNode.clone();
                parent_scope = InstNode::instanceParent(clsNode.clone());
            } else {
                parent_scope = instance.clone();
                inst_scope = scope.clone();
            }
            inst_scope = if (InstNode::isEmpty(scope.clone())) {instance.clone()} else {scope.clone()};
            let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(var_field!((*cls).elements, Class::NFClass::EXPANDED_CLASS).clone()) {
                Deref @ EXPANDED_TREE { tree: __pa0, classes: __pa1, components: __pa2, exts: __pa3, imports: __pa4, duplicates: __pa5 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ltree = __pa0.clone();
            old_clss = __pa1.clone();
            old_comps = __pa2.clone();
            exts = __pa3.clone();
            imps = __pa4.clone();
            dups = __pa5.clone();
            classCount = (old_clss.clone().borrow().len() as i32);
            compCount = (old_comps.clone().borrow().len() as i32) - (exts.clone().borrow().len() as i32);
            exts = metamodelica::arrayFromVec(exts.clone().borrow().clone());
            let __range6 = 1..=(exts.clone().borrow().len() as i32);
            for mut i in __range6 {
                node = exts.borrow()[(i.clone()-1) as usize].clone();
                let (__pa7, __pa8) = ::match_deref::match_deref! { match &(InstNode::nodeType(node.clone())?) {
                    Deref @ InstNodeType::BASE_CLASS { ty: __pa7, definition: __pa8, .. } => (__pa7.clone(), __pa8.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                inst_ty = __pa7.clone();
                ext_def = __pa8.clone();
                node = InstNode::setNodeType(Arc::new(InstNodeType::BASE_CLASS { parent: instance.clone(), definition: ext_def.clone(), ty: inst_ty.clone() }), node.clone());
                (node, _, cls_count, comp_count) = instantiate(node.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE), inst_scope.clone())?;
                {
                    let __cell9 = node.clone();
                    exts.clone().borrow_mut()[(i.clone()-1) as usize] = __cell9;
                }
                classCount = cls_count.clone() + classCount.clone();
                compCount = comp_count.clone() + compCount.clone();
            }
            comps = metamodelica::arrayCreate(compCount.clone(), Mutable::create(Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE)));
            clss = metamodelica::arrayCreate(classCount.clone(), Mutable::create(Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE)));
            is_typish = Restriction::isType(var_field!((*cls).restriction, Class::NFClass::EXPANDED_CLASS).clone()) || Restriction::isOperatorRecord(var_field!((*cls).restriction, Class::NFClass::EXPANDED_CLASS).clone()) || Restriction::isOperator(var_field!((*cls).restriction, Class::NFClass::EXPANDED_CLASS).clone());
            let __range10 = old_clss.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range10 {
                if is_typish.clone() {
                    c = InstNode::setParent(clsNode.clone(), c.clone())?;
                } else {
                    c = InstNode::clone(c.clone())?;
                    c = InstNode::setParent(instance.clone(), c.clone())?;
                }
                if InstNode::isOuter(c.clone())? {
                    checkOuterClass(c.clone())?;
                    c = linkInnerOuter(c.clone(), parent_scope.clone())?;
                }
                unsafe { metamodelica::Dangerous::arrayInitSlot(clss.clone(), cls_idx.clone(), Mutable::create(c.clone())) };
                cls_idx = cls_idx.clone() + 1;
            }
            let __range11 = exts.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut ext in __range11 {
                let () = (::match_deref::match_deref! { match &(Class::classTree(InstNode::getClass(ext.clone())?)?) {
        Deref @ INSTANTIATED_TREE { classes: ext_clss, .. } => {
            cls_count = (ext_clss.clone().borrow().len() as i32);
            if cls_count.clone() > 0 {
                Array::copyRange(ext_clss.clone(), clss.clone(), 1, cls_count.clone(), cls_idx.clone())?;
                cls_idx = cls_idx.clone() + cls_count.clone();
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            let __range12 = old_comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range12 {
                let () = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ InstNode::COMPONENT_NODE { .. } => {
            node = InstNode::cloneComponent(c.clone(), instance.clone())?;
            if InstNode::isOuter(node.clone())? {
                if '__try0: {
                    node = unwrap_break_err!(linkInnerOuter(node.clone(), inst_scope.clone()), '__try0);
                    Ok::<(), anyhow::Error>(())
                }.is_err() {
                    if !(Flags::isSet(Flags::NF_API.clone())?) {
                        bail!("fail");
                    }
                }
            }
            unsafe { metamodelica::Dangerous::arrayInitSlot(comps.clone(), comp_idx.clone(), Mutable::create(node.clone())) };
            local_comps = cons(comp_idx.clone(), local_comps.clone());
            comp_idx = comp_idx.clone() + 1;
            ()
        },
        Deref @ InstNode::REF_NODE { .. } => {
            comp_idx = instExtendsComps(exts.borrow()[(var_field!((*c).index, InstNode::InstNode::REF_NODE).clone()-1) as usize].clone(), comps.clone(), comp_idx.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
            }
            breakComponents(instance.clone(), comps.clone(), ltree.clone(), dups.clone())?;
            if comp_idx.clone() != compCount.clone() + 1 {
                Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFClassTree.ClassTree.instantiate")); __mm_s.push_str(&*literal!(" miscounted components in ")); __mm_s.push_str(&*InstNode::name(clsNode.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            }
            if cls_idx.clone() != classCount.clone() + 1 {
                Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFClassTree.ClassTree.instantiate")); __mm_s.push_str(&*literal!(" miscounted classes in ")); __mm_s.push_str(&*InstNode::name(clsNode.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            }
            local_comps = local_comps.clone().reverse();
            assign_variant_field!(cls => Class::NFClass::EXPANDED_CLASS; elements = Arc::new(ClassTree::INSTANTIATED_TREE { tree: ltree.clone(), classes: clss.clone(), components: comps.clone(), localComponents: local_comps.clone(), exts: exts.clone(), imports: imps.clone(), duplicates: dups.clone() }));
            ()
        },
        Deref @ Class::EXPANDED_DERIVED { baseClass: node, .. } => {
            let mut node = (*node).clone();
            node = InstNode::setNodeType(Arc::new(InstNodeType::BASE_CLASS { parent: clsNode.clone(), definition: InstNode::definition(node.clone())?, ty: InstNode::nodeType(node.clone())? }), node.clone());
            (node, instance, classCount, compCount) = instantiate(node.clone(), instance.clone(), scope.clone())?;
            assign_variant_field!(cls => Class::NFClass::EXPANDED_DERIVED; baseClass = node.clone());
            ()
        },
        Deref @ Class::PARTIAL_BUILTIN { elements: tree @ Deref @ FLAT_TREE { components: old_comps, .. }, .. } => {
            let mut tree = (*tree).clone();
            instance = if (InstNode::isEmpty(instance.clone())) {clsNode.clone()} else {instance.clone()};
            assign_variant_field!(tree => ClassTree::FLAT_TREE; components = Array::map(old_comps.clone(), Arc::new({ let __pe_b1 = instance.clone(); move |__pe_a0| InstNode::cloneComponent(__pe_a0, __pe_b1.clone()) })));
            assign_variant_field!(cls => Class::NFClass::PARTIAL_BUILTIN; elements = tree.clone());
            compCount = (old_comps.clone().borrow().len() as i32);
            for mut bm in &*getBreakModsInExtend(instance.clone())? {
                let mut bm = bm.clone();
                Error::addSourceMessage(Error::NON_BREAKABLE_ELEMENT.clone(), list![(bm.ident.clone()).clone()], SCodeUtil::getModifierInfo(bm.r#mod.clone()))?;
                bail!("fail");
            }
            ()
        },
        Deref @ Class::PARTIAL_BUILTIN { .. } => (),
        Deref @ Class::INSTANCED_CLASS { .. } if (InstNode::isBaseClass(clsNode.clone())) => {
            let __pa0 = ::match_deref::match_deref! { match &(InstNode::nodeType(clsNode.clone())?) {
                Deref @ InstNodeType::BASE_CLASS { definition: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            ext_def = __pa0.clone();
            Error::addSourceMessage(Error::EXTENDS_LOOP.clone(), list![(SCodeUtil::getElementName(ext_def.clone())?).clone()], InstNode::info(clsNode.clone())?)?;
            bail!("fail")
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFClassTree.ClassTree.instantiate")); __mm_s.push_str(&*literal!(" got invalid class")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        InstNode::updateClass(cls.clone(), clsNode.clone())?;
        Ok((clsNode, instance, classCount, compCount))
    }

    pub fn fromRecordConstructor(mut fields: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut out: Arc<InstNode::InstNode>) -> Result<Arc<ClassTree>> {
        let mut tree: Arc<ClassTree> = EMPTY().clone();
        let mut ltree: Arc<LookupTree::Tree> = LookupTree::new();
        let mut i: i32 = 1;
        let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
        comps = metamodelica::arrayCreate((fields.clone().len() as i32) + 1, Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE));
        for mut ci in &*fields.clone() {
            let mut ci = ci.clone();
            {
                let __cell0 = ci.clone();
                unsafe { metamodelica::Dangerous::arrayInitSlot(comps.clone().clone(), i.clone(), __cell0); }
            }
            ltree = addLocalElement((InstNode::name(ci.clone())?).clone(), Arc::new(LookupTree::Entry::Entry::COMPONENT { index: i.clone() }), tree.clone(), ltree.clone())?;
            i = i.clone() + 1;
        }
        {
            let __cell1 = out.clone();
            unsafe { metamodelica::Dangerous::arrayInitSlot(comps.clone().clone(), i.clone(), __cell1); }
        }
        ltree = addLocalElement((InstNode::name(out.clone())?).clone(), Arc::new(LookupTree::Entry::Entry::COMPONENT { index: i.clone() }), tree.clone(), ltree.clone())?;
        tree = Arc::new(ClassTree::FLAT_TREE { tree: ltree.clone(), classes: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), components: comps.clone(), imports: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), duplicates: DuplicateTree::new() });
        Ok(tree)
    }

    pub fn clone(mut tree: Arc<ClassTree>) -> Arc<ClassTree> {
        let mut outTree: Arc<ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
        outTree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ EXPANDED_TREE { .. } => {
            let mut clss: metamodelica::Array<Arc<InstNode::InstNode>>;
            clss = metamodelica::arrayFromVec(var_field!((*tree).classes, ClassTree::EXPANDED_TREE).clone().borrow().clone());
            clss = Array::mapNoCopy(clss.clone(), (std::sync::Arc::new(InstNode::clone) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<Arc<InstNode::InstNode>> + 'static>));
            Arc::new(ClassTree::EXPANDED_TREE { tree: var_field!((*tree).tree, ClassTree::EXPANDED_TREE).clone(), classes: clss.clone(), components: var_field!((*tree).components, ClassTree::EXPANDED_TREE).clone(), exts: var_field!((*tree).exts, ClassTree::EXPANDED_TREE).clone(), imports: var_field!((*tree).imports, ClassTree::EXPANDED_TREE).clone(), duplicates: var_field!((*tree).duplicates, ClassTree::EXPANDED_TREE).clone() })
        },
        _ => {
            tree.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outTree
    }

    pub fn mapRedeclareChains(mut tree: Arc<ClassTree>, mut func: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>>) -> Result<()> + 'static>) -> () {
        pub type FuncT = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>>) -> Result<()> + 'static>;

        let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ INSTANTIATED_TREE { .. } if (!(DuplicateTree::isEmpty(var_field!((*tree).duplicates, ClassTree::INSTANTIATED_TREE).clone()))) => {
            DuplicateTree::map(var_field!((*tree).duplicates, ClassTree::INSTANTIATED_TREE).clone(), Arc::new({ let __pe_b2: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>>) -> Result<()> + 'static> = func.clone(); let __pe_b3 = tree.clone(); move |__pe_a0, __pe_a1| mapRedeclareChain(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        ()
    }

    pub fn replaceDuplicates(mut tree: Arc<ClassTree>) -> Arc<ClassTree> {
        let mut tree: Arc<ClassTree> = tree;
        let mut duplicates: Arc<DuplicateTree::Tree> = Arc::new(DuplicateTree::Tree::EMPTY);
        let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ INSTANTIATED_TREE { .. } if (!(DuplicateTree::isEmpty(var_field!((*tree).duplicates, ClassTree::INSTANTIATED_TREE).clone()))) => {
            (duplicates, tree) = DuplicateTree::mapFold(var_field!((*tree).duplicates, ClassTree::INSTANTIATED_TREE).clone(), (std::sync::Arc::new(replaceDuplicates2) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DuplicateTree::Entry>, Arc<ClassTree>) -> Result<(Arc<DuplicateTree::Entry>, Arc<ClassTree>)> + 'static>), tree.clone());
            assign_variant_field!(tree => ClassTree::INSTANTIATED_TREE; duplicates = duplicates.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        tree
    }

    pub fn appendComponentsToInstTree(mut components: Arc<metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>>, mut tree: Arc<ClassTree>) -> Result<Arc<ClassTree>> {
        let mut tree: Arc<ClassTree> = tree;
        if components.clone().is_empty() {
            return Ok(tree);
        } else {
            let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ INSTANTIATED_TREE { .. } => {
            let mut comp_idx: i32 = 0;
            let mut local_comps: Arc<metamodelica::List<i32>> = metamodelica::nil();
            comp_idx = (var_field!((*tree).components, ClassTree::INSTANTIATED_TREE).clone().borrow().len() as i32);
            assign_variant_field!(tree => ClassTree::INSTANTIATED_TREE; components = Array::appendList(var_field!((*tree).components, ClassTree::INSTANTIATED_TREE).clone(), components.clone())?);
            local_comps = var_field!((*tree).localComponents, ClassTree::INSTANTIATED_TREE).clone();
            for mut i in comp_idx.clone() + 1..=comp_idx.clone() + (components.clone().len() as i32) {
                local_comps = cons(i.clone(), local_comps.clone());
            }
            assign_variant_field!(tree => ClassTree::INSTANTIATED_TREE; localComponents = local_comps.clone());
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFClassTree.ClassTree.appendComponentsToInstTree")); __mm_s.push_str(&*literal!(" failed for non-instantiated tree.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        Ok(tree)
    }

    pub fn appendComponentsToFlatTree(mut components: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut tree: Arc<ClassTree>) -> Result<Arc<ClassTree>> {
        let mut tree: Arc<ClassTree> = tree;
        if components.clone().is_empty() {
            return Ok(tree);
        } else {
            let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ FLAT_TREE { .. } => {
            assign_variant_field!(tree => ClassTree::FLAT_TREE; components = Array::appendList(var_field!((*tree).components, ClassTree::FLAT_TREE).clone(), components.clone())?);
            ()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFClassTree.ClassTree.appendComponentsToFlatTree")); __mm_s.push_str(&*literal!(" failed for non-flat tree.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        Ok(tree)
    }

    pub fn flatten(mut tree: Arc<ClassTree>) -> Result<Arc<ClassTree>> {
        let mut tree: Arc<ClassTree> = tree;
        tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ INSTANTIATED_TREE { .. } => {
            let mut clss: metamodelica::Array<Arc<InstNode::InstNode>>;
            let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
            let mut comp_offsets: metamodelica::Array<i32>;
            let mut clsc: i32 = 0;
            let mut compc: i32 = 0;
            let mut dup_comp: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ltree: Arc<LookupTree::Tree> = Arc::new(LookupTree::Tree::EMPTY);
            (_, dup_comp) = enumerateDuplicates(var_field!((*tree).duplicates, ClassTree::INSTANTIATED_TREE).clone())?;
            clsc = (var_field!((*tree).classes, ClassTree::INSTANTIATED_TREE).clone().borrow().len() as i32);
            compc = (var_field!((*tree).components, ClassTree::INSTANTIATED_TREE).clone().borrow().len() as i32) - (dup_comp.clone().len() as i32);
            clss = metamodelica::arrayCreate(clsc.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE));
            comps = metamodelica::arrayCreate(compc.clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE));
            flattenElements(var_field!((*tree).classes, ClassTree::INSTANTIATED_TREE).clone(), clss.clone());
            if dup_comp.clone().is_empty() {
                flattenElements(var_field!((*tree).components, ClassTree::INSTANTIATED_TREE).clone(), comps.clone());
                ltree = var_field!((*tree).tree, ClassTree::INSTANTIATED_TREE).clone();
            } else {
                comp_offsets = createFlatOffsets((var_field!((*tree).components, ClassTree::INSTANTIATED_TREE).clone().borrow().len() as i32), dup_comp.clone())?;
                flattenElementsWithOffset(var_field!((*tree).components, ClassTree::INSTANTIATED_TREE).clone(), comps.clone(), comp_offsets.clone());
                ltree = flattenLookupTree(var_field!((*tree).tree, ClassTree::INSTANTIATED_TREE).clone(), comp_offsets.clone());
            }
            Arc::new(ClassTree::FLAT_TREE { tree: ltree.clone(), classes: clss.clone(), components: comps.clone(), imports: var_field!((*tree).imports, ClassTree::INSTANTIATED_TREE).clone(), duplicates: var_field!((*tree).duplicates, ClassTree::INSTANTIATED_TREE).clone() })
        },
        _ => {
            tree.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(tree)
    }

    pub fn flattenElements(mut elements: metamodelica::Array<Mutable::Mutable<Arc<InstNode::InstNode>>>, mut flatElements: metamodelica::Array<Arc<InstNode::InstNode>>) -> () {
        let __range0 = 1..=(elements.clone().borrow().len() as i32);
        for mut i in __range0 {
            {let _arr = flatElements.clone(); let _val = Mutable::access(elements.clone().borrow()[(i.clone()-1) as usize].clone()); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        }
        ()
    }

    pub fn flattenElementsWithOffset(mut elements: metamodelica::Array<Mutable::Mutable<Arc<InstNode::InstNode>>>, mut flatElements: metamodelica::Array<Arc<InstNode::InstNode>>, mut offsets: metamodelica::Array<i32>) -> () {
        let mut offset: i32 = 0;
        let __range0 = 1..=(elements.clone().borrow().len() as i32);
        for mut i in __range0 {
            offset = offsets.clone().borrow()[(i.clone()-1) as usize].clone();
            if offset.clone() >= 0 {
                {let _arr = flatElements.clone(); let _val = Mutable::access(elements.clone().borrow()[(i.clone()-1) as usize].clone()); _arr.borrow_mut()[(i.clone() - offset.clone()-1) as usize] = _val; _arr};
            }
        }
        ()
    }

    pub fn createFlatOffsets(mut elementCount: i32, mut duplicates: Arc<metamodelica::List<i32>>) -> Result<metamodelica::Array<i32>> {
        let mut offsets: metamodelica::Array<i32>;
        let mut offset: i32 = 0;
        let mut dup: i32 = 0;
        let mut rest_dups: Arc<metamodelica::List<i32>> = metamodelica::nil();
        offsets = metamodelica::arrayCreate(elementCount.clone(), 0);
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(duplicates.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        dup = __pa0.clone();
        rest_dups = __pa1.clone();
        for mut i in 1..=elementCount.clone() {
            if i.clone() == dup.clone() {
                if rest_dups.clone().is_empty() {
                    dup = 0;
                } else {
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_dups.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    dup = __pa2.clone();
                    rest_dups = __pa3.clone();
                }
                offset = offset.clone() + 1;
                unsafe { metamodelica::Dangerous::arrayInitSlot(offsets.clone(), i.clone(), -1) };
            } else {
                unsafe { metamodelica::Dangerous::arrayInitSlot(offsets.clone(), i.clone(), offset.clone()) };
            }
        }
        Ok(offsets)
    }

    pub fn flattenLookupTree(mut tree: Arc<LookupTree::Tree>, mut offsets: metamodelica::Array<i32>) -> Arc<LookupTree::Tree> {
        let mut tree: Arc<LookupTree::Tree> = tree;
        tree = LookupTree::map(tree.clone(), Arc::new({ let __pe_b2 = offsets.clone(); move |__pe_a0, __pe_a1| Ok(flattenLookupTree2(__pe_a0, __pe_a1, __pe_b2.clone())) }));
        tree
    }

    pub fn flattenLookupTree2(mut key: ArcStr, mut entry: Arc<LookupTree::Entry::Entry>, mut offsets: metamodelica::Array<i32>) -> Arc<LookupTree::Entry::Entry> {
        let mut outEntry: Arc<LookupTree::Entry::Entry>;
        outEntry = (::match_deref::match_deref! { match &(entry.clone()) {
        Deref @ LookupTree::Entry::COMPONENT { .. } => Arc::new(LookupTree::Entry::Entry::COMPONENT { index: var_field!((*entry).index, LookupTree::Entry::Entry::COMPONENT).clone() - offsets.clone().borrow()[(var_field!((*entry).index, LookupTree::Entry::Entry::COMPONENT).clone()-1) as usize].clone() }),
        _ => entry.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outEntry
    }

    pub fn lookupElement(mut name: ArcStr, mut tree: Arc<ClassTree>) -> Result<(Arc<InstNode::InstNode>, bool)> {
        let mut element: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut isImport: bool = false;
        let mut entry: Arc<LookupTree::Entry::Entry>;
        entry = LookupTree::get(lookupTree(tree.clone())?, (name.clone()).clone())?;
        (element, isImport) = resolveEntry(entry.clone(), tree.clone())?;
        Ok((element, isImport))
    }

    pub fn lookupElementPtr(mut name: ArcStr, mut tree: Arc<ClassTree>) -> Result<Mutable::Mutable<Arc<InstNode::InstNode>>> {
        let mut element: Mutable::Mutable<Arc<InstNode::InstNode>>;
        let mut entry: Arc<LookupTree::Entry::Entry>;
        entry = LookupTree::get(lookupTree(tree.clone())?, (name.clone()).clone())?;
        element = resolveEntryPtr(entry.clone(), tree.clone())?;
        Ok(element)
    }

    pub fn lookupElementsPtr(mut name: ArcStr, mut tree: Arc<ClassTree>) -> Result<Arc<metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>>> {
        let mut elements: Arc<metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>> = metamodelica::nil();
        let mut dup_entry: Arc<DuplicateTree::Entry>;
        match '__try0: {
            dup_entry = unwrap_break_err!(DuplicateTree::get(getDuplicates(tree.clone())?, (name.clone()).clone()), '__try0);
            elements = unwrap_break_err!(resolveDuplicateEntriesPtr(dup_entry.clone(), tree.clone(), metamodelica::nil()), '__try0);
            Ok::<_, anyhow::Error>((elements.clone(),))
        } {
            Ok((__try0_o0,)) => {
                elements = __try0_o0;
            }
            Err(_) => {
                elements = list![lookupElementPtr((name.clone()).clone(), tree.clone())?];
            }
        }
        Ok(elements)
    }

    pub fn lookupComponentIndex(mut name: ArcStr, mut tree: Arc<ClassTree>) -> Result<i32> {
        let mut index: i32 = 0;
        let __pa0 = ::match_deref::match_deref! { match &(LookupTree::get(lookupTree(tree.clone())?, (name.clone()).clone())?) {
            Deref @ LookupTree::Entry::COMPONENT { index: __pa0 } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        index = __pa0.clone();
        Ok(index)
    }

    pub fn nthComponent(mut index: i32, mut tree: Arc<ClassTree>) -> Result<Arc<InstNode::InstNode>> {
        let mut component: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        component = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ PARTIAL_TREE { .. } => var_field!((*tree).components, ClassTree::PARTIAL_TREE).clone().borrow()[(index.clone()-1) as usize].clone(),
        Deref @ EXPANDED_TREE { .. } => var_field!((*tree).components, ClassTree::EXPANDED_TREE).clone().borrow()[(index.clone()-1) as usize].clone(),
        Deref @ INSTANTIATED_TREE { .. } => Mutable::access(var_field!((*tree).components, ClassTree::INSTANTIATED_TREE).clone().borrow()[(index.clone()-1) as usize].clone()),
        Deref @ FLAT_TREE { .. } => var_field!((*tree).components, ClassTree::FLAT_TREE).clone().borrow()[(index.clone()-1) as usize].clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok(component)
    }

    pub fn mapClasses(mut tree: Arc<ClassTree>, mut func: Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<Arc<InstNode::InstNode>> + 'static>) -> () {
        pub type FuncT = std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<Arc<InstNode::InstNode>> + 'static>;

        let mut clss: metamodelica::Array<Arc<InstNode::InstNode>> = getClasses(tree.clone()).unwrap();
        let __range0 = 1..=(clss.clone().borrow().len() as i32);
        for mut i in __range0 {
            {let _arr = clss.clone(); let _val = func(clss.clone().borrow()[(i.clone()-1) as usize].clone()).unwrap(); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        }
        ()
    }

    pub fn foldClasses<ArgT: Clone + 'static>(mut tree: Arc<ClassTree>, mut func: Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> ArgT {
        pub type FuncT<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, ArgT) -> Result<ArgT> + 'static>;

        let mut arg: ArgT = arg;
        let mut clss: metamodelica::Array<Arc<InstNode::InstNode>> = getClasses(tree.clone()).unwrap();
        let __range0 = clss.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut cls in __range0 {
            arg = func(cls.clone(), arg.clone()).unwrap();
        }
        arg
    }

    pub fn applyExtends(mut tree: Arc<ClassTree>, mut func: Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<()> + 'static>) -> () {
        pub type FuncT = std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<()> + 'static>;

        let mut exts: metamodelica::Array<Arc<InstNode::InstNode>> = getExtends(tree.clone());
        let __range0 = exts.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut ext in __range0 {
            func(ext.clone()).unwrap();
        }
        ()
    }

    pub fn mapExtends(mut tree: Arc<ClassTree>, mut func: Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<Arc<InstNode::InstNode>> + 'static>) -> () {
        pub type FuncT = std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<Arc<InstNode::InstNode>> + 'static>;

        let mut exts: metamodelica::Array<Arc<InstNode::InstNode>> = getExtends(tree.clone());
        let __range0 = 1..=(exts.clone().borrow().len() as i32);
        for mut i in __range0 {
            {let _arr = exts.clone(); let _val = func(exts.clone().borrow()[(i.clone()-1) as usize].clone()).unwrap(); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
        }
        ()
    }

    pub fn foldExtends<ArgT: Clone + 'static>(mut tree: Arc<ClassTree>, mut func: Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> ArgT {
        pub type FuncT<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, ArgT) -> Result<ArgT> + 'static>;

        let mut arg: ArgT = arg;
        let mut exts: metamodelica::Array<Arc<InstNode::InstNode>> = getExtends(tree.clone());
        let __range0 = exts.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut ext in __range0 {
            arg = func(ext.clone(), arg.clone()).unwrap();
        }
        arg
    }

    pub fn mapFoldExtends<ArgT: Clone + 'static>(mut tree: Arc<ClassTree>, mut func: Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, ArgT) -> Result<(Arc<InstNode::InstNode>, ArgT)> + 'static>, mut arg: ArgT) -> ArgT {
        pub type FuncT<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, ArgT) -> Result<(Arc<InstNode::InstNode>, ArgT)> + 'static>;

        let mut arg: ArgT = arg;
        let mut exts: metamodelica::Array<Arc<InstNode::InstNode>> = getExtends(tree.clone());
        let mut ext: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let __range0 = 1..=(exts.clone().borrow().len() as i32);
        for mut i in __range0 {
            (ext, arg) = func(exts.clone().borrow()[(i.clone()-1) as usize].clone(), arg.clone()).unwrap();
            {let _arr = exts.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = ext.clone(); _arr};
        }
        arg
    }

    pub fn applyLocalComponents(mut tree: Arc<ClassTree>, mut func: Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<()> + 'static>) -> Result<()> {
        pub type FuncT = std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<()> + 'static>;

        let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ INSTANTIATED_TREE { .. } => {
            for mut i in &*var_field!((*tree).localComponents, ClassTree::INSTANTIATED_TREE).clone() {
                let mut i = i.clone();
                func(Mutable::access(var_field!((*tree).components, ClassTree::INSTANTIATED_TREE).clone().borrow()[(i.clone()-1) as usize].clone()))?;
            }
            ()
        },
        Deref @ PARTIAL_TREE { .. } => {
            let __range0 = var_field!((*tree).components, ClassTree::PARTIAL_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                func(c.clone())?;
            }
            ()
        },
        Deref @ EXPANDED_TREE { .. } => {
            let __range0 = var_field!((*tree).components, ClassTree::EXPANDED_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                func(c.clone())?;
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(())
    }

    pub fn applyComponents(mut tree: Arc<ClassTree>, mut func: Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<()> + 'static>) -> () {
        pub type FuncT = std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<()> + 'static>;

        let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ PARTIAL_TREE { .. } => {
            let __range0 = var_field!((*tree).components, ClassTree::PARTIAL_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                func(c.clone()).unwrap();
            }
            ()
        },
        Deref @ EXPANDED_TREE { .. } => {
            let __range0 = var_field!((*tree).components, ClassTree::EXPANDED_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                func(c.clone()).unwrap();
            }
            ()
        },
        Deref @ INSTANTIATED_TREE { .. } => {
            let __range0 = var_field!((*tree).components, ClassTree::INSTANTIATED_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                func(Mutable::access(c.clone())).unwrap();
            }
            ()
        },
        Deref @ FLAT_TREE { .. } => {
            let __range0 = var_field!((*tree).components, ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                func(c.clone()).unwrap();
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        ()
    }

    pub fn foldComponents<ArgT: Clone + 'static>(mut tree: Arc<ClassTree>, mut func: Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> ArgT {
        pub type FuncT<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, ArgT) -> Result<ArgT> + 'static>;

        let mut arg: ArgT = arg;
        let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ PARTIAL_TREE { .. } => {
            let __range0 = var_field!((*tree).components, ClassTree::PARTIAL_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                arg = func(c.clone(), arg.clone()).unwrap();
            }
            ()
        },
        Deref @ EXPANDED_TREE { .. } => {
            let __range0 = var_field!((*tree).components, ClassTree::EXPANDED_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                arg = func(c.clone(), arg.clone()).unwrap();
            }
            ()
        },
        Deref @ INSTANTIATED_TREE { .. } => {
            let __range0 = var_field!((*tree).components, ClassTree::INSTANTIATED_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                arg = func(Mutable::access(c.clone()), arg.clone()).unwrap();
            }
            ()
        },
        Deref @ FLAT_TREE { .. } => {
            let __range0 = var_field!((*tree).components, ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                arg = func(c.clone(), arg.clone()).unwrap();
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        arg
    }

    pub fn findComponent(mut tree: Arc<ClassTree>, mut func: Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<bool> + 'static>) -> Option<Arc<InstNode::InstNode>> {
        pub type FuncT = std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<bool> + 'static>;

        let mut component: Option<Arc<InstNode::InstNode>> = None;
        let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ PARTIAL_TREE { .. } => {
            let __range0 = var_field!((*tree).components, ClassTree::PARTIAL_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                if func(c.clone()).unwrap() {
                    component = Some(c.clone());
                    break;
                }
            }
            ()
        },
        Deref @ EXPANDED_TREE { .. } => {
            let __range0 = var_field!((*tree).components, ClassTree::EXPANDED_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                if func(c.clone()).unwrap() {
                    component = Some(c.clone());
                    break;
                }
            }
            ()
        },
        Deref @ INSTANTIATED_TREE { .. } => {
            let __range0 = var_field!((*tree).components, ClassTree::INSTANTIATED_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                if func(Mutable::access(c.clone())).unwrap() {
                    component = Some(Mutable::access(c.clone()));
                    break;
                }
            }
            ()
        },
        Deref @ FLAT_TREE { .. } => {
            let __range0 = var_field!((*tree).components, ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                if func(c.clone()).unwrap() {
                    component = Some(c.clone());
                    break;
                }
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        component
    }

    pub fn classCount(mut tree: Arc<ClassTree>) -> i32 {
        let mut count: i32 = 0;
        count = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ PARTIAL_TREE { .. } => (var_field!((*tree).classes, ClassTree::PARTIAL_TREE).clone().borrow().len() as i32),
        Deref @ EXPANDED_TREE { .. } => (var_field!((*tree).classes, ClassTree::EXPANDED_TREE).clone().borrow().len() as i32),
        Deref @ INSTANTIATED_TREE { .. } => (var_field!((*tree).classes, ClassTree::INSTANTIATED_TREE).clone().borrow().len() as i32),
        Deref @ FLAT_TREE { .. } => (var_field!((*tree).classes, ClassTree::FLAT_TREE).clone().borrow().len() as i32),
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        count
    }

    pub fn componentCount(mut tree: Arc<ClassTree>) -> i32 {
        let mut count: i32 = 0;
        count = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ PARTIAL_TREE { .. } => (var_field!((*tree).components, ClassTree::PARTIAL_TREE).clone().borrow().len() as i32) - (var_field!((*tree).exts, ClassTree::PARTIAL_TREE).clone().borrow().len() as i32),
        Deref @ EXPANDED_TREE { .. } => (var_field!((*tree).components, ClassTree::EXPANDED_TREE).clone().borrow().len() as i32) - (var_field!((*tree).exts, ClassTree::EXPANDED_TREE).clone().borrow().len() as i32),
        Deref @ INSTANTIATED_TREE { .. } => (var_field!((*tree).components, ClassTree::INSTANTIATED_TREE).clone().borrow().len() as i32),
        Deref @ FLAT_TREE { .. } => (var_field!((*tree).components, ClassTree::FLAT_TREE).clone().borrow().len() as i32),
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        count
    }

    pub fn extendsCount(mut tree: Arc<ClassTree>) -> i32 {
        let mut count: i32 = (getExtends(tree.clone()).borrow().len() as i32);
        count
    }

    pub fn recursiveElementCount(mut tree: Arc<ClassTree>) -> Result<i32> {
        let mut count: i32 = 0;
        count = classCount(tree.clone()) + componentCount(tree.clone());
        let __range0 = getExtends(tree.clone()).borrow().iter().cloned().collect::<Vec<_>>();
        for mut ext in __range0 {
            count = count.clone() + recursiveElementCount(Class::classTree(InstNode::getClass(ext.clone())?)?)?;
        }
        Ok(count)
    }

    pub fn checkDuplicates(mut tree: Arc<ClassTree>) -> () {
        let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ INSTANTIATED_TREE { .. } if (!(DuplicateTree::isEmpty(var_field!((*tree).duplicates, ClassTree::INSTANTIATED_TREE).clone()))) => {
            DuplicateTree::fold(var_field!((*tree).duplicates, ClassTree::INSTANTIATED_TREE).clone(), (std::sync::Arc::new(checkDuplicates2) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DuplicateTree::Entry>, Arc<ClassTree>) -> Result<Arc<ClassTree>> + 'static>), tree.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        ()
    }

    pub fn checkDuplicates2(mut name: ArcStr, mut entry: Arc<DuplicateTree::Entry>, mut tree: Arc<ClassTree>) -> Result<Arc<ClassTree>> {
        let mut tree: Arc<ClassTree> = tree;
        let mut kept: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut dup: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        if isNone(entry.node.clone()) {
            return Ok(tree);
        }
        let __pa0 = ::match_deref::match_deref! { match &(entry.node.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        kept = __pa0.clone();
        let () = (match entry.ty.clone() {
        DuplicateTree::EntryType::REDECLARE => (),
        _ => {
            for mut c in &*entry.children.clone() {
                let mut c = c.clone();
                let __pa0 = ::match_deref::match_deref! { match &(c.node.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                dup = __pa0.clone();
                if !(InstNode::isEmpty(dup.clone())) {
                    InstNode::checkIdentical(kept.clone(), dup.clone())?;
                }
            }
            ()
        },
    });
        Ok(tree)
    }

    pub fn isIdentical(mut tree1: Arc<ClassTree>, mut tree2: Arc<ClassTree>) -> bool {
        let mut identical: bool = false;
        identical = true;
        identical
    }

    pub fn getRedeclaredNode(mut name: ArcStr, mut tree: Arc<ClassTree>) -> Result<Arc<InstNode::InstNode>> {
        let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut entry: Arc<DuplicateTree::Entry>;
        match '__try0: {
            entry = unwrap_break_err!(DuplicateTree::get(getDuplicates(tree.clone())?, (name.clone()).clone()), '__try0);
            entry = unwrap_break_err!(listHead(entry.children.clone()), '__try0);
            if isSome(entry.node.clone()) {
                let __pa1 = ::match_deref::match_deref! { match &(entry.node.clone()) {
                    Some(__pa1) => __pa1.clone(),
                    _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                node = __pa1.clone();
            } else {
                (node, _) = unwrap_break_err!(resolveEntry(entry.entry.clone(), tree.clone()), '__try0);
            }
            Ok::<_, anyhow::Error>((entry.clone(), node.clone()))
        } {
            Ok((__try0_o0, __try0_o1)) => {
                entry = __try0_o0;
                node = __try0_o1;
            }
            Err(_) => {
                Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFClassTree.ClassTree.getRedeclaredNode")); __mm_s.push_str(&*literal!(" failed on ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                bail!("try/else: outputs not set in else branch");
            }
        }
        Ok(node)
    }

    pub fn setClassExtends(mut extNode: Arc<InstNode::InstNode>, mut tree: Arc<ClassTree>) -> Result<Arc<ClassTree>> {
        let mut tree: Arc<ClassTree> = tree;
        {let _arr = getExtends(tree.clone()); _arr.borrow_mut()[(1-1) as usize] = extNode.clone(); _arr};
        Ok(tree)
    }

    pub fn enumerateComponents(mut tree: Arc<ClassTree>) -> Result<Arc<metamodelica::List<Arc<InstNode::InstNode>>>> {
        let mut components: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = metamodelica::nil();
        let mut ltree: Arc<LookupTree::Tree> = Arc::new(LookupTree::Tree::EMPTY);
        let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tree.clone()) {
            Deref @ FLAT_TREE { components: __pa0, tree: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        comps = __pa0.clone();
        ltree = __pa1.clone();
        components = LookupTree::fold(ltree.clone(), Arc::new({ let __pe_b2 = comps.clone(); move |__pe_a0, __pe_a1, __pe_a3| Ok(enumerateComponents2(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_a3)) }), metamodelica::nil());
        Ok(components)
    }

    pub fn enumerateComponents2(mut name: ArcStr, mut entry: Arc<LookupTree::Entry::Entry>, mut comps: metamodelica::Array<Arc<InstNode::InstNode>>, mut components: Arc<metamodelica::List<Arc<InstNode::InstNode>>>) -> Arc<metamodelica::List<Arc<InstNode::InstNode>>> {
        let mut components: Arc<metamodelica::List<Arc<InstNode::InstNode>>> = components;
        let () = (::match_deref::match_deref! { match &(entry.clone()) {
        Deref @ LookupTree::Entry::COMPONENT { .. } => {
            components = cons(comps.borrow()[(var_field!((*entry).index, LookupTree::Entry::Entry::COMPONENT).clone()-1) as usize].clone(), components.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        components
    }

    pub fn getClasses(mut tree: Arc<ClassTree>) -> Result<metamodelica::Array<Arc<InstNode::InstNode>>> {
        let mut clss: metamodelica::Array<Arc<InstNode::InstNode>>;
        clss = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ PARTIAL_TREE { .. } => var_field!((*tree).classes, ClassTree::PARTIAL_TREE).clone(),
        Deref @ EXPANDED_TREE { .. } => var_field!((*tree).classes, ClassTree::EXPANDED_TREE).clone(),
        Deref @ FLAT_TREE { .. } => var_field!((*tree).classes, ClassTree::FLAT_TREE).clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok(clss)
    }

    pub fn getExtends(mut tree: Arc<ClassTree>) -> metamodelica::Array<Arc<InstNode::InstNode>> {
        let mut exts: metamodelica::Array<Arc<InstNode::InstNode>>;
        exts = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ PARTIAL_TREE { .. } => var_field!((*tree).exts, ClassTree::PARTIAL_TREE).clone(),
        Deref @ EXPANDED_TREE { .. } => var_field!((*tree).exts, ClassTree::EXPANDED_TREE).clone(),
        Deref @ INSTANTIATED_TREE { .. } => var_field!((*tree).exts, ClassTree::INSTANTIATED_TREE).clone(),
        _ => metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        exts
    }

    pub fn getComponents(mut tree: Arc<ClassTree>) -> Result<metamodelica::Array<Arc<InstNode::InstNode>>> {
        let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
        comps = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ PARTIAL_TREE { .. } => var_field!((*tree).components, ClassTree::PARTIAL_TREE).clone(),
        Deref @ EXPANDED_TREE { .. } => var_field!((*tree).components, ClassTree::EXPANDED_TREE).clone(),
        Deref @ FLAT_TREE { .. } => var_field!((*tree).components, ClassTree::FLAT_TREE).clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok(comps)
    }

    pub fn getImports(mut tree: Arc<ClassTree>) -> Result<metamodelica::Array<Arc<Import::NFImport>>> {
        let mut imps: metamodelica::Array<Arc<Import::NFImport>>;
        imps = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ PARTIAL_TREE { .. } => var_field!((*tree).imports, ClassTree::PARTIAL_TREE).clone(),
        Deref @ EXPANDED_TREE { .. } => var_field!((*tree).imports, ClassTree::EXPANDED_TREE).clone(),
        Deref @ INSTANTIATED_TREE { .. } => var_field!((*tree).imports, ClassTree::INSTANTIATED_TREE).clone(),
        Deref @ FLAT_TREE { .. } => var_field!((*tree).imports, ClassTree::FLAT_TREE).clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok(imps)
    }

    pub fn isEmptyTree(mut tree: Arc<ClassTree>) -> bool {
        let mut isEmpty: bool = false;
        isEmpty = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ EMPTY_TREE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        isEmpty
    }

    pub fn appendClasses(mut clsNodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut tree: Arc<ClassTree>) -> Result<Arc<ClassTree>> {
        let mut tree: Arc<ClassTree> = tree;
        let mut classes: metamodelica::Array<Arc<InstNode::InstNode>>;
        let mut ltree: Arc<LookupTree::Tree> = Arc::new(LookupTree::Tree::EMPTY);
        let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ PARTIAL_TREE { .. } => {
            (ltree, classes) = appendClasses2(clsNodes.clone(), var_field!((*tree).tree, ClassTree::PARTIAL_TREE).clone(), var_field!((*tree).classes, ClassTree::PARTIAL_TREE).clone())?;
            assign_variant_field!(tree => ClassTree::PARTIAL_TREE;
                tree = ltree.clone(),
                classes = classes.clone()
            );
            ()
        },
        Deref @ EXPANDED_TREE { .. } => {
            (ltree, classes) = appendClasses2(clsNodes.clone(), var_field!((*tree).tree, ClassTree::EXPANDED_TREE).clone(), var_field!((*tree).classes, ClassTree::EXPANDED_TREE).clone())?;
            assign_variant_field!(tree => ClassTree::EXPANDED_TREE;
                tree = ltree.clone(),
                classes = classes.clone()
            );
            ()
        },
        Deref @ FLAT_TREE { .. } => {
            (ltree, classes) = appendClasses2(clsNodes.clone(), var_field!((*tree).tree, ClassTree::FLAT_TREE).clone(), var_field!((*tree).classes, ClassTree::FLAT_TREE).clone())?;
            assign_variant_field!(tree => ClassTree::FLAT_TREE;
                tree = ltree.clone(),
                classes = classes.clone()
            );
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(tree)
    }

    pub fn appendClasses2(mut clsNodes: Arc<metamodelica::List<Arc<InstNode::InstNode>>>, mut tree: Arc<LookupTree::Tree>, mut classes: metamodelica::Array<Arc<InstNode::InstNode>>) -> Result<(Arc<LookupTree::Tree>, metamodelica::Array<Arc<InstNode::InstNode>>)> {
        let mut tree: Arc<LookupTree::Tree> = tree;
        let mut classes: metamodelica::Array<Arc<InstNode::InstNode>> = classes;
        let mut index: i32 = 0;
        index = (classes.clone().borrow().len() as i32);
        classes = Array::appendList(classes.clone(), clsNodes.clone())?;
        for mut c in &*clsNodes.clone() {
            let mut c = c.clone();
            index = index.clone() + 1;
            tree = LookupTree::add(tree.clone(), (InstNode::name(c.clone())?).clone(), Arc::new(LookupTree::Entry::Entry::CLASS { index: index.clone() }), (std::sync::Arc::new(LookupTree::addConflictDefault) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
        }
        Ok((tree, classes))
    }

    pub fn replaceClass(mut node: Arc<InstNode::InstNode>, mut tree: Arc<ClassTree>) -> Result<Arc<ClassTree>> {
        let mut tree: Arc<ClassTree> = tree;
        let mut index: i32 = 0;
        let __pa0 = ::match_deref::match_deref! { match &(LookupTree::get(lookupTree(tree.clone())?, (InstNode::name(node.clone())?).clone())?) {
            Deref @ LookupTree::Entry::CLASS { index: __pa0 } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        index = __pa0.clone();
        {let _arr = getClasses(tree.clone())?; _arr.borrow_mut()[(index.clone()-1) as usize] = node.clone(); _arr};
        Ok(tree)
    }

    fn instExtendsComps(mut extNode: Arc<InstNode::InstNode>, mut comps: metamodelica::Array<Mutable::Mutable<Arc<InstNode::InstNode>>>, mut index: i32) -> Result<i32> {
        let mut index: i32 = index;
        let mut ext_comps_ptrs: metamodelica::Array<Mutable::Mutable<Arc<InstNode::InstNode>>>;
        let mut ext_comps: metamodelica::Array<Arc<InstNode::InstNode>>;
        let mut comp_count: i32 = 0;
        let mut ext_comp: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let () = (::match_deref::match_deref! { match &(Class::classTree(InstNode::getClass(extNode.clone())?)?) {
        Deref @ INSTANTIATED_TREE { components: ext_comps_ptrs, .. } => {
            comp_count = (ext_comps_ptrs.clone().borrow().len() as i32);
            if comp_count.clone() > 0 {
                Array::copyRange(ext_comps_ptrs.clone(), comps.clone(), 1, comp_count.clone(), index.clone())?;
                index = index.clone() + comp_count.clone();
            }
            ()
        },
        Deref @ FLAT_TREE { components: ext_comps, .. } => {
            comp_count = (ext_comps.clone().borrow().len() as i32);
            if comp_count.clone() > 0 {
                for mut i in index.clone()..=index.clone() + comp_count.clone() - 1 {
                    {let _arr = comps.clone(); let _val = Mutable::create(ext_comps.borrow()[(i.clone()-1) as usize].clone()); _arr.borrow_mut()[(i.clone()-1) as usize] = _val; _arr};
                }
                index = index.clone() + comp_count.clone();
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(index)
    }

    fn getDuplicates(mut tree: Arc<ClassTree>) -> Result<Arc<DuplicateTree::Tree>> {
        let mut duplicates: Arc<DuplicateTree::Tree> = Arc::new(DuplicateTree::Tree::EMPTY);
        duplicates = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ PARTIAL_TREE { .. } => var_field!((*tree).duplicates, ClassTree::PARTIAL_TREE).clone(),
        Deref @ EXPANDED_TREE { .. } => var_field!((*tree).duplicates, ClassTree::EXPANDED_TREE).clone(),
        Deref @ INSTANTIATED_TREE { .. } => var_field!((*tree).duplicates, ClassTree::INSTANTIATED_TREE).clone(),
        Deref @ FLAT_TREE { .. } => var_field!((*tree).duplicates, ClassTree::FLAT_TREE).clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok(duplicates)
    }

    fn lookupTree(mut ctree: Arc<ClassTree>) -> Result<Arc<LookupTree::Tree>> {
        let mut ltree: Arc<LookupTree::Tree> = Arc::new(LookupTree::Tree::EMPTY);
        ltree = (::match_deref::match_deref! { match &(ctree.clone()) {
        Deref @ PARTIAL_TREE { .. } => var_field!((*ctree).tree, ClassTree::PARTIAL_TREE).clone(),
        Deref @ EXPANDED_TREE { .. } => var_field!((*ctree).tree, ClassTree::EXPANDED_TREE).clone(),
        Deref @ INSTANTIATED_TREE { .. } => var_field!((*ctree).tree, ClassTree::INSTANTIATED_TREE).clone(),
        Deref @ FLAT_TREE { .. } => var_field!((*ctree).tree, ClassTree::FLAT_TREE).clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok(ltree)
    }

    fn setLookupTree(mut ltree: Arc<LookupTree::Tree>, mut ctree: Arc<ClassTree>) -> Arc<ClassTree> {
        let mut ctree: Arc<ClassTree> = ctree;
        let () = (::match_deref::match_deref! { match &(ctree.clone()) {
        Deref @ PARTIAL_TREE { .. } => {
            assign_variant_field!(ctree => ClassTree::PARTIAL_TREE; tree = ltree.clone());
            ()
        },
        Deref @ EXPANDED_TREE { .. } => {
            assign_variant_field!(ctree => ClassTree::EXPANDED_TREE; tree = ltree.clone());
            ()
        },
        Deref @ INSTANTIATED_TREE { .. } => {
            assign_variant_field!(ctree => ClassTree::INSTANTIATED_TREE; tree = ltree.clone());
            ()
        },
        Deref @ FLAT_TREE { .. } => {
            assign_variant_field!(ctree => ClassTree::FLAT_TREE; tree = ltree.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        ctree
    }

    fn addLocalElement(mut name: ArcStr, mut entry: Arc<LookupTree::Entry::Entry>, mut classTree: Arc<ClassTree>, mut tree: Arc<LookupTree::Tree>) -> Result<Arc<LookupTree::Tree>> {
        let mut tree: Arc<LookupTree::Tree> = tree;
        tree = LookupTree::add(tree.clone(), (name.clone()).clone(), entry.clone(), Arc::new({ let __pe_b3 = classTree.clone(); move |__pe_a0, __pe_a1, __pe_a2| addLocalElementConflict(__pe_a0, __pe_a1, __pe_a2, __pe_b3.clone()) }))?;
        Ok(tree)
    }

    fn addLocalElementConflict(mut newEntry: Arc<LookupTree::Entry::Entry>, mut oldEntry: Arc<LookupTree::Entry::Entry>, mut name: ArcStr, mut classTree: Arc<ClassTree>) -> Result<Arc<LookupTree::Entry::Entry>> {
        let mut entry: Arc<LookupTree::Entry::Entry>;
        let mut n1: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut n2: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        entry = (::match_deref::match_deref! { match &((newEntry.clone(), oldEntry.clone())) {
        (_, Deref @ LookupTree::Entry::IMPORT { .. }) => newEntry.clone(),
        _ => {
            n1 = findLocalConflictElement(newEntry.clone(), classTree.clone())?;
            n2 = findLocalConflictElement(oldEntry.clone(), classTree.clone())?;
            Error::addMultiSourceMessage(Error::DOUBLE_DECLARATION_OF_ELEMENTS.clone(), list![(name.clone()).clone()], list![InstNode::info(n2.clone())?, InstNode::info(n1.clone())?])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(entry)
    }

    fn findLocalConflictElement(mut entry: Arc<LookupTree::Entry::Entry>, mut classTree: Arc<ClassTree>) -> Result<Arc<InstNode::InstNode>> {
        let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        node = (::match_deref::match_deref! { match &(entry.clone()) {
        Deref @ LookupTree::Entry::CLASS { .. } => {
            resolveClass(var_field!((*entry).index, LookupTree::Entry::Entry::CLASS).clone(), classTree.clone())?
        },
        Deref @ LookupTree::Entry::COMPONENT { .. } => {
            let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
            let mut exts: metamodelica::Array<Arc<InstNode::InstNode>>;
            let mut i: i32 = 0;
            i = 0;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(classTree.clone()) {
                Deref @ PARTIAL_TREE { exts: __pa0, components: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            exts = __pa0.clone();
            comps = __pa1.clone();
            let __range2 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range2 {
                i = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ InstNode::COMPONENT_NODE { .. } => i.clone() + 1,
        Deref @ InstNode::REF_NODE { .. } => {
            (_, i) = countInheritedElements(exts.borrow()[(var_field!((*c).index, InstNode::InstNode::REF_NODE).clone()-1) as usize].clone(), 0, i.clone())?;
            i.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
                if i.clone() == var_field!((*entry).index, LookupTree::Entry::Entry::COMPONENT).clone() {
                    node = c.clone();
                    break;
                }
            }
            Error::assertion(i.clone() == var_field!((*entry).index, LookupTree::Entry::Entry::COMPONENT).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFClassTree.ClassTree.findLocalConflictElement")); __mm_s.push_str(&*literal!(" got invalid entry index")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            node.clone()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFClassTree.ClassTree.findLocalConflictElement")); __mm_s.push_str(&*literal!(" got invalid entry")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(node)
    }

    fn addEnumConflict(mut newEntry: Arc<LookupTree::Entry::Entry>, mut oldEntry: Arc<LookupTree::Entry::Entry>, mut name: ArcStr, mut literal: Arc<InstNode::InstNode>) -> Result<Arc<LookupTree::Entry::Entry>> {
        let mut entry: Arc<LookupTree::Entry::Entry>;
        Error::addSourceMessage(Error::DOUBLE_DECLARATION_OF_ELEMENTS.clone(), list![(InstNode::name(literal.clone())?).clone()], InstNode::info(literal.clone())?)?;
        bail!("fail");
        Ok(entry)
    }

    fn addImport(mut imp: Arc<Import::NFImport>, mut index: i32, mut tree: Arc<LookupTree::Tree>, mut imports: metamodelica::Array<Arc<Import::NFImport>>) -> Result<Arc<LookupTree::Tree>> {
        let mut tree: Arc<LookupTree::Tree> = tree;
        tree = LookupTree::add(tree.clone(), (Import::name(imp.clone())?).clone(), Arc::new(LookupTree::Entry::Entry::IMPORT { index: index.clone() }), Arc::new({ let __pe_b3 = imports.clone(); move |__pe_a0, __pe_a1, __pe_a2| addImportConflict(__pe_a0, __pe_a1, __pe_a2, __pe_b3.clone()) }))?;
        Ok(tree)
    }

    fn addImportConflict(mut newEntry: Arc<LookupTree::Entry::Entry>, mut oldEntry: Arc<LookupTree::Entry::Entry>, mut name: ArcStr, mut imports: metamodelica::Array<Arc<Import::NFImport>>) -> Result<Arc<LookupTree::Entry::Entry>> {
        let mut entry: Arc<LookupTree::Entry::Entry>;
        entry = (::match_deref::match_deref! { match &((newEntry.clone(), oldEntry.clone())) {
        (Deref @ LookupTree::Entry::IMPORT { .. }, Deref @ LookupTree::Entry::IMPORT { .. }) => {
            let mut imp1: Arc<Import::NFImport>;
            let mut imp2: Arc<Import::NFImport>;
            imp1 = imports.borrow()[(var_field!((*newEntry).index, LookupTree::Entry::Entry::IMPORT).clone()-1) as usize].clone();
            imp2 = imports.borrow()[(var_field!((*oldEntry).index, LookupTree::Entry::Entry::IMPORT).clone()-1) as usize].clone();
            entry = (::match_deref::match_deref! { match &((imp1.clone(), imp2.clone())) {
        (Deref @ Import::UNRESOLVED_IMPORT { .. }, Deref @ Import::UNRESOLVED_IMPORT { .. }) => {
            {let _arr = imports.clone(); _arr.borrow_mut()[(var_field!((*oldEntry).index, LookupTree::Entry::Entry::IMPORT).clone()-1) as usize] = Arc::new(Import::NFImport::CONFLICTING_IMPORT { imp1: imp1.clone(), imp2: imp2.clone() }); _arr};
            oldEntry.clone()
        },
        (Deref @ Import::RESOLVED_IMPORT { .. }, Deref @ Import::RESOLVED_IMPORT { .. }) => {
            {let _arr = imports.clone(); _arr.borrow_mut()[(var_field!((*oldEntry).index, LookupTree::Entry::Entry::IMPORT).clone()-1) as usize] = Arc::new(Import::NFImport::CONFLICTING_IMPORT { imp1: imp1.clone(), imp2: imp2.clone() }); _arr};
            oldEntry.clone()
        },
        (Deref @ Import::UNRESOLVED_IMPORT { .. }, _) => newEntry.clone(),
        _ => oldEntry.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            entry.clone()
        },
        _ => {
            oldEntry.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(entry)
    }

    fn addDuplicate(mut name: ArcStr, mut duplicateEntry: Arc<LookupTree::Entry::Entry>, mut keptEntry: Arc<LookupTree::Entry::Entry>, mut duplicates: Mutable::Mutable<Arc<DuplicateTree::Tree>>) -> Result<Mutable::Mutable<Arc<DuplicateTree::Tree>>> {
        let mut duplicates: Mutable::Mutable<Arc<DuplicateTree::Tree>> = duplicates;
        Mutable::update(duplicates.clone(), DuplicateTree::add(Mutable::access(duplicates.clone()), (name.clone()).clone(), DuplicateTree::newDuplicate(keptEntry.clone(), duplicateEntry.clone()), (std::sync::Arc::new(addDuplicateConflict) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DuplicateTree::Entry>, Arc<DuplicateTree::Entry>, ArcStr) -> Result<Arc<DuplicateTree::Entry>> + 'static>))?);
        Ok(duplicates)
    }

    fn addDuplicateConflict(mut newEntry: Arc<DuplicateTree::Entry>, mut oldEntry: Arc<DuplicateTree::Entry>, mut name: ArcStr) -> Result<Arc<DuplicateTree::Entry>> {
        let mut entry: Arc<DuplicateTree::Entry>;
        entry = Arc::new(DuplicateTree::Entry { entry: newEntry.entry.clone(), node: None, children: cons(listHead(newEntry.children.clone())?, oldEntry.children.clone()), ty: DuplicateTree::EntryType::DUPLICATE.clone() });
        Ok(entry)
    }

    fn resolveEntry(mut entry: Arc<LookupTree::Entry::Entry>, mut tree: Arc<ClassTree>) -> Result<(Arc<InstNode::InstNode>, bool)> {
        let mut element: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut isImport: bool = false;
        (element, isImport) = (::match_deref::match_deref! { match &(entry.clone()) {
        Deref @ LookupTree::Entry::CLASS { .. } => (resolveClass(var_field!((*entry).index, LookupTree::Entry::Entry::CLASS).clone(), tree.clone())?, false),
        Deref @ LookupTree::Entry::COMPONENT { .. } => (resolveComponent(var_field!((*entry).index, LookupTree::Entry::Entry::COMPONENT).clone(), tree.clone())?, false),
        Deref @ LookupTree::Entry::IMPORT { .. } => (resolveImport(var_field!((*entry).index, LookupTree::Entry::Entry::IMPORT).clone(), tree.clone())?, true),
        _ => bail!("match: no arm matched"),
    } });
        Ok((element, isImport))
    }

    fn resolveEntryPtr(mut entry: Arc<LookupTree::Entry::Entry>, mut tree: Arc<ClassTree>) -> Result<Mutable::Mutable<Arc<InstNode::InstNode>>> {
        let mut element: Mutable::Mutable<Arc<InstNode::InstNode>>;
        let mut elems: metamodelica::Array<Mutable::Mutable<Arc<InstNode::InstNode>>>;
        element = (::match_deref::match_deref! { match &(entry.clone()) {
        Deref @ LookupTree::Entry::CLASS { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(tree.clone()) {
                Deref @ INSTANTIATED_TREE { classes: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            elems = __pa0.clone();
            elems.clone().borrow()[(var_field!((*entry).index, LookupTree::Entry::Entry::CLASS).clone()-1) as usize].clone()
        },
        Deref @ LookupTree::Entry::COMPONENT { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(tree.clone()) {
                Deref @ INSTANTIATED_TREE { components: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            elems = __pa0.clone();
            elems.clone().borrow()[(var_field!((*entry).index, LookupTree::Entry::Entry::COMPONENT).clone()-1) as usize].clone()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(element)
    }

    fn resolveDuplicateEntriesPtr(mut entry: Arc<DuplicateTree::Entry>, mut tree: Arc<ClassTree>, mut elements: Arc<metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>>) -> Result<Arc<metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>>> {
        let mut elements: Arc<metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>> = elements;
        let mut node_ptr: Mutable::Mutable<Arc<InstNode::InstNode>>;
        node_ptr = resolveEntryPtr(entry.entry.clone(), tree.clone())?;
        elements = cons(node_ptr.clone(), elements.clone());
        for mut child in &*entry.children.clone() {
            let mut child = child.clone();
            elements = resolveDuplicateEntriesPtr(child.clone(), tree.clone(), elements.clone())?;
        }
        Ok(elements)
    }

    fn resolveClass(mut index: i32, mut tree: Arc<ClassTree>) -> Result<Arc<InstNode::InstNode>> {
        let mut element: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        element = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ PARTIAL_TREE { .. } => var_field!((*tree).classes, ClassTree::PARTIAL_TREE).clone().borrow()[(index.clone()-1) as usize].clone(),
        Deref @ EXPANDED_TREE { .. } => var_field!((*tree).classes, ClassTree::EXPANDED_TREE).clone().borrow()[(index.clone()-1) as usize].clone(),
        Deref @ INSTANTIATED_TREE { .. } => Mutable::access(var_field!((*tree).classes, ClassTree::INSTANTIATED_TREE).clone().borrow()[(index.clone()-1) as usize].clone()),
        Deref @ FLAT_TREE { .. } => var_field!((*tree).classes, ClassTree::FLAT_TREE).clone().borrow()[(index.clone()-1) as usize].clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok(element)
    }

    fn resolveComponent(mut index: i32, mut tree: Arc<ClassTree>) -> Result<Arc<InstNode::InstNode>> {
        let mut element: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        element = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ INSTANTIATED_TREE { .. } => Mutable::access(var_field!((*tree).components, ClassTree::INSTANTIATED_TREE).clone().borrow()[(index.clone()-1) as usize].clone()),
        Deref @ FLAT_TREE { .. } => var_field!((*tree).components, ClassTree::FLAT_TREE).clone().borrow()[(index.clone()-1) as usize].clone(),
        _ => bail!("match: no arm matched"),
    } });
        Ok(element)
    }

    fn resolveImport(mut index: i32, mut tree: Arc<ClassTree>) -> Result<Arc<InstNode::InstNode>> {
        let mut element: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut imports: metamodelica::Array<Arc<Import::NFImport>>;
        let mut imp: Arc<Import::NFImport>;
        let mut changed: bool = false;
        imports = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ PARTIAL_TREE { .. } => var_field!((*tree).imports, ClassTree::PARTIAL_TREE).clone(),
        Deref @ EXPANDED_TREE { .. } => var_field!((*tree).imports, ClassTree::EXPANDED_TREE).clone(),
        Deref @ INSTANTIATED_TREE { .. } => var_field!((*tree).imports, ClassTree::INSTANTIATED_TREE).clone(),
        Deref @ FLAT_TREE { .. } => var_field!((*tree).imports, ClassTree::FLAT_TREE).clone(),
        _ => bail!("match: no arm matched"),
    } });
        (element, changed, imp) = Import::resolve(imports.borrow()[(index.clone()-1) as usize].clone())?;
        if changed.clone() {
            {let _arr = imports.clone(); _arr.borrow_mut()[(index.clone()-1) as usize] = imp.clone(); _arr};
        }
        Ok(element)
    }

    fn countElements(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> (i32, i32, i32) {
        let mut classCount: i32 = 0;
        let mut compCount: i32 = 0;
        let mut extCount: i32 = 0;
        for mut e in &*elements.clone() {
            let mut e = e.clone();
            let () = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ SCode::Element::CLASS { .. } => {
            classCount = classCount.clone() + 1;
            ()
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            compCount = compCount.clone() + 1;
            ()
        },
        Deref @ SCode::Element::EXTENDS { .. } => {
            extCount = extCount.clone() + 1;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        (classCount, compCount, extCount)
    }

    fn countInheritedElements(mut extendsNode: Arc<InstNode::InstNode>, mut classCount: i32, mut componentCount: i32) -> Result<(i32, i32)> {
        let mut classCount: i32 = classCount;
        let mut componentCount: i32 = componentCount;
        let mut clss: metamodelica::Array<Arc<InstNode::InstNode>>;
        let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
        let mut exts: metamodelica::Array<Arc<InstNode::InstNode>>;
        let () = (::match_deref::match_deref! { match &(Class::classTree(InstNode::getClass(extendsNode.clone())?)?) {
        Deref @ EXPANDED_TREE { exts, components: comps, classes: clss, .. } => {
            componentCount = componentCount.clone() + (comps.clone().borrow().len() as i32) - (exts.clone().borrow().len() as i32);
            classCount = classCount.clone() + (clss.clone().borrow().len() as i32);
            let __range0 = exts.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut ext in __range0 {
                (classCount, componentCount) = countInheritedElements(ext.clone(), classCount.clone(), componentCount.clone())?;
            }
            ()
        },
        Deref @ FLAT_TREE { components: comps, classes: clss, .. } => {
            componentCount = componentCount.clone() + (comps.clone().borrow().len() as i32);
            classCount = classCount.clone() + (clss.clone().borrow().len() as i32);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((classCount, componentCount))
    }

    fn expandExtends(mut extendsNode: Arc<InstNode::InstNode>, mut tree: Arc<LookupTree::Tree>, mut classOffset: i32, mut componentOffset: i32, mut duplicates: Mutable::Mutable<Arc<DuplicateTree::Tree>>) -> Result<Arc<LookupTree::Tree>> {
        let mut tree: Arc<LookupTree::Tree> = tree;
        let mut cls_tree: Arc<ClassTree> = Arc::new(ClassTree::EMPTY_TREE);
        let mut ext_tree: Arc<LookupTree::Tree> = Arc::new(LookupTree::Tree::EMPTY);
        let mut ext_dups: Arc<DuplicateTree::Tree> = Arc::new(DuplicateTree::Tree::EMPTY);
        let mut dups: Arc<DuplicateTree::Tree> = Arc::new(DuplicateTree::Tree::EMPTY);
        let mut conf_func: LookupTree::ConflictFunc;
        cls_tree = Class::classTree(InstNode::getClass(extendsNode.clone())?)?;
        (ext_tree, ext_dups) = (::match_deref::match_deref! { match &(cls_tree.clone()) {
        Deref @ EXPANDED_TREE { .. } => (var_field!((*cls_tree).tree, ClassTree::EXPANDED_TREE).clone(), var_field!((*cls_tree).duplicates, ClassTree::EXPANDED_TREE).clone()),
        Deref @ FLAT_TREE { .. } => (var_field!((*cls_tree).tree, ClassTree::FLAT_TREE).clone(), var_field!((*cls_tree).duplicates, ClassTree::FLAT_TREE).clone()),
        _ => {
            return Ok(tree);
            (tree.clone(), DuplicateTree::new())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if !(DuplicateTree::isEmpty(ext_dups.clone())) {
            dups = DuplicateTree::map(ext_dups.clone(), Arc::new({ let __pe_b2 = classOffset.clone(); let __pe_b3 = componentOffset.clone(); move |__pe_a0, __pe_a1| offsetDuplicates(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }));
            dups = DuplicateTree::join(Mutable::access(duplicates.clone()), dups.clone(), (std::sync::Arc::new(fnptr!(joinDuplicates, Arc<DuplicateTree::Entry>, Arc<DuplicateTree::Entry>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DuplicateTree::Entry>, Arc<DuplicateTree::Entry>, ArcStr) -> Result<Arc<DuplicateTree::Entry>> + 'static>))?;
            Mutable::update(duplicates.clone(), dups.clone());
        }
        conf_func = Arc::new({ let __pe_b3 = duplicates.clone(); let __pe_b4 = ext_dups.clone(); move |__pe_a0, __pe_a1, __pe_a2| addInheritedElementConflict(__pe_a0, __pe_a1, __pe_a2, __pe_b3.clone(), __pe_b4.clone()) });
        tree = LookupTree::fold(ext_tree.clone(), Arc::new({ let __pe_b2 = classOffset.clone(); let __pe_b3 = componentOffset.clone(); let __pe_b4 = conf_func.clone(); move |__pe_a0, __pe_a1, __pe_a5| addInheritedElement(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_a5) }), tree.clone());
        Ok(tree)
    }

    fn addInheritedElement(mut name: ArcStr, mut entry: Arc<LookupTree::Entry::Entry>, mut classOffset: i32, mut componentOffset: i32, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<LookupTree::Entry::Entry>, Arc<LookupTree::Entry::Entry>, ArcStr) -> Result<Arc<LookupTree::Entry::Entry>> + 'static>, mut tree: Arc<LookupTree::Tree>) -> Result<Arc<LookupTree::Tree>> {
        let mut tree: Arc<LookupTree::Tree> = tree;
        let () = (::match_deref::match_deref! { match &(entry.clone()) {
        Deref @ LookupTree::Entry::CLASS { .. } => {
            assign_variant_field!(entry => LookupTree::Entry::Entry::CLASS; index = var_field!((*entry).index, LookupTree::Entry::Entry::CLASS).clone() + classOffset.clone());
            tree = LookupTree::add(tree.clone(), (name.clone()).clone(), entry.clone(), conflictFunc.clone())?;
            ()
        },
        Deref @ LookupTree::Entry::COMPONENT { .. } => {
            assign_variant_field!(entry => LookupTree::Entry::Entry::COMPONENT; index = var_field!((*entry).index, LookupTree::Entry::Entry::COMPONENT).clone() + componentOffset.clone());
            tree = LookupTree::add(tree.clone(), (name.clone()).clone(), entry.clone(), conflictFunc.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(tree)
    }

    fn addInheritedElementConflict(mut newEntry: Arc<LookupTree::Entry::Entry>, mut oldEntry: Arc<LookupTree::Entry::Entry>, mut name: ArcStr, mut duplicates: Mutable::Mutable<Arc<DuplicateTree::Tree>>, mut extDuplicates: Arc<DuplicateTree::Tree>) -> Result<Arc<LookupTree::Entry::Entry>> {
        let mut entry: Arc<LookupTree::Entry::Entry>;
        let mut dups: Arc<DuplicateTree::Tree> = Arc::new(DuplicateTree::Tree::EMPTY);
        let mut opt_dup_entry: Option<Arc<DuplicateTree::Entry>> = None;
        let mut dup_entry: Arc<DuplicateTree::Entry>;
        let mut new_id: i32 = LookupTree::Entry::index(newEntry.clone())?;
        let mut old_id: i32 = LookupTree::Entry::index(oldEntry.clone())?;
        let mut ty: DuplicateTree::EntryType = DuplicateTree::EntryType::DUPLICATE;
        if LookupTree::Entry::isImport(oldEntry.clone()) {
            entry = newEntry.clone();
            return Ok(entry);
        }
        dups = Mutable::access(duplicates.clone());
        opt_dup_entry = DuplicateTree::getOpt(dups.clone(), (name.clone()).clone());
        if isNone(opt_dup_entry.clone()) {
            if new_id.clone() < old_id.clone() {
                entry = newEntry.clone();
                dup_entry = DuplicateTree::newDuplicate(newEntry.clone(), oldEntry.clone());
            } else {
                entry = oldEntry.clone();
                dup_entry = DuplicateTree::newDuplicate(oldEntry.clone(), newEntry.clone());
            }
            dups = DuplicateTree::add(dups.clone(), (name.clone()).clone(), dup_entry.clone(), (std::sync::Arc::new(DuplicateTree::addConflictDefault) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
            Mutable::update(duplicates.clone(), dups.clone());
        } else {
            let __pa0 = ::match_deref::match_deref! { match &(opt_dup_entry.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            dup_entry = __pa0.clone();
            ty = dup_entry.ty.clone();
            if !(DuplicateTree::idExistsInEntry(newEntry.clone(), dup_entry.clone())) {
                if ty.clone() == DuplicateTree::EntryType::REDECLARE.clone() {
                    entry = newEntry.clone();
                    assign_field!(dup_entry.children = cons(DuplicateTree::newEntry(newEntry.clone()), dup_entry.children.clone()));
                } else {
                    if new_id.clone() < old_id.clone() {
                        entry = newEntry.clone();
                        dup_entry = Arc::new(DuplicateTree::Entry { entry: newEntry.clone(), node: None, children: cons(DuplicateTree::newEntry(oldEntry.clone()), dup_entry.children.clone()), ty: dup_entry.ty.clone() });
                    } else {
                        entry = oldEntry.clone();
                        assign_field!(dup_entry.children = cons(DuplicateTree::newEntry(newEntry.clone()), dup_entry.children.clone()));
                    }
                }
                dups = DuplicateTree::update(dups.clone(), (name.clone()).clone(), dup_entry.clone());
                Mutable::update(duplicates.clone(), dups.clone());
            } else if !(DuplicateTree::idExistsInEntry(oldEntry.clone(), dup_entry.clone())) {
                if ty.clone() == DuplicateTree::EntryType::REDECLARE.clone() || new_id.clone() < old_id.clone() {
                    entry = newEntry.clone();
                    assign_field!(dup_entry.children = cons(DuplicateTree::newEntry(oldEntry.clone()), dup_entry.children.clone()));
                } else {
                    entry = newEntry.clone();
                    dup_entry = Arc::new(DuplicateTree::Entry { entry: newEntry.clone(), node: None, children: cons(DuplicateTree::newEntry(oldEntry.clone()), dup_entry.children.clone()), ty: dup_entry.ty.clone() });
                }
                dups = DuplicateTree::update(dups.clone(), (name.clone()).clone(), dup_entry.clone());
                Mutable::update(duplicates.clone(), dups.clone());
            } else {
                entry = if (new_id.clone() < old_id.clone()) {newEntry.clone()} else {oldEntry.clone()};
            }
        }
        Ok(entry)
    }

    fn offsetDuplicates(mut name: ArcStr, mut entry: Arc<DuplicateTree::Entry>, mut classOffset: i32, mut componentOffset: i32) -> Result<Arc<DuplicateTree::Entry>> {
        let mut offsetEntry: Arc<DuplicateTree::Entry>;
        let mut parent: Arc<LookupTree::Entry::Entry>;
        let mut children: Arc<metamodelica::List<Arc<DuplicateTree::Entry>>> = metamodelica::nil();
        parent = offsetDuplicate(entry.entry.clone(), classOffset.clone(), componentOffset.clone())?;
        children = {
        let mut __acc: Arc<metamodelica::List<Arc<DuplicateTree::Entry>>> = metamodelica::nil();
        for mut c in (entry.children.clone()).into_iter().cloned() {
            let __x = offsetDuplicates((name.clone()).clone(), c.clone(), classOffset.clone(), componentOffset.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        offsetEntry = Arc::new(DuplicateTree::Entry { entry: parent.clone(), node: None, children: children.clone(), ty: entry.ty.clone() });
        Ok(offsetEntry)
    }

    fn offsetDuplicate(mut entry: Arc<LookupTree::Entry::Entry>, mut classOffset: i32, mut componentOffset: i32) -> Result<Arc<LookupTree::Entry::Entry>> {
        let mut offsetEntry: Arc<LookupTree::Entry::Entry>;
        offsetEntry = (::match_deref::match_deref! { match &(entry.clone()) {
        Deref @ LookupTree::Entry::CLASS { .. } => Arc::new(LookupTree::Entry::Entry::CLASS { index: var_field!((*entry).index, LookupTree::Entry::Entry::CLASS).clone() + classOffset.clone() }),
        Deref @ LookupTree::Entry::COMPONENT { .. } => Arc::new(LookupTree::Entry::Entry::COMPONENT { index: var_field!((*entry).index, LookupTree::Entry::Entry::COMPONENT).clone() + componentOffset.clone() }),
        _ => bail!("match: no arm matched"),
    } });
        Ok(offsetEntry)
    }

    fn joinDuplicates(mut newEntry: Arc<DuplicateTree::Entry>, mut oldEntry: Arc<DuplicateTree::Entry>, mut name: ArcStr) -> Arc<DuplicateTree::Entry> {
        let mut entry: Arc<DuplicateTree::Entry> = oldEntry.clone();
        assign_field!(entry.children = cons(newEntry.clone(), entry.children.clone()));
        entry
    }

    fn enumerateDuplicates(mut duplicates: Arc<DuplicateTree::Tree>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
        let mut classes: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut components: Arc<metamodelica::List<i32>> = metamodelica::nil();
        if DuplicateTree::isEmpty(duplicates.clone()) {
            classes = metamodelica::nil();
            components = metamodelica::nil();
        } else {
            (classes, components) = DuplicateTree::fold_2(duplicates.clone(), (std::sync::Arc::new(enumerateDuplicates2) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DuplicateTree::Entry>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> + 'static>), metamodelica::nil(), metamodelica::nil());
            classes = List::sort(classes.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            components = List::sort(components.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        }
        Ok((classes, components))
    }

    fn enumerateDuplicates2(mut name: ArcStr, mut entry: Arc<DuplicateTree::Entry>, mut classes: Arc<metamodelica::List<i32>>, mut components: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
        let mut classes: Arc<metamodelica::List<i32>> = classes;
        let mut components: Arc<metamodelica::List<i32>> = components;
        for mut c in &*entry.children.clone() {
            let mut c = c.clone();
            (classes, components) = enumerateDuplicates3(c.clone(), classes.clone(), components.clone())?;
        }
        Ok((classes, components))
    }

    fn enumerateDuplicates3(mut entry: Arc<DuplicateTree::Entry>, mut classes: Arc<metamodelica::List<i32>>, mut components: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
        let mut classes: Arc<metamodelica::List<i32>> = classes;
        let mut components: Arc<metamodelica::List<i32>> = components;
        (classes, components) = enumerateDuplicates4(entry.entry.clone(), classes.clone(), components.clone())?;
        for mut c in &*entry.children.clone() {
            let mut c = c.clone();
            (classes, components) = enumerateDuplicates3(c.clone(), classes.clone(), components.clone())?;
        }
        Ok((classes, components))
    }

    fn enumerateDuplicates4(mut entry: Arc<LookupTree::Entry::Entry>, mut classes: Arc<metamodelica::List<i32>>, mut components: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
        let mut classes: Arc<metamodelica::List<i32>> = classes;
        let mut components: Arc<metamodelica::List<i32>> = components;
        let () = (::match_deref::match_deref! { match &(entry.clone()) {
        Deref @ LookupTree::Entry::CLASS { .. } => (),
        Deref @ LookupTree::Entry::COMPONENT { .. } => {
            components = cons(var_field!((*entry).index, LookupTree::Entry::Entry::COMPONENT).clone(), components.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok((classes, components))
    }

    fn mapRedeclareChain(mut name: ArcStr, mut entry: Arc<DuplicateTree::Entry>, mut func: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>>) -> Result<()> + 'static>, mut tree: Arc<ClassTree>) -> Result<Arc<DuplicateTree::Entry>> {
        pub type FuncT = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>>) -> Result<()> + 'static>;

        let mut entry: Arc<DuplicateTree::Entry> = entry;
        let mut chain: Arc<metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>> = metamodelica::nil();
        chain = getRedeclareChain(entry.clone(), tree.clone(), metamodelica::nil())?;
        if !(chain.clone().is_empty()) {
            func(chain.clone())?;
        }
        Ok(entry)
    }

    fn getRedeclareChain(mut entry: Arc<DuplicateTree::Entry>, mut tree: Arc<ClassTree>, mut chain: Arc<metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>>) -> Result<Arc<metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>>> {
        let mut chain: Arc<metamodelica::List<Mutable::Mutable<Arc<InstNode::InstNode>>>> = chain;
        chain = (match entry.ty.clone() {
        DuplicateTree::EntryType::REDECLARE => {
            let mut node_ptr: Mutable::Mutable<Arc<InstNode::InstNode>>;
            let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
            node_ptr = resolveEntryPtr(entry.entry.clone(), tree.clone())?;
            if entry.children.clone().is_empty() {
                node = Mutable::access(node_ptr.clone());
                if SCodeUtil::isClassExtends(InstNode::definition(node.clone())?) {
                    Error::addSourceMessage(Error::CLASS_EXTENDS_TARGET_NOT_FOUND.clone(), list![(InstNode::name(node.clone())?).clone()], InstNode::info(node.clone())?)?;
                } else {
                    Error::addSourceMessage(Error::REDECLARE_NONEXISTING_ELEMENT.clone(), list![(InstNode::name(node.clone())?).clone()], InstNode::info(node.clone())?)?;
                }
                bail!("fail");
            }
            getRedeclareChain(listHead(entry.children.clone())?, tree.clone(), cons(node_ptr.clone(), chain.clone()))?
        },
        DuplicateTree::EntryType::ENTRY => {
            let mut node_ptr: Mutable::Mutable<Arc<InstNode::InstNode>>;
            node_ptr = resolveEntryPtr(entry.entry.clone(), tree.clone())?;
            cons(node_ptr.clone(), chain.clone())
        },
        _ => {
            chain.clone()
        },
    });
        Ok(chain)
    }

    fn replaceDuplicates2(mut name: ArcStr, mut entry: Arc<DuplicateTree::Entry>, mut tree: Arc<ClassTree>) -> Result<(Arc<DuplicateTree::Entry>, Arc<ClassTree>)> {
        let mut entry: Arc<DuplicateTree::Entry> = entry;
        let mut tree: Arc<ClassTree> = tree;
        let mut kept: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut node_ptr: Mutable::Mutable<Arc<InstNode::InstNode>>;
        let mut node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut entries: Arc<metamodelica::List<Arc<DuplicateTree::Entry>>> = metamodelica::nil();
        let mut broken_entries: Arc<metamodelica::List<Arc<DuplicateTree::Entry>>> = metamodelica::nil();
        let () = (match entry.ty.clone() {
        DuplicateTree::EntryType::REDECLARE => {
            kept = Mutable::access(resolveEntryPtr(entry.entry.clone(), tree.clone())?);
            entry = replaceDuplicates3(entry.clone(), kept.clone());
            ()
        },
        DuplicateTree::EntryType::DUPLICATE => {
            entries = metamodelica::nil();
            broken_entries = metamodelica::nil();
            kept = Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE);
            for mut e in &*DuplicateTree::entryToList(entry.clone()) {
                let mut e = e.clone();
                node_ptr = resolveEntryPtr(e.entry.clone(), tree.clone())?;
                node = Mutable::access(node_ptr.clone());
                assign_field!(
                    e.node = Some(node.clone()),
                    e.children = metamodelica::nil()
                );
                if !(InstNode::isEmpty(node.clone())) {
                    if InstNode::isEmpty(kept.clone()) {
                        kept = node.clone();
                    }
                    entries = cons(e.clone(), entries.clone());
                } else {
                    broken_entries = cons(e.clone(), broken_entries.clone());
                }
            }
            for mut e in &*entries.clone() {
                let mut e = e.clone();
                node_ptr = resolveEntryPtr(e.entry.clone(), tree.clone())?;
                Mutable::update(node_ptr.clone(), kept.clone());
            }
            if entries.clone().is_empty() {
                assign_field!(
                    entry.node = None,
                    entry.children = metamodelica::nil()
                );
                return Ok((entry, tree));
            } else {
                entries = entries.clone().reverse();
                entry = listHead(entries.clone())?;
                assign_field!(entry.children = listAppend(listRest(entries.clone())?, broken_entries.clone()));
            }
            ()
        },
        _ => (),
    });
        Ok((entry, tree))
    }

    fn replaceDuplicates3(mut entry: Arc<DuplicateTree::Entry>, mut node: Arc<InstNode::InstNode>) -> Arc<DuplicateTree::Entry> {
        let mut entry: Arc<DuplicateTree::Entry> = entry;
        assign_field!(
            entry.node = Some(node.clone()),
            entry.children = {
        let mut __acc: Arc<metamodelica::List<Arc<DuplicateTree::Entry>>> = metamodelica::nil();
        for mut c in (entry.children.clone()).into_iter().cloned() {
            let __x = replaceDuplicates3(c.clone(), node.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
        );
        entry
    }

    fn linkInnerOuter(mut outerNode: Arc<InstNode::InstNode>, mut scope: Arc<InstNode::InstNode>) -> Result<Arc<InstNode::InstNode>> {
        let mut innerOuterNode: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut inner_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        inner_node = Lookup::lookupInner(outerNode.clone(), scope.clone())?;
        if metamodelica::valueConstructor((&*outerNode.clone()))? != metamodelica::valueConstructor((&*inner_node.clone()))? {
            Error::addMultiSourceMessage(Error::FOUND_WRONG_INNER_ELEMENT.clone(), list![(InstNode::typeName(inner_node.clone())?).clone(), (InstNode::name(outerNode.clone())?).clone(), (InstNode::typeName(outerNode.clone())?).clone()], list![InstNode::info(outerNode.clone())?, InstNode::info(inner_node.clone())?])?;
            bail!("fail");
        }
        innerOuterNode = Arc::new(InstNode::InstNode::INNER_OUTER_NODE { innerNode: inner_node.clone(), outerNode: outerNode.clone() });
        Ok(innerOuterNode)
    }

    fn checkOuterClass(mut outerCls: Arc<InstNode::InstNode>) -> Result<()> {
        let mut def: Arc<SCode::ClassDef>;
        if InstNode::isOnlyOuter(outerCls.clone())? {
            def = SCodeUtil::getClassDef(InstNode::definition(outerCls.clone())?)?;
            let () = (::match_deref::match_deref! { match &(def.clone()) {
        Deref @ SCode::ClassDef::DERIVED { modifications: Deref @ SCode::Mod::NOMOD { .. }, .. } => (),
        Deref @ SCode::ClassDef::DERIVED { .. } => {
            Error::addSourceMessage(Error::OUTER_ELEMENT_MOD.clone(), list![(SCodeDump::printModStr(var_field!((*def).modifications, SCode::ClassDef::DERIVED).clone(), SCodeDump::defaultOptions.clone())?).clone(), (InstNode::name(outerCls.clone())?).clone()], InstNode::info(outerCls.clone())?)?;
            bail!("fail")
        },
        _ => {
            Error::addSourceMessage(Error::OUTER_LONG_CLASS.clone(), list![(InstNode::name(outerCls.clone())?).clone()], InstNode::info(outerCls.clone())?)?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        Ok(())
    }

    fn getBreakModsInExtend(mut extendsNode: Arc<InstNode::InstNode>) -> Result<Arc<metamodelica::List<Arc<SCode::SubMod>>>> {
        let mut breaks: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
        let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
        let mut opt_def: Option<Arc<SCode::Element>> = None;
        opt_def = InstNode::extendsDefinition(extendsNode.clone())?;
        breaks = (::match_deref::match_deref! { match &(opt_def.clone()) {
        Some(Deref @ SCode::Element::EXTENDS { modifications: r#mod @ Deref @ SCode::Mod::MOD { .. }, .. }) => {
        let mut __acc: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
        for mut sm in (var_field!((**r#mod).subModLst, SCode::Mod::MOD).clone()).into_iter().cloned() {
            if !(SCodeUtil::isBreakComponentSubMod(sm.clone())) { continue; }
            let __x = sm.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(breaks)
    }

    fn breakComponents(mut node: Arc<InstNode::InstNode>, mut components: metamodelica::Array<Mutable::Mutable<Arc<InstNode::InstNode>>>, mut tree: Arc<LookupTree::Tree>, mut duplicates: Arc<DuplicateTree::Tree>) -> Result<()> {
        let mut break_mods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
        let mut opt_dentry: Option<Arc<DuplicateTree::Entry>> = None;
        let mut opt_lentry: Option<Arc<LookupTree::Entry::Entry>> = None;
        let mut entries: Arc<metamodelica::List<Arc<LookupTree::Entry::Entry>>> = metamodelica::nil();
        let mut index: i32 = 0;
        let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
        break_mods = getBreakModsInExtend(node.clone())?;
        if break_mods.clone().is_empty() {
            return Ok(());
        }
        for mut bm in &*break_mods.clone() {
            let mut bm = bm.clone();
            info = SCodeUtil::getModifierInfo(bm.r#mod.clone());
            opt_dentry = DuplicateTree::getOpt(duplicates.clone(), (bm.ident.clone()).clone());
            if isSome(opt_dentry.clone()) {
                entries = DuplicateTree::getLookupEntries(Util::getOption(opt_dentry.clone())?);
            } else {
                opt_lentry = LookupTree::getOpt(tree.clone(), (bm.ident.clone()).clone());
                entries = if (isSome(opt_lentry.clone())) {list![Util::getOption(opt_lentry.clone())?]} else {metamodelica::nil()};
            }
            if entries.clone().is_empty() || List::all(entries.clone(), (std::sync::Arc::new(fnptr!(LookupTree::Entry::isImport, Arc<LookupTree::Entry::Entry>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<LookupTree::Entry::Entry>) -> Result<bool> + 'static>)) {
                Error::addSourceMessage(Error::MISSING_MODIFIED_ELEMENT.clone(), list![(bm.ident.clone()).clone(), (InstNode::name(node.clone())?).clone()], info.clone())?;
                bail!("fail");
            }
            for mut e in &*entries.clone() {
                let mut e = e.clone();
                index = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ LookupTree::Entry::COMPONENT { .. } => var_field!((*e).index, LookupTree::Entry::Entry::COMPONENT).clone(),
        _ => {
            Error::addSourceMessage(Error::NON_BREAKABLE_ELEMENT.clone(), list![(bm.ident.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                checkIsBreakable(Mutable::access(components.borrow()[(index.clone()-1) as usize].clone()), node.clone(), info.clone())?;
                Mutable::update(components.borrow()[(index.clone()-1) as usize].clone(), Arc::new(crate::NFInstNode::InstNode::EMPTY_NODE));
            }
        }
        Ok(())
    }

    fn checkIsBreakable(mut node: Arc<InstNode::InstNode>, mut scope: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<()> {
        let mut ty_path: Arc<Absyn::Path>;
        let mut cls_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
        let mut restriction: SCode::Restriction = SCode::Restriction::R_BLOCK;
        match '__try0: {
            ty_path = unwrap_break_err!(SCodeUtil::getElementTypePath(InstNode::definition(node.clone())?), '__try0);
            (cls_node, _) = unwrap_break_err!(Lookup::lookupName(ty_path.clone(), scope.clone(), NFInstContext::NO_CONTEXT.clone(), false), '__try0);
            restriction = unwrap_break_err!(SCodeUtil::getClassRestriction(InstNode::definition(cls_node.clone())?), '__try0);
            Ok::<_, anyhow::Error>((restriction.clone(),))
        } {
            Ok((__try0_o0,)) => {
                restriction = __try0_o0;
            }
            Err(_) => {
                restriction = openmodelica_frontend_types::SCode::Restriction::R_CLASS;
            }
        }
        let () = (match restriction.clone() {
        SCode::Restriction::R_MODEL { .. } => (),
        SCode::Restriction::R_BLOCK { .. } => (),
        SCode::Restriction::R_CONNECTOR { .. } => (),
        _ => {
            Error::addMultiSourceMessage(Error::NON_BREAKABLE_COMPONENT.clone(), list![(InstNode::name(node.clone())?).clone()], list![info.clone(), InstNode::info(node.clone())?])?;
            bail!("fail")
        },
    });
        Ok(())
    }

}

