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

use crate::Error;
use crate::SBSet;
use crate::StringUtil;
use crate::Vector;
use openmodelica_util_datatypes_basic::List;

pub type VertexEq<VertexT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(VertexT, VertexT) -> Result<bool> + 'static>;

pub type EdgeEq<EdgeT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(EdgeT, EdgeT) -> Result<bool> + 'static>;

pub type VertexStr<VertexT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(VertexT) -> Result<ArcStr> + 'static>;

pub type EdgeStr<EdgeT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(EdgeT) -> Result<ArcStr> + 'static>;

pub type VertexDescriptor = i32;

// types of sets
// V - generic vertex set (F u U)
// F - function/equation vertex set
// U - unknown/variable vertex set
// E - edge set
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum SetType {
    V = 1,
    F = 2,
    U = 3,
    E = 4,
}
impl PartialOrd for SetType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for SetType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn edge_finder<EdgeT: Clone + 'static>(mut index: i32, mut e: EdgeT, mut edges: Arc<Vector::Vector<EdgeT>>, mut eqFn: Arc<dyn ::std::ops::Fn(EdgeT, EdgeT) -> Result<bool> + 'static>) -> bool {
    let mut matching: bool = eqFn(e.clone(), Vector::get(edges.clone(), index.clone()).unwrap()).unwrap();
    matching
}

pub mod IncidenceList {
    use super::*;
#[derive(Clone)]
    pub struct IncidenceList<VertexT: Clone, EdgeT: Clone> {
        pub vertices: Arc<Vector::Vector<VertexT>>,
        pub edges: Arc<Vector::Vector<EdgeT>>,
        pub graph: Arc<Vector::Vector<Arc<metamodelica::List<i32>>>>,
        pub vertEqFn: VertexEq<VertexT>,
        pub edgeEqFn: EdgeEq<EdgeT>,
        pub vertToString: VertexStr<VertexT>,
        pub edgeToString: EdgeStr<EdgeT>,
    }

