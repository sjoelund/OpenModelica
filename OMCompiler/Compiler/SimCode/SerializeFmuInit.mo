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

encapsulated package SerializeFmuInit
"Writes resources/<prefix>_init.bin of a C FMU 2.0 or 3.0: the default
 experiment and the info and attributes of the variables. The format is
 described in SimulationRuntime/c/util/omc_fmu_init.h."

import SimCode;
import SimCodeVar;

protected
import DAE;
import DAE.{Exp,Type};
import Expression;
import ExpressionBasics;
import File;
import List;
import SimCode.SimulationSettings;
import SimCodeVar.SimVar;
import Types;
import UnorderedMap;
import Util;

constant Integer VERSION = 1;
constant Integer HEADER_SIZE = 104;
constant Integer RECORD_SIZE = 88;

uniontype Value
  record REAL
    Real r;
  end REAL;
  record INT
    Integer i;
  end INT;
end Value;

uniontype Values
  record VALUES
    Integer count;
    Integer index;
  end VALUES;
end Values;

constant Values NO_VALUES = VALUES(0, 0);

uniontype Pools
  record POOLS
    UnorderedMap<String, Integer> stringOffsets;
    list<String> strings "reversed";
    Integer stringsSize;
    list<Value> values "reversed";
    Integer numValues;
  end POOLS;
end Pools;

uniontype VarRecord
  record VAR_RECORD
    Integer valueReference;
    Integer name, comment, fileName, unit, displayUnit;
    Integer lineStart, colStart, lineEnd, colEnd;
    Boolean readonly, fixed, useNominal;
    Values dimensions, start, min, max, nominal;
  end VAR_RECORD;
end VarRecord;

public

function write
  input SimCode.SimCode simCode;
  input String fileName;
protected
  SimCodeVar.SimVars vars = simCode.modelInfo.vars;
  SimulationSettings s;
  Pools pools;
  list<list<SimVar>> arrays;
  list<VarRecord> records = {};
  list<Integer> counts = {};
  list<Integer> experimentStrings = {};
  Integer vr = 1000, o;
  VarRecord r;
  File.File file = File.File();
algorithm
  SOME(s) := simCode.simulationSettingsOpt;
  pools := POOLS(UnorderedMap.new<Integer>(stringHashDjb2, stringEq), {""}, 1, {}, 0);
  UnorderedMap.add("", 0, pools.stringOffsets);
  arrays := {List.flatten({vars.stateVars, vars.derivativeVars, vars.algVars, vars.discreteAlgVars,
                           vars.realOptimizeConstraintsVars, vars.realOptimizeFinalConstraintsVars}),
             vars.paramVars, vars.intAlgVars, vars.intParamVars, vars.boolAlgVars, vars.boolParamVars,
             vars.stringAlgVars, vars.stringParamVars};
  for a in arrays loop
    for var in a loop
      (r, pools) := variable(var, vr, pools);
      records := r :: records;
      vr := vr + 1;
    end for;
    counts := listLength(a) :: counts;
  end for;
  for str in {s.method, s.outputFormat, s.variableFilter, simCode.makefileParams.omhome} loop
    (o, pools) := internString(str, pools);
    experimentStrings := o :: experimentStrings;
  end for;

  File.open(file, fileName, File.Mode.Write);
  File.write(file, "OMCFMUI");
  File.writeBinaryInteger(file, 0, 1);
  File.writeBinaryInteger(file, VERSION, 4);
  File.writeBinaryInteger(file, RECORD_SIZE, 4);
  for n in listReverse(counts) loop
    File.writeBinaryInteger(file, n, 4);
  end for;
  File.writeBinaryInteger(file, pools.numValues, 4);
  File.writeBinaryInteger(file, pools.stringsSize, 4);
  File.writeBinaryReal(file, s.startTime);
  File.writeBinaryReal(file, s.stopTime);
  File.writeBinaryReal(file, s.stepSize);
  File.writeBinaryReal(file, s.tolerance);
  for offset in listReverse(experimentStrings) loop
    File.writeBinaryInteger(file, offset, 4);
  end for;
  for rec in listReverse(records) loop
    writeRecord(file, rec);
  end for;
  for v in listReverse(pools.values) loop
    _ := match v
      case REAL() algorithm File.writeBinaryReal(file, v.r); then ();
      case INT() algorithm File.writeBinaryInteger(file, v.i, 8); then ();
    end match;
  end for;
  for str in listReverse(pools.strings) loop
    File.write(file, str);
    File.writeBinaryInteger(file, 0, 1);
  end for;
end write;

protected

function variable
  input SimVar var;
  input Integer valueReference;
  output VarRecord r;
  input output Pools pools;
