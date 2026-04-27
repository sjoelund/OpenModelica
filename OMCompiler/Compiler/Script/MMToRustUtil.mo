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

encapsulated package MMToRustUtil
protected
import Absyn;
import AbsynUtil;
import Tpl;
import Util;
protected
import CevalScript;
import StringUtil;
import System;

public
uniontype Context

  record FUNCTION
    String retValsStr "Contains return values";
  end FUNCTION;

  record FUNCTION_RETURN_CONTEXT
    String retValsStr "Contains return values";
    String ty_str "String of the type we are currently operating on";
  end FUNCTION_RETURN_CONTEXT;

  record PACKAGE
  end PACKAGE;

  record UNIONTYPE
    String name;
  end UNIONTYPE;

  record NO_CONTEXT
  end NO_CONTEXT;

  record INPUT_CONTEXT
    String ty_str;
  end INPUT_CONTEXT;

  record CONSTANT_CONTEXT
    String ty_str;
  end CONSTANT_CONTEXT;

  record MATCH_CONTEXT
    Absyn.Exp inputExp;
  end MATCH_CONTEXT;

  record STRUCT_CONTEXT
  end STRUCT_CONTEXT;

  record TOP_CONTEXT
  end TOP_CONTEXT;

end Context;

constant Context packageContext = PACKAGE();
constant Context noContext = NO_CONTEXT();
constant Context functionContext = FUNCTION("");
constant Context returnContext = FUNCTION_RETURN_CONTEXT("","");
constant Context inputContext = INPUT_CONTEXT("");
constant Context structContext = STRUCT_CONTEXT();
constant Context topContext = TOP_CONTEXT();

function makeUniontypeContext
  input String name;
  output Context context;
algorithm
  context := UNIONTYPE(name);
end makeUniontypeContext;

function makeInputContext
  input String ty_str;
  output Context context;
algorithm
  context := INPUT_CONTEXT(ty_str);
end makeInputContext;

function makeConstantContext
  input String ty_str;
  output Context context;
algorithm
  context := CONSTANT_CONTEXT(ty_str);
end makeConstantContext;

function makeFunctionContext
  input String returnValuesStr;
  output Context context;
algorithm
  context := FUNCTION(returnValuesStr);
end makeFunctionContext;

function makeFunctionReturnContext
  input String returnValuesStr;
  input String ty_str;
  output Context context;
algorithm
  context := FUNCTION_RETURN_CONTEXT(returnValuesStr, ty_str);
end makeFunctionReturnContext;

function makeMatchContext
  input Absyn.Exp iExp;
  output Context context;
algorithm
  context := MATCH_CONTEXT(iExp);
end makeMatchContext;

function makeInputDirection
  output Absyn.Direction direction;
algorithm
  direction := Absyn.INPUT();
end makeInputDirection;

function makeOutputDirection
  output Absyn.Direction direction;
algorithm
  direction := Absyn.OUTPUT();
end makeOutputDirection;

function makeInputOutputDirection
  output Absyn.Direction direction;
algorithm
  direction := Absyn.INPUT_OUTPUT();
end makeInputOutputDirection;

function makeBDirection
  output Absyn.Direction direction;
algorithm
  direction := Absyn.BIDIR();
end makeBDirection;

function isFunctionContext
  input Context givenCTX;
  output Boolean isFuncCTX = false;
algorithm
  isFuncCTX := match givenCTX case FUNCTION(__) then true; else false; end match;
end isFunctionContext;

function filterOnDirection
"@author johti17
Returns a list<ElementItem>, where the direction is equal to the supplied direction or input-output direction"
  input list<Absyn.ElementItem> inputs;
  input Absyn.Direction direction;
  output list<Absyn.ElementItem> outputs = {};
protected
  Absyn.Direction ioDirection = makeInputOutputDirection();
  Boolean directionEQ = false;
algorithm
  for i in inputs loop
    directionEQ := AbsynUtil.directionEqual(direction, AbsynUtil.getDirection(i))
      or AbsynUtil.directionEqual(ioDirection, AbsynUtil.getDirection(i));
    if directionEQ then
      outputs := i :: outputs;
    end if;
  end for;
end filterOnDirection;

function elementSpecIsBIDIR
 "@author:johti17"
  input Absyn.ElementSpec spec;
  output Boolean isBidir;
algorithm
  isBidir := match spec
    local Absyn.ElementAttributes attributes;
    case Absyn.COMPONENTS(attributes=attributes) then
      match attributes.direction
        case Absyn.BIDIR() then true;
        else false;
      end match;
    else false;
  end match;
end elementSpecIsBIDIR;

function elementSpecIsOUTPUT
 "@author:johti17"
  input Absyn.ElementSpec spec;
  output Boolean isOutput;
