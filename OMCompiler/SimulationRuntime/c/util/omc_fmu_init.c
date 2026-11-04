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

#include "omc_fmu_init.h"

#include <float.h>
#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if defined(OMC_METAMODELICA_RUNTIME)
#include "../meta/meta_modelica_string.h"
#endif
#include "../gc/omc_gc.h"
#include "boolean_array.h"
#include "integer_array.h"
#include "omc_file.h"
#include "omc_strdup.h"
#include "omc_string.h"
#include "real_array.h"
#include "string_array.h"

#define VERSION 1
#define HEADER_SIZE 104
#define MIN_RECORD_SIZE 88
#define NUM_ARRAYS 8

enum { DIMS, START, MIN, MAX, NOMINAL };

struct OMC_FMU_INIT {
  unsigned char *data;
  uint32_t recordSize, counts[NUM_ARRAYS], numValues, stringsSize;
  const unsigned char *records, *values;
  const char *strings;
  char error[512];
};

typedef struct VALUES {
  uint32_t count, first;
} VALUES;

static uint32_t u32(const unsigned char *p)
{
  return (uint32_t) p[0] | (uint32_t) p[1] << 8 | (uint32_t) p[2] << 16 | (uint32_t) p[3] << 24;
}

static uint64_t u64(const unsigned char *p)
{
  return (uint64_t) u32(p) | (uint64_t) u32(p + 4) << 32;
}

static double f64(const unsigned char *p)
{
  uint64_t u = u64(p);
  double d;
  memcpy(&d, &u, sizeof(d));
  return d;
}

static const char* str(const OMC_FMU_INIT *init, uint64_t offset)
{
  return offset < init->stringsSize ? init->strings + offset : "";
}

static double real_value(const OMC_FMU_INIT *init, VALUES v, uint32_t k)
{
  return f64(init->values + 8 * ((uint64_t) v.first + (v.count > 1 ? k : 0)));
}

static int64_t int_value(const OMC_FMU_INIT *init, VALUES v, uint32_t k)
{
  return (int64_t) u64(init->values + 8 * ((uint64_t) v.first + (v.count > 1 ? k : 0)));
}

static VALUES values(const unsigned char *rec, int attribute)
{
  VALUES v;
  v.count = u32(rec + 44 + 8 * attribute);
  v.first = u32(rec + 48 + 8 * attribute);
  return v;
}

static void check(OMC_FMU_INIT *init, const MODEL_DATA *md, const char *path, long size)
{
  const long expected[NUM_ARRAYS] = {md->nVariablesRealArray, md->nParametersRealArray, md->nVariablesIntegerArray,
                                     md->nParametersIntegerArray, md->nVariablesBooleanArray, md->nParametersBooleanArray,
                                     md->nVariablesStringArray, md->nParametersStringArray};
  uint64_t numRecords = 0, end, r;
  int i;

  if (size < HEADER_SIZE || memcmp(init->data, "OMCFMUI", 8)) {
    snprintf(init->error, sizeof(init->error), "%s is not an init file of an FMU.", path);
    return;
  }
  if (u32(init->data + 8) != VERSION || u32(init->data + 12) < MIN_RECORD_SIZE) {
    snprintf(init->error, sizeof(init->error), "%s has version %u, expected %d.", path, u32(init->data + 8), VERSION);
    return;
  }
  init->recordSize = u32(init->data + 12);
  for (i = 0; i < NUM_ARRAYS; i++) {
    init->counts[i] = u32(init->data + 16 + 4 * i);
    numRecords += init->counts[i];
    if ((long) init->counts[i] != expected[i]) {
      snprintf(init->error, sizeof(init->error), "%s has %u variables of kind %d, the model %ld.", path, init->counts[i], i, expected[i]);
      return;
    }
  }
  init->numValues = u32(init->data + 48);
  init->stringsSize = u32(init->data + 52);
  init->records = init->data + HEADER_SIZE;
  init->values = init->records + numRecords * init->recordSize;
  init->strings = (const char*) init->values + 8 * (uint64_t) init->numValues;
  end = HEADER_SIZE + numRecords * init->recordSize + 8 * (uint64_t) init->numValues + init->stringsSize;
  if (end > (uint64_t) size || init->stringsSize == 0 || init->strings[init->stringsSize - 1]) {
    snprintf(init->error, sizeof(init->error), "%s is truncated.", path);
    return;
  }
  for (r = 0; r < numRecords; r++) {
    for (i = DIMS; i <= NOMINAL; i++) {
      VALUES v = values(init->records + r * init->recordSize, i);
      if ((uint64_t) v.first + v.count > init->numValues) {
        snprintf(init->error, sizeof(init->error), "%s has a value index out of range.", path);
        return;
      }
    }
  }
}