    impl<VertexT: Clone + 'static + PartialEq, EdgeT: Clone + 'static + PartialEq> PartialEq for IncidenceList<VertexT, EdgeT> {
        fn eq(&self, other: &Self) -> bool {
            self.vertices == other.vertices && self.edges == other.edges && self.graph == other.graph && std::sync::Arc::ptr_eq((&self.vertEqFn), (&other.vertEqFn)) && std::sync::Arc::ptr_eq((&self.edgeEqFn), (&other.edgeEqFn)) && std::sync::Arc::ptr_eq((&self.vertToString), (&other.vertToString)) && std::sync::Arc::ptr_eq((&self.edgeToString), (&other.edgeToString))
        }
    }
    impl<VertexT: Clone + 'static + PartialEq + Eq, EdgeT: Clone + 'static + PartialEq + Eq> Eq for IncidenceList<VertexT, EdgeT> {}
    impl<VertexT: Clone + 'static + PartialEq + Eq + PartialOrd + Ord, EdgeT: Clone + 'static + PartialEq + Eq + PartialOrd + Ord> PartialOrd for IncidenceList<VertexT, EdgeT> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
    }
    impl<VertexT: Clone + 'static + PartialEq + Eq + PartialOrd + Ord, EdgeT: Clone + 'static + PartialEq + Eq + PartialOrd + Ord> Ord for IncidenceList<VertexT, EdgeT> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.vertices.cmp(&other.vertices).then_with(|| self.edges.cmp(&other.edges).then_with(|| self.graph.cmp(&other.graph).then_with(|| (std::sync::Arc::as_ptr((&self.vertEqFn)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.vertEqFn)) as *const ())).then_with(|| (std::sync::Arc::as_ptr((&self.edgeEqFn)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.edgeEqFn)) as *const ())).then_with(|| (std::sync::Arc::as_ptr((&self.vertToString)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.vertToString)) as *const ())).then_with(|| (std::sync::Arc::as_ptr((&self.edgeToString)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.edgeToString)) as *const ()))))))))
        }
    }
    impl<VertexT: Clone + 'static + std::fmt::Debug, EdgeT: Clone + 'static + std::fmt::Debug> std::fmt::Debug for IncidenceList<VertexT, EdgeT> {
        fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut __ds = __f.debug_struct("IncidenceList");
            __ds.field("vertices", &self.vertices);
            __ds.field("edges", &self.edges);
            __ds.field("graph", &self.graph);
            __ds.field("vertEqFn", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.vertEqFn))));
            __ds.field("edgeEqFn", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.edgeEqFn))));
            __ds.field("vertToString", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.vertToString))));
            __ds.field("edgeToString", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.edgeToString))));
            __ds.finish()
        }
    }

    pub type INCIDENCE_LIST<VertexT, EdgeT> = IncidenceList<VertexT, EdgeT>;

    pub fn new<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut vertexEq: Arc<dyn ::std::ops::Fn(VertexT, VertexT) -> Result<bool> + 'static>, mut edgeEq: Arc<dyn ::std::ops::Fn(EdgeT, EdgeT) -> Result<bool> + 'static>, mut vertexStr: Arc<dyn ::std::ops::Fn(VertexT) -> Result<ArcStr> + 'static>, mut edgeStr: Arc<dyn ::std::ops::Fn(EdgeT) -> Result<ArcStr> + 'static>) -> Arc<IncidenceList<VertexT, EdgeT>> {
        let mut il: Arc<IncidenceList<VertexT, EdgeT>>;
        il = Arc::new(IncidenceList { vertices: Vector::new(0), edges: Vector::new(0), graph: Vector::new(0), vertEqFn: vertexEq.clone(), edgeEqFn: edgeEq.clone(), vertToString: vertexStr.clone(), edgeToString: edgeStr.clone() });
        il
    }

    pub fn getRow<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<IncidenceList<VertexT, EdgeT>>, mut d: i32) -> Result<Arc<metamodelica::List<i32>>> {
        let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
        row = Vector::get(il.graph.clone(), d.clone())?;
        Ok(row)
    }

    pub fn addVertex<VertexT: Clone + 'static + Default, EdgeT: Clone + 'static>(mut il: Arc<IncidenceList<VertexT, EdgeT>>, mut v: VertexT) -> i32 {
        let mut d: i32 = 0;
        Vector::push(il.vertices.clone(), v.clone());
        Vector::push(il.graph.clone(), metamodelica::nil());
        d = Vector::size(il.vertices.clone());
        d
    }

    pub fn findVertex<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<IncidenceList<VertexT, EdgeT>>, mut predFn: Arc<dyn ::std::ops::Fn(VertexT) -> Result<bool> + 'static>) -> Option<i32> {
        pub type PredFn<VertexT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(VertexT) -> Result<bool> + 'static>;

        let mut od: Option<i32> = None;
        let mut index: i32 = 0;
        (_, index) = Vector::find(il.vertices.clone(), predFn.clone());
        od = if (index.clone() > 0) {Some(index.clone())} else {None};
        od
    }

    pub fn getVertex<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<IncidenceList<VertexT, EdgeT>>, mut d: i32) -> Result<VertexT> {
        let mut v: VertexT;
        v = Vector::get(il.vertices.clone(), d.clone())?;
        Ok(v)
    }

    pub fn getVerticesFromSet<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<IncidenceList<VertexT, EdgeT>>, mut set: Arc<SBSet::SBSet>, mut getSet: Arc<dyn ::std::ops::Fn(VertexT) -> Result<Arc<SBSet::SBSet>> + 'static>) -> Result<Arc<metamodelica::List<VertexT>>> {
        pub type getSetFn<VertexT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(VertexT) -> Result<Arc<SBSet::SBSet>> + 'static>;

        let mut set_vertices: Arc<metamodelica::List<VertexT>> = metamodelica::nil();
        for mut v in &*vertices(il.clone()) {
            let mut v = v.clone();
            if !(SBSet::isEmpty(SBSet::intersection(getSet(v.clone())?, set.clone())?)) {
                set_vertices = cons(v.clone(), set_vertices.clone());
            }
        }
        Ok(set_vertices)
    }

    pub fn addEdge<VertexT: Clone + 'static, EdgeT: Clone + 'static + Default>(mut il: Arc<IncidenceList<VertexT, EdgeT>>, mut d1: i32, mut d2: i32, mut e: EdgeT) -> Result<i32> {
        let mut ei: i32 = 0;
        let mut eil: Arc<metamodelica::List<i32>> = metamodelica::nil();
        eil = Vector::get(il.graph.clone(), d1.clone())?;
        ei = List::positionOnTrue(eil.clone(), Arc::new({ let __pe_b1 = e.clone(); let __pe_b2 = il.edges.clone(); let __pe_b3 = il.edgeEqFn.clone(); move |__pe_a0| Ok(edge_finder(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())) }));
        if ei.clone() == -1 {
            Vector::push(il.edges.clone(), e.clone());
            ei = Vector::size(il.edges.clone());
            Vector::update(il.graph.clone(), d1.clone(), cons(ei.clone(), eil.clone()))?;
            Vector::update(il.graph.clone(), d2.clone(), cons(ei.clone(), Vector::get(il.graph.clone(), d2.clone())?))?;
        } else {
            Vector::update(il.edges.clone(), ei.clone(), e.clone())?;
        }
        Ok(ei)
    }

    pub fn getEdge<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<IncidenceList<VertexT, EdgeT>>, mut d: i32) -> Result<EdgeT> {
        let mut e: EdgeT;
        e = Vector::get(il.edges.clone(), d.clone())?;
        Ok(e)
    }

    pub fn isEmpty<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<IncidenceList<VertexT, EdgeT>>) -> bool {
        let mut empty: bool = Vector::size(il.vertices.clone()) == 0;
        empty
    }

    pub fn vertexCount<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<IncidenceList<VertexT, EdgeT>>) -> i32 {
        let mut count: i32 = Vector::size(il.vertices.clone());
        count
    }

    pub fn edgeCount<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<IncidenceList<VertexT, EdgeT>>) -> i32 {
        let mut count: i32 = Vector::size(il.edges.clone());
        count
    }

    pub fn vertices<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<IncidenceList<VertexT, EdgeT>>) -> Arc<metamodelica::List<VertexT>> {
        let mut vl: Arc<metamodelica::List<VertexT>> = Vector::toList(il.vertices.clone());
        vl
    }

    pub fn edges<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<IncidenceList<VertexT, EdgeT>>) -> Arc<metamodelica::List<EdgeT>> {
        let mut el: Arc<metamodelica::List<EdgeT>> = Vector::toList(il.edges.clone());
        el
    }

    pub fn toString<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<IncidenceList<VertexT, EdgeT>>) -> ArcStr {
        let mut r#str: ArcStr = arcstr::literal!("");
        let mut vertToString: VertexStr<VertexT> = il.vertToString.clone();
        let mut edgeToString: EdgeStr<EdgeT> = il.edgeToString.clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2((literal!("Set-Based Graph")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (Vector::toList(il.vertices.clone())).into_iter().cloned() {
            let __x = vertToString(v.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (Vector::toList(il.edges.clone())).into_iter().cloned() {
            let __x = edgeToString(e.clone()).unwrap();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        r#str
    }

}

pub mod BipartiteIncidenceList {
    use super::*;
#[derive(Clone)]
    pub struct BipartiteIncidenceList<VertexT: Clone, EdgeT: Clone> {
        pub F_vertices: Arc<Vector::Vector<VertexT>>,
        pub U_vertices: Arc<Vector::Vector<VertexT>>,
        pub edges: Arc<Vector::Vector<EdgeT>>,
        pub graph: Arc<Vector::Vector<Arc<metamodelica::List<i32>>>>,
        pub vertEqFn: VertexEq<VertexT>,
        pub edgeEqFn: EdgeEq<EdgeT>,
        pub vertToString: VertexStr<VertexT>,
        pub edgeToString: EdgeStr<EdgeT>,
    }

    impl<VertexT: Clone + 'static + PartialEq, EdgeT: Clone + 'static + PartialEq> PartialEq for BipartiteIncidenceList<VertexT, EdgeT> {
        fn eq(&self, other: &Self) -> bool {
            self.F_vertices == other.F_vertices && self.U_vertices == other.U_vertices && self.edges == other.edges && self.graph == other.graph && std::sync::Arc::ptr_eq((&self.vertEqFn), (&other.vertEqFn)) && std::sync::Arc::ptr_eq((&self.edgeEqFn), (&other.edgeEqFn)) && std::sync::Arc::ptr_eq((&self.vertToString), (&other.vertToString)) && std::sync::Arc::ptr_eq((&self.edgeToString), (&other.edgeToString))
        }
    }
    impl<VertexT: Clone + 'static + PartialEq + Eq, EdgeT: Clone + 'static + PartialEq + Eq> Eq for BipartiteIncidenceList<VertexT, EdgeT> {}
    impl<VertexT: Clone + 'static + PartialEq + Eq + PartialOrd + Ord, EdgeT: Clone + 'static + PartialEq + Eq + PartialOrd + Ord> PartialOrd for BipartiteIncidenceList<VertexT, EdgeT> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
    }
    impl<VertexT: Clone + 'static + PartialEq + Eq + PartialOrd + Ord, EdgeT: Clone + 'static + PartialEq + Eq + PartialOrd + Ord> Ord for BipartiteIncidenceList<VertexT, EdgeT> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.F_vertices.cmp(&other.F_vertices).then_with(|| self.U_vertices.cmp(&other.U_vertices).then_with(|| self.edges.cmp(&other.edges).then_with(|| self.graph.cmp(&other.graph).then_with(|| (std::sync::Arc::as_ptr((&self.vertEqFn)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.vertEqFn)) as *const ())).then_with(|| (std::sync::Arc::as_ptr((&self.edgeEqFn)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.edgeEqFn)) as *const ())).then_with(|| (std::sync::Arc::as_ptr((&self.vertToString)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.vertToString)) as *const ())).then_with(|| (std::sync::Arc::as_ptr((&self.edgeToString)) as *const ()).cmp(&(std::sync::Arc::as_ptr((&other.edgeToString)) as *const ())))))))))
        }
    }
    impl<VertexT: Clone + 'static + std::fmt::Debug, EdgeT: Clone + 'static + std::fmt::Debug> std::fmt::Debug for BipartiteIncidenceList<VertexT, EdgeT> {
        fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut __ds = __f.debug_struct("BipartiteIncidenceList");
            __ds.field("F_vertices", &self.F_vertices);
            __ds.field("U_vertices", &self.U_vertices);
            __ds.field("edges", &self.edges);
            __ds.field("graph", &self.graph);
            __ds.field("vertEqFn", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.vertEqFn))));
            __ds.field("edgeEqFn", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.edgeEqFn))));
            __ds.field("vertToString", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.vertToString))));
            __ds.field("edgeToString", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.edgeToString))));
            __ds.finish()
        }
    }

    pub type BIPARTITE_INCIDENCE_LIST<VertexT, EdgeT> = BipartiteIncidenceList<VertexT, EdgeT>;

    pub fn new<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut vertexEq: Arc<dyn ::std::ops::Fn(VertexT, VertexT) -> Result<bool> + 'static>, mut edgeEq: Arc<dyn ::std::ops::Fn(EdgeT, EdgeT) -> Result<bool> + 'static>, mut vertexStr: Arc<dyn ::std::ops::Fn(VertexT) -> Result<ArcStr> + 'static>, mut edgeStr: Arc<dyn ::std::ops::Fn(EdgeT) -> Result<ArcStr> + 'static>) -> Arc<BipartiteIncidenceList<VertexT, EdgeT>> {
        let mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>;
        il = Arc::new(BipartiteIncidenceList { F_vertices: Vector::new(0), U_vertices: Vector::new(0), edges: Vector::new(0), graph: Vector::new(0), vertEqFn: vertexEq.clone(), edgeEqFn: edgeEq.clone(), vertToString: vertexStr.clone(), edgeToString: edgeStr.clone() });
        il
    }

    pub fn getRow<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut d: i32) -> Arc<metamodelica::List<i32>> {
        let mut row: Arc<metamodelica::List<i32>> = Vector::get(il.graph.clone(), d.clone()).unwrap();
        row
    }

    pub fn addVertex<VertexT: Clone + 'static + Default, EdgeT: Clone + 'static>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut v: VertexT, mut ST: SetType) -> Result<i32> {
        let mut d: i32 = 0;
        d = (match ST.clone() {
        SetType::F => {
            Vector::push(il.F_vertices.clone(), v.clone());
            Vector::push(il.graph.clone(), metamodelica::nil());
            Vector::size(il.F_vertices.clone())
        },
        SetType::U => {
            Vector::push(il.U_vertices.clone(), v.clone());
            Vector::push(il.graph.clone(), metamodelica::nil());
            Vector::size(il.U_vertices.clone())
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SBGraph.BipartiteIncidenceList.addVertex")); __mm_s.push_str(&*literal!(" failed for wrong SetType: ")); __mm_s.push_str(&*setTypeString(ST.clone())); __mm_s.push_str(&*literal!("\nAllowed: F,U")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
        Ok(d)
    }

    pub fn findVertex<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut ST: SetType, mut predFn: Arc<dyn ::std::ops::Fn(VertexT) -> Result<bool> + 'static>) -> Result<Option<i32>> {
        pub type PredFn<VertexT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(VertexT) -> Result<bool> + 'static>;

        let mut od: Option<i32> = None;
        let mut index: i32 = 0;
        index = (match ST.clone() {
        SetType::F => {
            (_, index) = Vector::find(il.F_vertices.clone(), predFn.clone());
            index.clone()
        },
        SetType::U => {
            (_, index) = Vector::find(il.U_vertices.clone(), predFn.clone());
            index.clone()
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SBGraph.BipartiteIncidenceList.findVertex")); __mm_s.push_str(&*literal!(" failed for wrong SetType: ")); __mm_s.push_str(&*setTypeString(ST.clone())); __mm_s.push_str(&*literal!("\nAllowed: F,U")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
        od = if (index.clone() > 0) {Some(index.clone())} else {None};
        Ok(od)
    }

    pub fn getVertex<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut d: i32, mut ST: SetType) -> Result<VertexT> {
        let mut v: VertexT;
        v = (match ST.clone() {
        SetType::F => Vector::get(il.F_vertices.clone(), d.clone())?,
        SetType::U => Vector::get(il.U_vertices.clone(), d.clone())?,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SBGraph.BipartiteIncidenceList.getVertex")); __mm_s.push_str(&*literal!(" failed for wrong SetType: ")); __mm_s.push_str(&*setTypeString(ST.clone())); __mm_s.push_str(&*literal!("\nAllowed: F,U")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
        Ok(v)
    }

    pub fn getVerticesFromSet<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut set: Arc<SBSet::SBSet>, mut ST: SetType, mut getSet: Arc<dyn ::std::ops::Fn(VertexT) -> Result<Arc<SBSet::SBSet>> + 'static>) -> Result<Arc<metamodelica::List<VertexT>>> {
        pub type getSetFn<VertexT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(VertexT) -> Result<Arc<SBSet::SBSet>> + 'static>;

        let mut set_vertices: Arc<metamodelica::List<VertexT>> = metamodelica::nil();
        for mut v in &*vertices(il.clone(), ST.clone())? {
            let mut v = v.clone();
            if !(SBSet::isEmpty(SBSet::intersection(getSet(v.clone())?, set.clone())?)) {
                set_vertices = cons(v.clone(), set_vertices.clone());
            }
        }
        Ok(set_vertices)
    }

    pub fn addEdge<VertexT: Clone + 'static, EdgeT: Clone + 'static + Default>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut d1: i32, mut d2: i32, mut e: EdgeT) -> Result<i32> {
        let mut ei: i32 = 0;
        let mut eil: Arc<metamodelica::List<i32>> = metamodelica::nil();
        eil = getRow(il.clone(), d1.clone());
        ei = List::positionOnTrue(eil.clone(), Arc::new({ let __pe_b1 = e.clone(); let __pe_b2 = il.edges.clone(); let __pe_b3 = il.edgeEqFn.clone(); move |__pe_a0| Ok(edge_finder(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())) }));
        if ei.clone() == -1 {
            Vector::push(il.edges.clone(), e.clone());
            ei = Vector::size(il.edges.clone());
            Vector::update(il.graph.clone(), d1.clone(), cons(ei.clone(), eil.clone()))?;
            Vector::update(il.graph.clone(), d2.clone(), cons(ei.clone(), Vector::get(il.graph.clone(), d2.clone())?))?;
        } else {
            Vector::update(il.edges.clone(), ei.clone(), e.clone())?;
        }
        Ok(ei)
    }

    pub fn getEdgesFromSet<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut set: Arc<SBSet::SBSet>, mut getSet: Arc<dyn ::std::ops::Fn(EdgeT) -> Result<Arc<SBSet::SBSet>> + 'static>) -> Result<Arc<metamodelica::List<EdgeT>>> {
        pub type getSetFn<EdgeT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(EdgeT) -> Result<Arc<SBSet::SBSet>> + 'static>;

        let mut set_edges: Arc<metamodelica::List<EdgeT>> = metamodelica::nil();
        for mut e in &*edges(il.clone()) {
            let mut e = e.clone();
            if !(SBSet::isEmpty(SBSet::intersection(getSet(e.clone())?, set.clone())?)) {
                set_edges = cons(e.clone(), set_edges.clone());
            }
        }
        Ok(set_edges)
    }

    pub fn getEdge<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut d: i32) -> EdgeT {
        let mut e: EdgeT = Vector::get(il.edges.clone(), d.clone()).unwrap();
        e
    }

    pub fn isEmpty<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>) -> bool {
        let mut empty: bool = Vector::size(il.F_vertices.clone()) == 0 && Vector::size(il.U_vertices.clone()) == 0;
        empty
    }

    pub fn vertexCount<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut ST: SetType) -> Result<i32> {
        let mut count: i32 = 0;
        count = (match ST.clone() {
        SetType::V { .. } => Vector::size(il.F_vertices.clone()) + Vector::size(il.U_vertices.clone()),
        SetType::F => Vector::size(il.F_vertices.clone()),
        SetType::U => Vector::size(il.U_vertices.clone()),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SBGraph.BipartiteIncidenceList.vertexCount")); __mm_s.push_str(&*literal!(" failed for wrong SetType: ")); __mm_s.push_str(&*setTypeString(ST.clone())); __mm_s.push_str(&*literal!("\nAllowed: V,F,U")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
        Ok(count)
    }

    pub fn edgeCount<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>) -> i32 {
        let mut count: i32 = Vector::size(il.edges.clone());
        count
    }

    pub fn vertices<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>, mut ST: SetType) -> Result<Arc<metamodelica::List<VertexT>>> {
        let mut vl: Arc<metamodelica::List<VertexT>> = metamodelica::nil();
        vl = (match ST.clone() {
        SetType::V { .. } => listAppend(Vector::toList(il.F_vertices.clone()), Vector::toList(il.U_vertices.clone())),
        SetType::F => Vector::toList(il.F_vertices.clone()),
        SetType::U => Vector::toList(il.U_vertices.clone()),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SBGraph.BipartiteIncidenceList.vertices")); __mm_s.push_str(&*literal!(" failed for wrong SetType: ")); __mm_s.push_str(&*setTypeString(ST.clone())); __mm_s.push_str(&*literal!("\nAllowed: V,F,U")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
        Ok(vl)
    }

    pub fn edges<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>) -> Arc<metamodelica::List<EdgeT>> {
        let mut el: Arc<metamodelica::List<EdgeT>> = Vector::toList(il.edges.clone());
        el
    }

    pub fn toString<VertexT: Clone + 'static, EdgeT: Clone + 'static>(mut il: Arc<BipartiteIncidenceList<VertexT, EdgeT>>) -> Result<ArcStr> {
        let mut r#str: ArcStr = arcstr::literal!("");
        let mut vertToString: VertexStr<VertexT>;
        let mut edgeToString: EdgeStr<EdgeT>;
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(il.clone()) {
            Deref @ BipartiteIncidenceList { edgeToString: __pa0, vertToString: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        edgeToString = __pa0.clone();
        vertToString = __pa1.clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2((literal!("Set-Based Graph")).clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_3((literal!("F-Vertices")).clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (Vector::toList(il.F_vertices.clone())).into_iter().cloned() {
            let __x = vertToString(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_3((literal!("U-Vertices")).clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (Vector::toList(il.U_vertices.clone())).into_iter().cloned() {
            let __x = vertToString(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_3((literal!("Edges")).clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (Vector::toList(il.edges.clone())).into_iter().cloned() {
            let __x = edgeToString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    pub fn setTypeString(mut ST: SetType) -> ArcStr {
        let mut r#str: ArcStr = arcstr::literal!("");
        r#str = ((match ST.clone() {
        SetType::V { .. } => literal!("V (generic vertex set)"),
        SetType::F => literal!("F (function vertex set)"),
        SetType::U => literal!("U (unknown vertex set)"),
        SetType::E => literal!("E (edge set)"),
        _ => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SBGraph.BipartiteIncidenceList.setTypeString")); __mm_s.push_str(&*literal!(" ERROR")); ArcStr::from(__mm_s) },
    })).clone();
        r#str
    }

}