algorithm
  isOutput := match spec
    local Absyn.ElementAttributes attributes;
    case Absyn.COMPONENTS(attributes=attributes) then
      match attributes.direction
        case Absyn.OUTPUT() then true;
        else false;
      end match;
    else false;
  end match;
end elementSpecIsOUTPUT;

function elementSpecIsOUTPUT_OR_BIDIR
 "@author:johti17"
  input Absyn.ElementSpec spec;
  output Boolean isOutput;
algorithm
  isOutput := elementSpecIsOUTPUT(spec) or elementSpecIsBIDIR(spec);
end elementSpecIsOUTPUT_OR_BIDIR;

function explicitReturnInClassPart
  "@author:johti17
   Only works for Algorithms!"
  input list<Absyn.ClassPart> classParts;
  output Boolean existsImplicitReturn;
algorithm
  for cp in classParts loop
    existsImplicitReturn := match cp
      local list<Absyn.AlgorithmItem> contents;
      case Absyn.ALGORITHMS(contents = contents) then algorithmItemsContainsReturn(contents);
      else false;
    end match;
  end for;
end explicitReturnInClassPart;

function allPublicElementItems
  "@author:johti17
   Only works for Algorithms!"
  input list<Absyn.ClassPart> classParts;
  output list<Absyn.ElementItem> outputs = {};
algorithm
  for cp in classParts loop
    outputs := match cp
      local list<Absyn.ElementItem> contents;
      case Absyn.PUBLIC(contents = contents) then listAppend(outputs, contents);
      else outputs;
    end match;
  end for;
end allPublicElementItems;

function algorithmItemsContainsReturn
  "@author: johti17"
  input list<Absyn.AlgorithmItem> contents;
  output Boolean existsReturn;
algorithm
  for item in contents loop
    existsReturn := match item
      local Absyn.Algorithm alg;
      case Absyn.ALGORITHMITEM(algorithm_ = alg) then
        match alg
          case Absyn.ALG_RETURN(__) then true;
          else false;
        end match;
      else false;
    end match;
  end for;
end algorithmItemsContainsReturn;

function fixKeywords
  input String inName;
  output String res;
algorithm
  res := match inName
    case "type" then "r#type";
    case "ref" then "r#ref";
    case "Self" then "r#self";
    case "mod" then "r#mod";
    case "static" then "r#static";
    case "typeof" then "r#typeof";
    else inName;
  end match;
end fixKeywords;

function toSnakeCase
  input String inName;
  output String res;
protected
  Integer i;
  Integer c;
  Boolean prevIsUpper = false;
  Boolean prevIsInsert = false;
  list<String> result;
  String name = inName;
algorithm
  name := fixKeywords(inName);
  name := match name
    case "SCode" then "scode";
    case "SimCode" then "simcode";
    case "Static" then "r#static";
    case "Mod" then "modification";
    else inName;
  end match;
  result := {};

  for i in 1:stringLength(name) loop
    c := stringGet(name, i);
    if StringUtil.isAsciiUpper(c) then
      // Insert underscore before uppercase if:
      // - previous char was lowercase, OR
      // - next char exists and is lowercase (handles acronyms: "XMLParser" → "xml_parser")
      if prevIsUpper and stringLength(name) < i + 1 then
        // Last char of acronym, no underscore needed before final letter
        prevIsInsert := false;
      elseif prevIsUpper then
        // Check if next char is lowercase → insert underscore
        if stringLength(name) >= i + 1 and StringUtil.isAsciiLower(stringGet(name, i + 1)) then
          if not prevIsInsert then
            result := "_" :: result;
            prevIsInsert := true;
          end if;
        else
          prevIsInsert := false;
        end if;
      else
        // Previous was lowercase, always insert underscore
        result := "_" :: result;
        prevIsInsert := true;
      end if;
      result := intStringChar(StringUtil.toAsciiLower(c)) :: result;
      prevIsUpper := true;
    else
      if StringUtil.isAsciiLower(c) then
        prevIsUpper := false;
        prevIsInsert := false;
      end if;
      result := intStringChar(c) :: result;
    end if;
  end for;
  result := listReverse(result);
  if listGet(result,1) == "_" then
    result := listRest(result);
  end if;
  res := stringAppendList(result);
end toSnakeCase;

function getImports
  input Absyn.Class _class;
  output list<Absyn.Import> imports;
protected
  list<Absyn.Import> pub,pro;
algorithm
  (pub,pro) := CevalScript.getImportList(_class);
  imports := listAppend(pub, pro);
end getImports;

annotation(__OpenModelica_Interface="backend");
end MMToRustUtil;
