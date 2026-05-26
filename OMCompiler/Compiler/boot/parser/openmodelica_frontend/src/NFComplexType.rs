// Auto-generated from MetaModelica source
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use metamodelica::*; // Built-in types and functions

use crate::NFComplexType as ComplexType;
use crate::NFInstNode::InstNode;
use crate::NFRecord as Record;
use openmodelica_util::UnorderedMap;

pub enum NFComplexType {
    CLASS,
    EXTENDS_TYPE {
        baseClass: Arc<InstNode::InstNode>,
    },
    CONNECTOR {
        potentials: metamodelica::List<Arc<InstNode::InstNode>>,
        flows: metamodelica::List<Arc<InstNode::InstNode>>,
        streams: metamodelica::List<Arc<InstNode::InstNode>>,
    },
    EXPANDABLE_CONNECTOR {
        potentiallyPresents: metamodelica::List<Arc<InstNode::InstNode>>,
        expandableConnectors: metamodelica::List<Arc<InstNode::InstNode>>,
    },
    RECORD {
        constructor: Arc<InstNode::InstNode>,
        fields: Vec<Arc<NFRecord::Field::Field>>,
        indexMap: UnorderedMap::UnorderedMap<i32, String>,
    },
    EXTERNAL_OBJECT {
        constructor: Arc<InstNode::InstNode>,
        destructor: Arc<InstNode::InstNode>,
    },
}
pub use NFComplexType::*;

