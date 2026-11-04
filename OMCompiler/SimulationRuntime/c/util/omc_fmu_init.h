/*
 * This file belongs to the OpenModelica Run-Time System
 *
 * Copyright (c) 1998-2026, Open Source Modelica Consortium (OSMC), c/o Linköpings
 * universitet, Department of Computer and Information Science, SE-58183 Linköping, Sweden. All rights
 * reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF THE BSD NEW LICENSE OR THE
 * AGPL VERSION 3 LICENSE OR THE OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.8. ANY
 * USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES RECIPIENT'S
 * ACCEPTANCE OF THE BSD NEW LICENSE OR THE OSMC PUBLIC LICENSE OR THE AGPL
 * VERSION 3, ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the OSMC (Open Source Modelica Consortium) Public License
 * (OSMC-PL) are obtained from OSMC, either from the above address, from the URLs:
 * http://www.openmodelica.org or https://github.com/OpenModelica/ or
 * http://www.ida.liu.se/projects/OpenModelica, and in the OpenModelica distribution. GNU
 * AGPL version 3 is obtained from: https://www.gnu.org/licenses/licenses.html#GPL. The BSD NEW
 * License is obtained from: http://www.opensource.org/licenses/BSD-3-Clause.
 *
 * This program is distributed WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY
 * SET FORTH IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF
 * OSMC-PL.
 *
 */

/*
 * resources/<prefix>_init.bin of a C FMU 2.0 or 3.0: the default experiment
 * and the info and attributes of the variables, written by omc
 * (Compiler/SimCode/SerializeFmuInit.mo). FMI 1.0 compiles the same data as
 * <prefix>_init_fmu.c instead.
 *
 * Format version 1, little-endian:
 *
 *   Header, 104 bytes: "OMCFMUI\0", u32 version, u32 record size, u32 number of
 *   records of realVarsData, realParameterData, integerVarsData,
 *   integerParameterData, booleanVarsData, booleanParameterData, stringVarsData
 *   and stringParameterData, u32 number of values, u32 size of the strings,
 *   f64 startTime, stopTime, stepSize, tolerance, u32 strings solver,
 *   outputFormat, variableFilter and OPENMODELICAHOME.
 *
 *   Records, in the order of the counts and of the MODEL_DATA arrays:
 *   i32 value reference, u32 strings name, comment, file name, unit and
 *   display unit, i32 lineStart, colStart, lineEnd, colEnd, u8 readonly, fixed,
 *   useNominal and 0, {u32 count, u32 first value} for the dimensions, start,
 *   min, max and nominal, u32 0. A reader skips fields after these.
 *
 *   Values, 8 bytes each: f64 for Real attributes, i64 for Integer, Boolean and
 *   enumeration values, dimension sizes and the string offsets of String values.
 *
 *   Strings: NUL-terminated UTF-8; offset 0 is "".
 *
 * An attribute with count 0 has its default value; count 1 is the value of all
 * elements of an array variable.
 */

#ifndef OMC_FMU_INIT_H
#define OMC_FMU_INIT_H

#include "../simulation_data.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct OMC_FMU_INIT OMC_FMU_INIT;

/* Reads <resourcesDir>/<modelFilePrefix>_init.bin and checks it against the
 * sizes in modelData. Never NULL; omc_fmu_init_error says if it failed. */
OMC_FMU_INIT* omc_fmu_init_load(const MODEL_DATA *modelData);
const char* omc_fmu_init_error(const OMC_FMU_INIT *init);
void omc_fmu_init_free(OMC_FMU_INIT *init);

/* What the generated read_simulation_info set. */
void omc_fmu_init_experiment(const OMC_FMU_INIT *init, SIMULATION_INFO *simulationInfo);
/* With startOnly, the start attributes (the generated setDefaultStartValues),
 * else also the info, dimensions and other attributes (read_input_fmu). */
void omc_fmu_init_variables(const OMC_FMU_INIT *init, MODEL_DATA *modelData, int startOnly);

#ifdef __cplusplus
}
#endif

#endif
