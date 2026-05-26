// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::Absyn;
use crate::BaseModelica;
use crate::DAEDump;
use crate::NFAlgorithm as Algorithm;
use crate::NFComponentRef as ComponentRef;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFSections as Sections;
use crate::NFStatement as Statement;
use crate::SCode;
use crate::SCodeUtil;
use openmodelica_util::IOStream;

pub enum NFSections {
    SECTIONS {
        equations: metamodelica::List<Arc<Equation::NFEquation>>,
        initialEquations: metamodelica::List<Arc<Equation::NFEquation>>,
        algorithms: metamodelica::List<Arc<Algorithm::NFAlgorithm>>,
        initialAlgorithms: metamodelica::List<Arc<Algorithm::NFAlgorithm>>,
    },
    EXTERNAL {
        name: String,
        args: metamodelica::List<Arc<Expression::NFExpression>>,
        outputRef: Arc<ComponentRef::NFComponentRef>,
        language: String,
        ann: Option<Arc<SCode::Annotation>>,
        explicit: bool,
        info: SourceInfo,
    },
    EMPTY,
}
pub use NFSections::*;
pub fn new(equations: metamodelica::List<Arc<Equation::NFEquation>>, initialEquations: metamodelica::List<Arc<Equation::NFEquation>>, algorithms: metamodelica::List<Arc<Algorithm::NFAlgorithm>>, initialAlgorithms: metamodelica::List<Arc<Algorithm::NFAlgorithm>>) -> Arc<Sections> {
    todo!()
}

pub fn equations(sections: Arc<Sections>) -> metamodelica::List<Arc<Equation::NFEquation>> {
    todo!()
}

pub fn prepend(equations: metamodelica::List<Arc<Equation::NFEquation>>, initialEquations: metamodelica::List<Arc<Equation::NFEquation>>, algorithms: metamodelica::List<Arc<Algorithm::NFAlgorithm>>, initialAlgorithms: metamodelica::List<Arc<Algorithm::NFAlgorithm>>, sections: Arc<Sections>) -> Arc<Sections> {
    todo!()
}

pub fn prependEquation(eq: Arc<Equation::NFEquation>, sections: Arc<Sections>, isInitial: bool) -> Arc<Sections> {
    todo!()
}

pub fn prependAlgorithm(alg: Arc<Algorithm::NFAlgorithm>, sections: Arc<Sections>, isInitial: bool) -> Arc<Sections> {
    todo!()
}

pub fn append(equations: metamodelica::List<Arc<Equation::NFEquation>>, initialEquations: metamodelica::List<Arc<Equation::NFEquation>>, algorithms: metamodelica::List<Arc<Algorithm::NFAlgorithm>>, initialAlgorithms: metamodelica::List<Arc<Algorithm::NFAlgorithm>>, sections: Arc<Sections>) -> Arc<Sections> {
    todo!()
}

pub fn join(sections1: Arc<Sections>, sections2: Arc<Sections>) -> Arc<Sections> {
    todo!()
}

pub fn map(sections: Arc<Sections>, eqFn: fn(Arc<Equation::NFEquation>) -> Arc<Equation::NFEquation>, algFn: fn(Arc<Algorithm::NFAlgorithm>) -> Arc<Algorithm::NFAlgorithm>, ieqFn: fn(Arc<Equation::NFEquation>) -> Arc<Equation::NFEquation>, ialgFn: fn(Arc<Algorithm::NFAlgorithm>) -> Arc<Algorithm::NFAlgorithm>) -> Arc<Sections> {
    todo!()
}

pub fn map1<ArgT>(sections: Arc<Sections>, arg: ArgT, eqFn: fn(Arc<Equation::NFEquation>, ArgT) -> Arc<Equation::NFEquation>, algFn: fn(Arc<Algorithm::NFAlgorithm>, ArgT) -> Arc<Algorithm::NFAlgorithm>, ieqFn: fn(Arc<Equation::NFEquation>, ArgT) -> Arc<Equation::NFEquation>, ialgFn: fn(Arc<Algorithm::NFAlgorithm>, ArgT) -> Arc<Algorithm::NFAlgorithm>) -> Arc<Sections> {
    todo!()
}

pub fn mapExp(sections: Arc<Sections>, mapFn: fn(Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression>) -> Arc<Sections> {
    todo!()
}

pub fn foldExp<ArgT>(sections: Arc<Sections>, foldFn: fn(Arc<Expression::NFExpression>, ArgT) -> ArgT, arg: ArgT) -> ArgT {
    todo!()
}

pub fn apply(sections: Arc<Sections>, eqFn: fn(Arc<Equation::NFEquation>) -> (), algFn: fn(Arc<Algorithm::NFAlgorithm>) -> (), ieqFn: fn(Arc<Equation::NFEquation>) -> (), ialgFn: fn(Arc<Algorithm::NFAlgorithm>) -> ()) -> () {
    todo!()
}

pub fn isEmpty(sections: Arc<Sections>) -> bool {
    todo!()
}

pub fn toStream(sections: Arc<Sections>, indent: String, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}

pub fn toFlatStream(sections: Arc<Sections>, scopeName: Arc<Absyn::Path>, format: BaseModelica::OutputFormat, indent: String, s: IOStream::IOStream) -> IOStream::IOStream {
    todo!()
}


