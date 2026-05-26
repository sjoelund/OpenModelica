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

use crate::SimCodeVar;

pub const fn emptyHpcomData() -> HpcOmData { HpcOmData { schedules: None, hpcOmMemory: None } }

#[derive(Clone, Debug, PartialEq)]
pub struct HpcOmData {
    pub schedules: Option<(Arc<Schedule>, Arc<Schedule>, Arc<Schedule>)>,
    pub hpcOmMemory: Option<MemoryMap>,
}

impl Default for HpcOmData {
    fn default() -> Self {
        Self {
            schedules: Default::default(),
            hpcOmMemory: Default::default(),
        }
    }
}

pub type HPCOMDATA = HpcOmData;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryMap {
    MEMORYMAP_ARRAY {
        floatArraySize: i32,
        intArraySize: i32,
        boolArraySize: i32,
        stringArraySize: i32,
    },
    MEMORYMAP_UNIFORM,
}
impl Default for MemoryMap {
    fn default() -> Self { Self::MEMORYMAP_UNIFORM }
}
pub use self::MemoryMap::{MEMORYMAP_ARRAY,MEMORYMAP_UNIFORM};

#[derive(Clone, Debug, PartialEq)]
pub struct CommunicationInfo {
    pub floatVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    pub intVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    pub boolVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
}

impl Default for CommunicationInfo {
    fn default() -> Self {
        Self {
            floatVars: Default::default(),
            intVars: Default::default(),
            boolVars: Default::default(),
        }
    }
}

pub type COMMUNICATION_INFO = CommunicationInfo;


#[derive(Clone, Debug, PartialEq)]
pub enum Task {
    SCHEDULED_TASK {
        compIdx: i32,
        numThreads: i32,
        taskSchedule: Arc<Schedule>,
    },
    CALCTASK {
        weighting: i32,
        index: i32,
        calcTime: metamodelica::Real,
        timeFinished: metamodelica::Real,
        threadIdx: i32,
        eqIdc: Arc<metamodelica::List<i32>>,
    },
    CALCTASK_LEVEL {
        eqIdc: Arc<metamodelica::List<i32>>,
        nodeIdc: Arc<metamodelica::List<i32>>,
        threadIdx: Option<i32>,
    },
    DEPTASK {
        sourceTask: Arc<Task>,
        targetTask: Arc<Task>,
        outgoing: bool,
        id: i32,
        communicationInfo: CommunicationInfo,
    },
    PREFETCHTASK {
        varIdc: Arc<metamodelica::List<i32>>,
        varArrayidx: i32,
    },
    TASKEMPTY,
}
impl Default for Task {
    fn default() -> Self { Self::TASKEMPTY }
}
pub use self::Task::{SCHEDULED_TASK,CALCTASK,CALCTASK_LEVEL,DEPTASK,PREFETCHTASK,TASKEMPTY};

#[derive(Clone, Debug, PartialEq)]
pub enum TaskList {
    PARALLELTASKLIST {
        tasks: Arc<metamodelica::List<Arc<Task>>>,
    },
    SERIALTASKLIST {
        tasks: Arc<metamodelica::List<Arc<Task>>>,
        masterOnly: bool,
    },
}
impl Default for TaskList {
    fn default() -> Self {
        Self::PARALLELTASKLIST {
            tasks: Default::default(),
        }
    }
}
pub use self::TaskList::{PARALLELTASKLIST,SERIALTASKLIST};

//TODO: Use the TaskList for the other schedulers, too
#[derive(Clone, Debug, PartialEq)]
pub enum Schedule {
    LEVELSCHEDULE {
        tasksOfLevels: Arc<metamodelica::List<TaskList>>,
        useFixedAssignments: bool,
    },
    THREADSCHEDULE {
        threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<Task>>>>,
        outgoingDepTasks: Arc<metamodelica::List<Arc<Task>>>,
        scheduledTasks: Arc<metamodelica::List<Arc<Task>>>,
        allCalcTasks: metamodelica::Array<(Arc<Task>, i32)>,
    },
    TASKDEPSCHEDULE {
        tasks: Arc<metamodelica::List<(Arc<Task>, Arc<metamodelica::List<i32>>)>>,
    },
    EMPTYSCHEDULE {
        tasks: TaskList,
    },
}
impl Default for Schedule {
    fn default() -> Self {
        Self::TASKDEPSCHEDULE {
            tasks: Default::default(),
        }
    }
}
pub use self::Schedule::{LEVELSCHEDULE,THREADSCHEDULE,TASKDEPSCHEDULE,EMPTYSCHEDULE};

