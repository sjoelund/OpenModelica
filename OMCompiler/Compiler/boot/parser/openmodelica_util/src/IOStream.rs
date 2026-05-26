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

use crate::IOStreamExt;
use openmodelica_util_datatypes_basic::List;

/// TODO! change these to X_TYPE
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IOStreamType {
    FILE {
        name: ArcStr,
    },
    LIST,
    BUFFER,
}
pub use self::IOStreamType::{FILE,LIST,BUFFER};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IOStreamData {
    FILE_DATA {
        data: i32,
    },
    LIST_DATA {
        data: Arc<metamodelica::List<ArcStr>>,
    },
    BUFFER_DATA {
        data: i32,
    },
}
pub use self::IOStreamData::{FILE_DATA,LIST_DATA,BUFFER_DATA};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IOStream {
    pub name: ArcStr,
    pub ty: IOStreamType,
    pub data: IOStreamData,
}

pub type IOSTREAM = IOStream;


pub const stdInput: i32 = 0;

pub const stdOutput: i32 = 1;

pub const stdError: i32 = 2;

pub static emptyStreamOfTypeList: std::sync::LazyLock<IOStream> = std::sync::LazyLock::new(|| { IOStream { name: (literal!("emptyStreamOfTypeList")).clone(), ty: crate::IOStream::IOStreamType::LIST, data: IOStreamData::LIST_DATA { data: metamodelica::nil() } } });

