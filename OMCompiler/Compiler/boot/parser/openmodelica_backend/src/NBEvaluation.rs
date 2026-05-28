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

use crate::BackendDAE as OldBackendDAE;
use crate::NBModule as Module;
use crate::NBPartition::Partition;
use crate::NBStrongComponent as StrongComponent;
use crate::NBackendDAE as BackendDAE;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;

// Old Backend imports
pub mod Stages {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Stages {
        pub dynamicEval: bool,
        pub algebraicEval: bool,
        pub zerocrossEval: bool,
        pub discreteEval: bool,
    }

    impl Default for Stages {
        fn default() -> Self {
            Self {
                dynamicEval: Default::default(),
                algebraicEval: Default::default(),
                zerocrossEval: Default::default(),
                discreteEval: Default::default(),
            }
        }
    }

    pub type STAGES = Stages;

    pub fn convert(mut stages: Arc<Stages>) -> OldBackendDAE::EvaluationStages {
        let mut oldEvalStages: OldBackendDAE::EvaluationStages = <OldBackendDAE::EvaluationStages as ::std::default::Default>::default();
        oldEvalStages = OldBackendDAE::EvaluationStages { discreteEval: stages.discreteEval.clone(), zerocrossEval: stages.zerocrossEval.clone(), algebraicEval: stages.algebraicEval.clone(), dynamicEval: stages.dynamicEval.clone() };
        oldEvalStages
    }

}

pub static DEFAULT_STAGES: std::sync::LazyLock<Arc<Stages::Stages>> = std::sync::LazyLock::new(|| { Arc::new(Stages::Stages { dynamicEval: true, algebraicEval: true, zerocrossEval: false, discreteEval: true }) });

pub fn removeDummies(mut bdae: Arc<BackendDAE::NBackendDAE>) -> Arc<BackendDAE::NBackendDAE> {
    let mut bdae: Arc<BackendDAE::NBackendDAE> = bdae;
    bdae = (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ BackendDAE::MAIN { .. } => {
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                ode = ({
        let mut __acc: Arc<metamodelica::List<Arc<Partition::Partition>>> = metamodelica::nil();
        for mut p in (var_field!((*bdae).ode, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
            let __x = removeDummyComponents(p.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
                algebraic = ({
        let mut __acc: Arc<metamodelica::List<Arc<Partition::Partition>>> = metamodelica::nil();
        for mut p in (var_field!((*bdae).algebraic, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
            let __x = removeDummyComponents(p.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
                ode_event = ({
        let mut __acc: Arc<metamodelica::List<Arc<Partition::Partition>>> = metamodelica::nil();
        for mut p in (var_field!((*bdae).ode_event, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
            let __x = removeDummyComponents(p.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
                alg_event = ({
        let mut __acc: Arc<metamodelica::List<Arc<Partition::Partition>>> = metamodelica::nil();
        for mut p in (var_field!((*bdae).alg_event, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
            let __x = removeDummyComponents(p.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
            );
            bdae.clone()
        },
        _ => bdae.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    bdae
}

pub fn removeDummyComponents(mut part: Arc<Partition::Partition>) -> Arc<Partition::Partition> {
    let mut part: Arc<Partition::Partition> = part;
    assign_field!(part.strongComponents = Util::applyOption(part.strongComponents.clone(), Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<bool> + 'static> = (std::sync::Arc::new(fnptr!(StrongComponent::isDummy, Arc<StrongComponent::NBStrongComponent>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>) -> Result<bool> + 'static>); move |__pe_a0| Ok(Array::filter(__pe_a0, __pe_b1.clone())) })));
    part
}