OMC_FMU_INIT* omc_fmu_init_load(const MODEL_DATA *modelData)
{
  OMC_FMU_INIT *init = (OMC_FMU_INIT*) calloc(1, sizeof(OMC_FMU_INIT));
  const char *resourcesDir = modelData->resourcesDir, *prefix = modelData->modelFilePrefix;
  char *path;
  FILE *file;
  long size = -1;

  if (!resourcesDir) {
    snprintf(init->error, sizeof(init->error), "The FMU has no resources directory with %s_init.bin.", prefix);
    return init;
  }
  path = (char*) malloc(strlen(resourcesDir) + strlen(prefix) + 11);
  sprintf(path, "%s/%s_init.bin", resourcesDir, prefix);
  file = omc_fopen(path, "rb");
  if (file && !fseek(file, 0, SEEK_END) && (size = ftell(file)) >= 0 && !fseek(file, 0, SEEK_SET)) {
    init->data = (unsigned char*) malloc(size > 0 ? size : 1);
    if (fread(init->data, 1, size, file) != (size_t) size) {
      size = -1;
    }
  }
  if (size < 0) {
    snprintf(init->error, sizeof(init->error), "Cannot read %s.", path);
  } else {
    check(init, modelData, path, size);
  }
  if (file) {
    fclose(file);
  }
  free(path);
  return init;
}

const char* omc_fmu_init_error(const OMC_FMU_INIT *init)
{
  return init->error[0] ? init->error : NULL;
}

void omc_fmu_init_free(OMC_FMU_INIT *init)
{
  free(init->data);
  free(init);
}

void omc_fmu_init_experiment(const OMC_FMU_INIT *init, SIMULATION_INFO *simulationInfo)
{
  const unsigned char *h = init->data;
  simulationInfo->startTime = f64(h + 56);
  simulationInfo->stopTime = f64(h + 64);
  simulationInfo->stepSize = f64(h + 72);
  simulationInfo->tolerance = f64(h + 80);
  simulationInfo->solverMethod = GC_strdup(str(init, u32(h + 88)));
  simulationInfo->outputFormat = GC_strdup(str(init, u32(h + 92)));
  simulationInfo->variableFilter = GC_strdup(str(init, u32(h + 96)));
  simulationInfo->OPENMODELICAHOME = GC_strdup(str(init, u32(h + 100)));
}

/* The array of an attribute with the n elements of v, or one for a default. */
static uint32_t resize(base_array_t *a, VALUES v, void (*alloc)(base_array_t*, int), void (*release)(base_array_t*), size_t elementSize)
{
  uint32_t n = v.count ? v.count : 1;
  if (!a->data || base_array_nr_of_elements(*a) != (_index_t) n) {
    release(a);
    alloc(a, (int) n);
    memset(a->data, 0, n * elementSize);
  }
  return n;
}

static void set_reals(const OMC_FMU_INIT *init, real_array *a, VALUES v, modelica_real dflt)
{
  uint32_t k, n = resize(a, v, simple_alloc_1d_real_array, omc_array_release, sizeof(modelica_real));
  for (k = 0; k < n; k++) {
    put_real_element(v.count ? real_value(init, v, k) : dflt, k, a);
  }
}

static void set_integers(const OMC_FMU_INIT *init, integer_array *a, VALUES v, modelica_integer dflt)
{
  uint32_t k, n = resize(a, v, simple_alloc_1d_integer_array, omc_array_release, sizeof(modelica_integer));
  for (k = 0; k < n; k++) {
    put_integer_element(v.count ? (modelica_integer) int_value(init, v, k) : dflt, k, a);
  }
}

static void set_booleans(const OMC_FMU_INIT *init, boolean_array *a, VALUES v)
{
  uint32_t k, n = resize(a, v, simple_alloc_1d_boolean_array, omc_array_release, sizeof(modelica_boolean));
  for (k = 0; k < n; k++) {
    put_boolean_element(v.count ? int_value(init, v, k) != 0 : 0, k, a);
  }
}