protected
  SourceInfo info = var.source.info;
  Type ty = Types.arrayElementType(var.type_);
  Boolean isReal = Types.isRealOrSubTypeReal(ty);
  Integer name, comment, fileName, unit = 0, displayUnit = 0;
  Values dims, start, min = NO_VALUES, max = NO_VALUES, nominal = NO_VALUES;
algorithm
  (name, pools) := appendString(crefString(var.name), pools);
  (comment, pools) := internString(var.comment, pools);
  (fileName, pools) := internString(info.fileName, pools);
  if isReal then
    (unit, pools) := internString(var.unit, pools);
    (displayUnit, pools) := internString(var.displayUnit, pools);
  end if;
  (dims, pools) := dimensions(var, pools);
  (start, pools) := values(var.initialValue, isReal, pools);
  _ := match ty
    case DAE.T_REAL()
      algorithm
        (min, pools) := values(var.minValue, true, pools);
        (max, pools) := values(var.maxValue, true, pools);
        (nominal, pools) := values(var.nominalValue, true, pools);
      then ();
    case DAE.T_INTEGER()
      algorithm
        (min, pools) := values(var.minValue, false, pools);
        (max, pools) := values(var.maxValue, false, pools);
      then ();
    case DAE.T_ENUMERATION()
      algorithm
        (min, pools) := values(if isSome(var.minValue) then var.minValue else SOME(DAE.ICONST(1)), false, pools);
        (max, pools) := values(if isSome(var.maxValue) then var.maxValue else SOME(DAE.ICONST(listLength(ty.names))), false, pools);
      then ();
    else ();
  end match;
  r := VAR_RECORD(valueReference, name, comment, fileName, unit, displayUnit,
                  info.lineNumberStart, info.columnNumberStart, info.lineNumberEnd, info.columnNumberEnd,
                  info.isReadOnly, var.isFixed, isReal and isSome(var.nominalValue),
                  dims, start, min, max, nominal);
end variable;

function dimensions
  "The sizes of a non-scalarized array variable."
  input SimVar var;
  output Values v = NO_VALUES;
  input output Pools pools;
algorithm
  if Types.isArray(var.type_) and not listEmpty(var.numArrayElement) then
    (v, pools) := addValues(list(INT(stringInt(d)) for d in var.numArrayElement), pools);
  end if;
end dimensions;

function values
  "A literal start, min, max or nominal: one value for a scalar or a value of
   all elements of an array, else one per element. Not given if it is not a
   literal."
  input Option<Exp> exp;
  input Boolean isReal;
  output Values v = NO_VALUES;
  input output Pools pools;
protected
  list<Exp> elements;
  list<Value> vals = {};
  Integer o;
algorithm
  if isNone(exp) then
    return;
  end if;
  try
    elements := literalElements(Util.getOption(exp));
  else
    return;
  end try;
  for e in elements loop
    _ := match e
      case Exp.SCONST()
        algorithm
          (o, pools) := internString(e.string, pools);
          vals := INT(o) :: vals;
        then ();
      else
        algorithm
          vals := (if isReal then REAL(realValue(e)) else INT(integerValue(e))) :: vals;
        then ();
    end match;
  end for;
  (v, pools) := addValues(listReverse(vals), pools);
end values;

function literalElements
  input Exp exp;
  output list<Exp> elements;
algorithm
  elements := match exp
    local
      Integer n;
    case Exp.ICONST() then {exp};
    case Exp.RCONST() then {exp};
    case Exp.SCONST() then {exp};
    case Exp.BCONST() then {exp};
    case Exp.ENUM_LITERAL() then {exp};
    case Exp.ARRAY() guard Expression.isSimpleLiteralValue(exp, true)
      then List.flatten(list(literalElements(e) for e in exp.array));
    case Exp.REDUCTION() guard isBroadcastValue(exp.expr) then literalElements(exp.expr);
    case Exp.REDUCTION()
      algorithm
        n := List.fold(list(Expression.dimensionSize(d) for d in List.firstN(Expression.arrayDimension(exp.reductionInfo.exprType), listLength(exp.iterators))), intMul, 1);
      then List.flatten(List.fill(literalElements(exp.expr), n));
  end match;
end literalElements;

function isBroadcastValue
  input Exp exp;
  output Boolean b;
algorithm
  b := match exp
    case Exp.ARRAY() then false;
    case Exp.REDUCTION() then isBroadcastValue(exp.expr);
    else true;
  end match;
end isBroadcastValue;

function realValue
  input Exp exp;
  output Real r;
algorithm
  r := match exp
    case Exp.RCONST() then exp.real;
    case Exp.ICONST() then intReal(exp.integer);
    case Exp.ENUM_LITERAL() then intReal(exp.index);
    case Exp.BCONST() then if exp.bool then 1.0 else 0.0;
  end match;
end realValue;

function integerValue
  input Exp exp;
  output Integer i;