pub fn create(mut streamName: ArcStr, mut streamType: IOStreamType) -> Result<IOStream> {
    let mut outStream: IOStream;
    outStream = (match (streamName.clone(), streamType.clone()) {
        (_, IOStreamType::FILE { name: mut fileName }) => {
            let mut fileID: i32 = 0;
            fileID = IOStreamExt::createFile((fileName.clone()).clone())?;
            IOStream { name: (streamName.clone()).clone(), ty: streamType.clone(), data: IOStreamData::FILE_DATA { data: fileID.clone() } }
        },
        (_, IOStreamType::LIST { .. }) => {
            IOStream { name: (streamName.clone()).clone(), ty: streamType.clone(), data: IOStreamData::LIST_DATA { data: metamodelica::nil() } }
        },
        (_, IOStreamType::BUFFER { .. }) => {
            let mut bufferID: i32 = 0;
            bufferID = IOStreamExt::createBuffer()?;
            IOStream { name: (streamName.clone()).clone(), ty: streamType.clone(), data: IOStreamData::BUFFER_DATA { data: bufferID.clone() } }
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outStream)
}

pub fn append(mut inStream: IOStream, mut inString: ArcStr) -> Result<IOStream> {
    let mut outStream: IOStream;
    outStream = (match (inStream.clone(), inString.clone()) {
        (ref fStream @ IOStream { data: IOStreamData::FILE_DATA { data: ref fileID }, .. }, _) => {
            IOStreamExt::appendFile(fileID.clone(), (inString.clone()).clone())?;
            fStream.clone()
        },
        (IOStream { name: mut streamName, ty: mut streamType, data: IOStreamData::LIST_DATA { data: ref listData } }, _) => {
            IOStream { name: (streamName.clone()).clone(), ty: streamType.clone(), data: IOStreamData::LIST_DATA { data: cons(inString.clone(), listData.clone()) } }
        },
        (ref bStream @ IOStream { data: IOStreamData::BUFFER_DATA { data: ref bufferID }, .. }, _) => {
            IOStreamExt::appendBuffer(bufferID.clone(), (inString.clone()).clone())?;
            bStream.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outStream)
}

pub fn appendList(mut inStream: IOStream, mut inStringList: Arc<metamodelica::List<ArcStr>>) -> IOStream {
    let mut outStream: IOStream;
    outStream = List::foldr(inStringList.clone(), Arc::new(append), inStream.clone());
    outStream
}

pub fn appendListReverse(mut s: IOStream, mut data: Arc<metamodelica::List<ArcStr>>) -> Result<IOStream> {
    let mut s: IOStream = s;
    let mut s_data: IOStreamData = s.data.clone();
    let () = (match s_data.clone() {
        IOStreamData::FILE_DATA { .. } => {
            for mut r#str in &*data.clone() {
                let mut r#str = r#str.clone();
                IOStreamExt::appendFile(var_field!(s_data.data, IOStreamData::FILE_DATA).clone(), (r#str.clone()).clone())?;
            }
            ()
        },
        IOStreamData::LIST_DATA { .. } => {
            let __owned_variant_data_0 = listAppend(data.clone(), var_field!(s_data.data, IOStreamData::LIST_DATA).clone());
            if let IOStreamData::LIST_DATA { data, .. } = &mut s_data {
                *data = __owned_variant_data_0;
            } else { panic!("owned-variant field-assign: value held a different variant than IOStreamData::LIST_DATA"); }
            s.data = s_data.clone();
            ()
        },
        IOStreamData::BUFFER_DATA { .. } => {
            for mut r#str in &*data.clone() {
                let mut r#str = r#str.clone();
                IOStreamExt::appendBuffer(var_field!(s_data.data, IOStreamData::BUFFER_DATA).clone(), (r#str.clone()).clone())?;
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(s)
}

pub fn appendListStream(mut srcStream: IOStream, mut dstStream: IOStream) -> Result<IOStream> {
    let mut dstStream: IOStream = dstStream;
    let mut data: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let IOSTREAM { data: IOStreamData::LIST_DATA { data: __pa0 }, .. } = (srcStream.clone()) else { bail!("pattern mismatch") };
    data = __pa0.clone();
    dstStream = appendListReverse(dstStream.clone(), data.clone())?;
    Ok(dstStream)
}

pub fn close(mut inStream: IOStream) -> Result<IOStream> {
    let mut outStream: IOStream;
    outStream = 'mc: {
        let __mc_input = inStream.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let ref fStream @ IOStream { data: IOStreamData::FILE_DATA { data: ref fileID }, .. } = __mc_input.clone() else { bail!("nomatch") };
            IOStreamExt::closeFile(fileID.clone())?;
            Ok(fStream.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(inStream.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStream)
}

pub fn delete(mut inStream: IOStream) -> Result<()> {
    let _ = (match inStream.clone() {
        IOStream { data: IOStreamData::FILE_DATA { data: mut fileID }, .. } => {
            IOStreamExt::deleteFile(fileID.clone())?;
            ()
        },
        IOStream { data: IOStreamData::LIST_DATA { .. }, .. } => {
            ()
        },
        IOStream { data: IOStreamData::BUFFER_DATA { data: mut bufferID }, .. } => {
            IOStreamExt::deleteBuffer(bufferID.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

pub fn clear(mut inStream: IOStream) -> Result<IOStream> {
    let mut outStream: IOStream;
    outStream = 'mc: {
        let __mc_input = inStream.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let ref fStream @ IOStream { data: IOStreamData::FILE_DATA { data: ref fileID }, .. } = __mc_input.clone() else { bail!("nomatch") };
            IOStreamExt::clearFile(fileID.clone())?;
            Ok(fStream.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let IOStream { name: mut name, ty: mut ty, data: _ } = __mc_input.clone() else { bail!("nomatch") };
            Ok(IOStream { name: (name.clone()).clone(), ty: ty.clone(), data: IOStreamData::LIST_DATA { data: metamodelica::nil() } })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let ref bStream @ IOStream { data: IOStreamData::BUFFER_DATA { data: ref bufferID }, .. } = __mc_input.clone() else { bail!("nomatch") };
            IOStreamExt::clearBuffer(bufferID.clone())?;
            Ok(bStream.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStream)
}

pub fn empty(mut inStream: IOStream) -> Result<bool> {
    let mut res: bool = false;
    let mut data: IOStreamData = inStream.data.clone();
    res = (match data.clone() {
        IOStreamData::LIST_DATA { .. } => var_field!(data.data, IOStreamData::LIST_DATA).clone().is_empty(),
        _ => bail!("match: no arm matched"),
    });
    Ok(res)
}

pub fn string(mut inStream: IOStream) -> Result<ArcStr> {
    let mut string: ArcStr = arcstr::literal!("");
    string = ((match inStream.clone() {
        IOStream { data: IOStreamData::FILE_DATA { data: mut fileID }, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (IOStreamExt::readFile(fileID.clone())?).clone();
            r#str.clone()
        },
        IOStream { data: IOStreamData::LIST_DATA { data: ref listData }, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (IOStreamExt::appendReversedList(listData.clone())).clone();
            r#str.clone()
        },
        IOStream { data: IOStreamData::BUFFER_DATA { data: mut bufferID }, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (IOStreamExt::readBuffer(bufferID.clone())?).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(string)
}

pub fn print(mut inStream: IOStream, mut whereToPrint: i32) -> Result<()> {
    let _ = (match (inStream.clone(), whereToPrint.clone()) {
        (IOStream { data: IOStreamData::FILE_DATA { data: mut fileID }, .. }, _) => {
            IOStreamExt::printFile(fileID.clone(), whereToPrint.clone())?;
            ()
        },
        (IOStream { data: IOStreamData::BUFFER_DATA { data: mut bufferID }, .. }, _) => {
            IOStreamExt::printBuffer(bufferID.clone(), whereToPrint.clone())?;
            ()
        },
        (IOStream { data: IOStreamData::LIST_DATA { data: ref listData }, .. }, _) => {
            IOStreamExt::printReversedList(listData.clone(), whereToPrint.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

/*
TODO! Global Streams to be implemented later
IOStream.remember(IOStream, id);
IOStream = IOStream.aquire(id);
IOStream.forget(IOStream, id);
*/