static void set_strings(const OMC_FMU_INIT *init, string_array *a, VALUES v)
{
  uint32_t k, n = resize(a, v, simple_alloc_1d_string_array, omc_string_array_release, sizeof(modelica_string));
  for (k = 0; k < n; k++) {
    omc_string_move(&((modelica_string*) a->data)[k], omc_string_new(v.count ? str(init, (uint64_t) int_value(init, v, k)) : ""));
  }
}

static void read_info(const OMC_FMU_INIT *init, const unsigned char *rec, VAR_INFO *info, DIMENSION_INFO *dimension)
{
  VALUES dims = values(rec, DIMS);
  uint32_t k;

  info->id = (int) u32(rec);
  if (!info->name) {
    info->name = omc_strdup(str(init, u32(rec + 4)));
    info->comment = omc_strdup(str(init, u32(rec + 8)));
    info->info.filename = omc_strdup(str(init, u32(rec + 12)));
  }
  info->info.lineStart = (int) u32(rec + 24);
  info->info.colStart = (int) u32(rec + 28);
  info->info.lineEnd = (int) u32(rec + 32);
  info->info.colEnd = (int) u32(rec + 36);
  info->info.readonly = rec[40];
  if (dims.count && !dimension->numberOfDimensions) {
    dimension->numberOfDimensions = dims.count;
    dimension->dimensions = (DIMENSION_ATTRIBUTE*) calloc(dims.count, sizeof(DIMENSION_ATTRIBUTE));
    for (k = 0; k < dims.count; k++) {
      dimension->dimensions[k].type = DIMENSION_BY_START;
      dimension->dimensions[k].start = (modelica_integer) int_value(init, dims, k);
      dimension->dimensions[k].valueReference = -1;
    }
  }
}

void omc_fmu_init_variables(const OMC_FMU_INIT *init, MODEL_DATA *md, int startOnly)
{
  const unsigned char *rec = init->records;
  uint32_t a, i;

  for (a = 0; a < NUM_ARRAYS; a++) {
    for (i = 0; i < init->counts[a]; i++, rec += init->recordSize) {
      switch (a) {
        case 0:
        case 1: {
          STATIC_REAL_DATA *d = a == 0 ? &md->realVarsData[i] : &md->realParameterData[i];
          set_reals(init, &d->attribute.start, values(rec, START), 0.0);
          if (startOnly) break;
          read_info(init, rec, &d->info, &d->dimension);
          omc_string_move(&d->attribute.unit, omc_string_new(str(init, u32(rec + 16))));
          omc_string_move(&d->attribute.displayUnit, omc_string_new(str(init, u32(rec + 20))));
          set_reals(init, &d->attribute.min, values(rec, MIN), -DBL_MAX);
          set_reals(init, &d->attribute.max, values(rec, MAX), DBL_MAX);
          set_reals(init, &d->attribute.nominal, values(rec, NOMINAL), 1.0);
          d->attribute.fixed = rec[41];
          d->attribute.useNominal = rec[42];
          break;
        }
        case 2:
        case 3: {
          STATIC_INTEGER_DATA *d = a == 2 ? &md->integerVarsData[i] : &md->integerParameterData[i];
          set_integers(init, &d->attribute.start, values(rec, START), 0);
          if (startOnly) break;
          read_info(init, rec, &d->info, &d->dimension);
          set_integers(init, &d->attribute.min, values(rec, MIN), -LONG_MAX);
          set_integers(init, &d->attribute.max, values(rec, MAX), LONG_MAX);
          d->attribute.fixed = rec[41];
          break;
        }
        case 4:
        case 5: {
          STATIC_BOOLEAN_DATA *d = a == 4 ? &md->booleanVarsData[i] : &md->booleanParameterData[i];
          set_booleans(init, &d->attribute.start, values(rec, START));
          if (startOnly) break;
          read_info(init, rec, &d->info, &d->dimension);
          d->attribute.fixed = rec[41];
          break;
        }
        default: {
          STATIC_STRING_DATA *d = a == 6 ? &md->stringVarsData[i] : &md->stringParameterData[i];
          set_strings(init, &d->attribute.start, values(rec, START));
          if (startOnly) break;
          read_info(init, rec, &d->info, &d->dimension);
          break;
        }
      }
    }
  }
}