algorithm
  i := match exp
    case Exp.ICONST() then exp.integer;
    case Exp.ENUM_LITERAL() then exp.index;
    case Exp.BCONST() then if exp.bool then 1 else 0;
    case Exp.RCONST() then realInt(exp.real);
  end match;
end integerValue;

function addValues
  input list<Value> vals;
  output Values v;
  input output Pools pools;
algorithm
  v := VALUES(listLength(vals), pools.numValues);
  pools.values := List.append_reverse(vals, pools.values);
  pools.numValues := pools.numValues + v.count;
end addValues;

function internString
  input String str;
  output Integer offset;
  input output Pools pools;
algorithm
  offset := match UnorderedMap.get(str, pools.stringOffsets)
    case SOME(offset) then offset;
    else
      algorithm
        (offset, pools) := appendString(str, pools);
        UnorderedMap.add(str, offset, pools.stringOffsets);
      then offset;
  end match;
end internString;

function appendString
  "For strings that occur once, e.g. variable names."
  input String str;
  output Integer offset;
  input output Pools pools;
algorithm
  offset := pools.stringsSize;
  pools.strings := str :: pools.strings;
  pools.stringsSize := offset + stringLength(str) + 1;
end appendString;

function crefString
  "The name as the result file spells it, e.g. der(a.b[1])."
  input DAE.ComponentRef cref;
  output String str = stringAppendList(listReverse(crefParts(cref, {})));
end crefString;

function crefParts
  "The pieces of crefString, reversed."
  input DAE.ComponentRef cref;
  input output list<String> parts;
algorithm
  parts := match cref
    case DAE.CREF_IDENT() then subscriptsParts(cref.subscriptLst, cref.ident :: parts);
    case DAE.CREF_QUAL(ident = DAE.derivativeNamePrefix) then ")" :: crefParts(cref.componentRef, "der(" :: parts);
    case DAE.CREF_QUAL(ident = DAE.previousNamePrefix) then ")" :: crefParts(cref.componentRef, "previous(" :: parts);
    case DAE.CREF_QUAL() then crefParts(cref.componentRef, "." :: subscriptsParts(cref.subscriptLst, cref.ident :: parts));
  end match;
end crefParts;

function subscriptsParts
  input list<DAE.Subscript> subs;
  input output list<String> parts;
protected
  Boolean first = true;
algorithm
  if listEmpty(subs) then
    return;
  end if;
  parts := "[" :: parts;
  for s in subs loop
    if not first then
      parts := "," :: parts;
    end if;
    first := false;
    parts := subscriptString(s) :: parts;
  end for;
  parts := "]" :: parts;
end subscriptsParts;

function subscriptString
  input DAE.Subscript sub;
  output String str;
algorithm
  str := match sub
    local
      Integer i;
    case DAE.WHOLEDIM() then ":";
    case DAE.SLICE(DAE.ICONST(i)) then intString(i);
    case DAE.INDEX(DAE.ICONST(i)) then intString(i);
    case DAE.WHOLE_NONEXP(DAE.ICONST(i)) then intString(i);
    case DAE.SLICE() then ExpressionBasics.printExpStr(sub.exp);
    case DAE.INDEX() then ExpressionBasics.printExpStr(sub.exp);
    case DAE.WHOLE_NONEXP() then ExpressionBasics.printExpStr(sub.exp);
  end match;
end subscriptString;

function writeRecord
  input File.File file;
  input VarRecord r;
algorithm
  File.writeBinaryInteger(file, r.valueReference, 4);
  File.writeBinaryInteger(file, r.name, 4);
  File.writeBinaryInteger(file, r.comment, 4);
  File.writeBinaryInteger(file, r.fileName, 4);
  File.writeBinaryInteger(file, r.unit, 4);
  File.writeBinaryInteger(file, r.displayUnit, 4);
  File.writeBinaryInteger(file, r.lineStart, 4);
  File.writeBinaryInteger(file, r.colStart, 4);
  File.writeBinaryInteger(file, r.lineEnd, 4);
  File.writeBinaryInteger(file, r.colEnd, 4);
  File.writeBinaryInteger(file, if r.readonly then 1 else 0, 1);
  File.writeBinaryInteger(file, if r.fixed then 1 else 0, 1);
  File.writeBinaryInteger(file, if r.useNominal then 1 else 0, 1);
  File.writeBinaryInteger(file, 0, 1);
  writeValues(file, r.dimensions);
  writeValues(file, r.start);
  writeValues(file, r.min);
  writeValues(file, r.max);
  writeValues(file, r.nominal);
  File.writeBinaryInteger(file, 0, 4);
end writeRecord;

function writeValues
  input File.File file;
  input Values v;
algorithm
  File.writeBinaryInteger(file, v.count, 4);
  File.writeBinaryInteger(file, v.index, 4);
end writeValues;

annotation(__OpenModelica_Interface="backend_tools");
end SerializeFmuInit;
