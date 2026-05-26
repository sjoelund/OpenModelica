// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

pub const configureCommandLine: &'static str = "Configured  using arguments: ";

pub const os: &'static str = "linux";

pub const is64Bit: bool = true;

pub const isWindows: bool = const_str::equal!(os, "Windows_NT");

pub const platform: &'static str = if isWindows && is64Bit { "WIN64" } else if isWindows { "WIN32" } else { "Unix" };

pub const make: &'static str = "make";

pub const cmake: &'static str = "cmake";

pub const exeExt: &'static str = if isWindows { ".exe" } else { "" };

pub const dllExt: &'static str = ".so";

pub const haveBStatic: bool = true;

pub const bstatic: &'static str = if haveBStatic { "-Wl,-Bstatic" } else { "" };

pub const bdynamic: &'static str = if haveBStatic { "-Wl,-Bdynamic" } else { "" };

pub const groupDelimiter: &'static str = if isWindows { ";" } else { ":" };

pub const pathDelimiter: &'static str = "/";

pub const ldflags_runtime: &'static str = " -lOpenModelicaRuntimeC -lomcgc -llapack -lblas -lm -lpthread -rdynamic";

pub const ldflags_runtime_sim: &'static str = " -lSimulationRuntimeC -lOpenModelicaRuntimeC -lomcgc -lzlib -llapack -lblas -lm -ldl -lpthread -lgfortran -lstdc++ -rdynamic ";

pub const ldflags_runtime_fmu: &'static str = " -llapack -lblas -lm -lpthread -rdynamic ";

pub const ldflags_runtime_fmu_static: &'static str = "-Wl,-Bstatic -lSimulationRuntimeFMI -Wl,-Bdynamic -llapack -lblas -lm -ldl -lpthread -lgfortran -lstdc++ -rdynamic ";

pub const corbaLibs: &'static str = "";

pub const hwloc: &'static str = if 0 == 1 { "-lhwloc" } else { "" };

pub static systemLibs: std::sync::LazyLock<Arc<metamodelica::List<ArcStr>>> = std::sync::LazyLock::new(|| { list![(literal!("-lomcruntime")).clone(), (literal!("-lexpat")).clone(), (literal!("-lsqlite3")).clone(), (arcstr::literal!(corbaLibs)).clone(), (literal!("-lomcgc")).clone(), (arcstr::literal!(hwloc)).clone()] });

pub const triple: &'static str = "x86_64-linux-gnu";

