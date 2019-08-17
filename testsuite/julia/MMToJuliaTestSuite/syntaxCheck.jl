#=
# This file is part of OpenModelica.
#
# Copyright (c) 1998-2019, Open Source Modelica Consortium (OSMC),
# c/o Linköpings universitet, Department of Computer and Information Science,
# SE-58183 Linköping, Sweden.
#
# All rights reserved.
#
# THIS PROGRAM IS PROVIDED UNDER THE TERMS OF GPL VERSION 3 LICENSE OR
# THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.2.
# ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES
# RECIPIENT'S ACCEPTANCE OF THE OSMC PUBLIC LICENSE OR THE GPL VERSION 3,
# ACCORDING TO RECIPIENTS CHOICE.
#
# The OpenModelica software and the Open Source Modelica
# Consortium (OSMC) Public License (OSMC-PL) are obtained
# from OSMC, either from the above address,
# from the URLs: http://www.ida.liu.se/projects/OpenModelica or
# http://www.openmodelica.org, and in the OpenModelica distribution.
# GNU version 3 is obtained from: http://www.gnu.org/copyleft/gpl.html.
#
# This program is distributed WITHOUT ANY WARRANTY; without
# even the implied warranty of  MERCHANTABILITY or FITNESS
# FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
# IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.
#
# See the full OSMC Public License conditions for more details.
#
=#

module SyntaxTest
#=
#    @Author John Tinnerholm
#    This part of the tests checks that the files generated by MMTojulia.dumpProgram complies and that MMToJulia.dumpProgram does not crash
#    to the syntactic rules of Julia. No semantic check nor runtime check.
#    E.g can Julia parse what we generate :)
=#
include("../metaModelicaToJulia.jl")
include("./testsuiteUtil.jl")
using Test
using .MMToJuliaTestSuiteUtil

function executeTestSteps(homeDirectory, sourceDirectory, outputDirectory, omc)
    executeTestSteps(homeDirectory, sourceDirectory, outputDirectory, omc, x -> endswith(x, "mo"))
end

function executeTestSteps(homeDirectory, sourceDirectory, outputDirectory, omc, filterFunc)
  local outDir = createDirectoryReportErrorOnFailure(abspath(outputDirectory))
  translateFilesIfOutputIsEmpty(sourceDirectory, outDir, omc, homeDirectory, filterFunc)
  checkSyntax(outDir, sourceDirectory)
  cd(homeDirectory)
end

function syntaxCheck(omc)
  @assert pwd() == abspath(".")[1:end - 1] "Tests should be run from the suite"
  local checkHome = pwd()
  local OPENMODELICA_HOME = ENV["OPENMODELICAHOME"]
  @testset "Syntax tests" begin
    xIsAbsyn = (x -> x == "Absyn.mo")
    xIsSCode = (x -> x == "SCode.mo")
    xIsGraphviz = (x -> x == "Graphviz.mo")
    xIsAbsynUtil = (x -> x == "AbsynUtil.mo")
    xIsSCodeUtil = (x -> x == "SCodeUtil.mo")
    xIsMatchTests = (x -> x == "MatchExpressions.mo")
    executeTestSteps(checkHome, "./Primitives", "./OutputPrimitives", omc)
    executeTestSteps(checkHome, "./Algorithms", "./OutputAlgorithms", omc)
    executeTestSteps(checkHome, "$OPENMODELICA_HOME/../OMCompiler/Compiler/FrontEnd", "./OutputAbsyn", omc, xIsAbsyn)
    executeTestSteps(checkHome, "$OPENMODELICA_HOME/../OMCompiler/Compiler/FrontEnd", "./OutputSCode", omc, xIsSCode)
    executeTestSteps(checkHome, "$OPENMODELICA_HOME/../OMCompiler/Compiler/FrontEnd", "./OutputGraphviz", omc, xIsGraphviz)
    executeTestSteps(checkHome, "$OPENMODELICA_HOME/../OMCompiler/Compiler/FrontEnd", "./OutputAbsynUtil", omc, xIsAbsynUtil)
    executeTestSteps(checkHome, "$OPENMODELICA_HOME/../OMCompiler/Compiler/FrontEnd", "./OutputSCodeUtil", omc, xIsSCodeUtil)
    executeTestSteps(checkHome, "./MatchExpressions", "./OutputMatchExpressions", omc, xIsMatchTests)
  end
end

function createDirectoryReportErrorOnFailure(dirToCreate)
  local directory = dirToCreate
  if !isdir(directory)
    mkdir(directory)
  end
  @assert isdir(directory) "Failed to create directory. Aborting test"
  return directory
end

function translateFilesIfOutputIsEmpty(directoryWithModelicaFiles, directory, omc, homeDirectory, filterFunc)
  cd(directoryWithModelicaFiles)
  local primitiveTestSet = 3
  if size(readdir(directory), 1) < primitiveTestSet
    filesToConvert = [abspath(f) for f in filter(filterFunc, readdir())]
    @testset "Translation test $directory" begin
      @test metaModelicaToJulia(filesToConvert, omc, directory) != nothing
    end
  end
  cd(homeDirectory)
end

function checkSyntax(directory, description)
  @testset "Syntax test for $description: " begin
    cd(abspath("$directory"))
    #For some reason all.jl cannot be parsed and passed to eval.
    for f in filter(x -> endswith(x, "jl") && !endswith(x, "all.jl"), readdir())
      fullPath = abspath("$f")
      println("Parsing: $fullPath...")
      fileContents = read(f, String)
      println(abspath("$f"))
      @test_nothrow_nowarn Meta.parse(fileContents)
    end
  end
end

end #= End syntaxCheck.jl =#
