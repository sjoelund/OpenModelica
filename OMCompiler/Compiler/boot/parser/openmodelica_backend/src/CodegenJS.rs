// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::CodegenUtil;
use crate::SimCode;
use crate::SimCodeFunction;
use crate::SimCodeVar;
use openmodelica_ast::Absyn;
use openmodelica_susan::Tpl;
use openmodelica_util::Settings;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn markdownFile(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { fileNamePrefix: ref i_fileNamePrefix, .. }) => {
            let mut txt_3: Tpl::Text;
            let mut txt_2: Tpl::Text;
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            txt_0 = markdownContents(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_1 = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_1 = Tpl::writeTok(txt_1.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".md")).clone() }))?;
            Tpl::textFile(txt_0.clone(), (Tpl::textString(txt_1.clone())?).clone())?;
            txt_2 = nodeJSDriver(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_3 = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_3 = Tpl::writeTok(txt_3.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_node.js")).clone() }))?;
            Tpl::textFile(txt_2.clone(), (Tpl::textString(txt_3.clone())?).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn nodeJSDriver(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { fileNamePrefix: mut i_fileNamePrefix, simulationSettingsOpt: Some(SimCode::SimulationSettings { outputFormat: mut i_s_outputFormat, .. }), .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#!/usr/bin/nodejs --max-old-space-size=8192\n")).clone(), (literal!("var fs = require('fs');\n")).clone(), (literal!("\n")).clone(), (literal!("var initXML = fs.readFileSync('./")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_init.xml');\n")).clone(), (literal!("\n")).clone(), (literal!("var mod = require('./")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".js');\n")).clone(), (literal!("mod.FS_createDataFile(\"/\", '")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_init.xml', initXML, true, false);\n")).clone(), (literal!("mod.FS_createLazyFile(\"/\", '")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_info.xml', '")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_info.xml', true, false);\n")).clone(), (literal!("mod.callMain(process.argv.slice(2));\n")).clone(), (literal!("\n")).clone(), (literal!("var fname = '")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_res.")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_s_outputFormat.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("';\n")).clone(), (literal!("var content = mod.OpenModelica_readFile(fname);\n")).clone(), (literal!("fs.writeFileSync(fname,content);")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn markdownContents(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { fileNamePrefix: mut i_fileNamePrefix, makefileParams: SimCodeFunction::MakefileParams { ccompiler: _, .. }, simulationSettingsOpt: Some(SimCode::SimulationSettings { tolerance: mut i_s_tolerance, numberOfIntervals: mut i_s_numberOfIntervals, stopTime: mut i_s_stopTime, .. }), modelInfo: SimCode::ModelInfo { name: ref i_modelInfo_name, vars: SimCodeVar::SimVars { stateVars: _, .. }, varInfo: SimCode::VarInfo { numZeroCrossings: _, .. }, functions: _, .. }, .. }) => {
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut txt_0: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("# OpenModelica simulation example\n")).clone(), (literal!("## ")).clone()], lastHasNewLine: false }))?;
            txt_0 = CodegenUtil::dotPath(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            ret_1 = (Util::escapeModelicaStringToXmlString((Tpl::textString(txt_0.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("```yaml script=scriptloader\n")).clone(), (literal!("- lib/tinytimer.js\n")).clone(), (literal!("```\n")).clone(), (literal!("\n")).clone(), (literal!("<style media=\"screen\" type=\"text/css\">\n")).clone(), (literal!("label {font-weight:normal; size: 0.9em}\n")).clone(), (literal!("</style>\n")).clone(), (literal!("\n")).clone(), (literal!("<br/>\n")).clone(), (literal!("<br/>\n")).clone(), (literal!("\n")).clone(), (literal!("<div id=\"status\" style=\"text-align:center\"><span id=\"statustext\">\n")).clone(), (literal!("Simulation loading</span>. &nbsp Time: <span id=\"statustimer\"> </span></div>\n")).clone(), (literal!("\n")).clone(), (literal!("<br/>\n")).clone(), (literal!("\n")).clone(), (literal!("<div class = \"row\">\n")).clone(), (literal!("<div class = \"col-md-4\">\n")).clone(), (literal!("\n")).clone(), (literal!("<br/>\n")).clone(), (literal!("<br/>\n")).clone(), (literal!("<br/>\n")).clone(), (literal!("<br/>\n")).clone(), (literal!("\n")).clone(), (literal!("```yaml jquery=dform\n")).clone(), (literal!("class : form-horizontal\n")).clone(), (literal!("col1class : col-sm-7\n")).clone(), (literal!("col2class : col-sm-5\n")).clone(), (literal!("html:\n")).clone(), (literal!("  - name: stopTime\n")).clone(), (literal!("    type: number\n")).clone(), (literal!("    bs3caption: Stop time, sec\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("value: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_s_stopTime.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  - name: intervals\n")).clone(), (literal!("    type: number\n")).clone(), (literal!("    bs3caption: Output intervals\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("value: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_s_numberOfIntervals.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  - name: tolerance\n")).clone(), (literal!("    type: number\n")).clone(), (literal!("    bs3caption: Tolerance\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("value: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_s_tolerance.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("```\n")).clone(), (literal!("\n")).clone(), (literal!("```js\n")).clone(), (literal!("if (typeof(isRunning) == \"undefined\") isRunning = false\n")).clone(), (literal!("\n")).clone(), (literal!("if (typeof(timer) != \"undefined\") {clearInterval(timer.interval); timer = null};\n")).clone(), (literal!("\n")).clone(), (literal!("$(\"#statustext\").html('Simulation running')\n")).clone(), (literal!("$(\"#statustimer\").html(\"\");\n")).clone(), (literal!("$('#statustimer').tinyTimer({ from: Date.now() });\n")).clone(), (literal!("\n")).clone(), (literal!("timer = $(\"#statustimer\").data(\"tinyTimer\")\n")).clone(), (literal!("\n")).clone(), (literal!("// Start the simulation!\n")).clone(), (literal!("basename = \"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("\n")).clone(), (literal!("if (typeof(wworker) != \"undefined\" && isRunning) wworker.terminate();\n")).clone(), (literal!("if (typeof(wworker) == \"undefined\" || isRunning) {\n")).clone(), (literal!("  wworker = new Worker(basename + \".js\");\n")).clone(), (literal!("  isRunning = true\n")).clone(), (literal!("  wworker.addEventListener('error', function(event) {\n")).clone(), (literal!("  });\n")).clone(), (literal!("  // read the csv file with the simulation results\n")).clone(), (literal!("  wworker.addEventListener(\"message\", function(e) {\n")).clone(), (literal!("    var data = e.data;\n")).clone(), (literal!("    if (data.preloaded) {\n")).clone(), (literal!("      preloadComplete = true;\n")).clone(), (literal!("      wworker.postMessage({basename: basename, override: {stopTime: stopTime, tolerance: tolerance, stepSize: +stopTime / intervals}});\n")).clone(), (literal!("      return;\n")).clone(), (literal!("    }\n")).clone(), (literal!("    $(\"#statustext\").html(e.data.status)\n")).clone(), (literal!("    timer.stop();\n")).clone(), (literal!("    isRunning = false\n")).clone(), (literal!("    x = $.csv.toArrays(e.data.csv, {onParseValue: $.csv.hooks.castToScalar})\n")).clone(), (literal!("\n")).clone(), (literal!("    // `header` has the column names. The first is the time, and the rest\n")).clone(), (literal!("    // of the columns are the variables.\n")).clone(), (literal!("    header = x.slice(0,1)[0]\n")).clone(), (literal!("\n")).clone(), (literal!("    // Select graph variables with a select box based on the header values\n")).clone(), (literal!("    if (typeof(graphvar) == \"undefined\") graphvar = header[1];\n")).clone(), (literal!("    if (typeof(graphvarX) == \"undefined\") graphvarX = header[0];\n")).clone(), (literal!("\n")).clone(), (literal!("    var jsonform = {\n")).clone(), (literal!("      html: {\n")).clone(), (literal!("        type: \"select\",\n")).clone(), (literal!("        bs3caption: \"Plot variable\",\n")).clone(), (literal!("        name: \"graphvar\",\n")).clone(), (literal!("        selectvalue: graphvar,\n")).clone(), (literal!("        choices: header\n")).clone(), (literal!("    }};\n")).clone(), (literal!("    var jsonformX = {\n")).clone(), (literal!("      html: {\n")).clone(), (literal!("        type: \"select\",\n")).clone(), (literal!("        bs3caption: \"\",\n")).clone(), (literal!("        name: \"graphvarX\",\n")).clone(), (literal!("        selectvalue: graphvarX,\n")).clone(), (literal!("        choices: header\n")).clone(), (literal!("    }};\n")).clone(), (literal!("    updatefun = function (evt) {\n")).clone(), (literal!("        calculate_forms();\n")).clone(), (literal!("        $(\"#plotdiv\").calculate();\n")).clone(), (literal!("    }\n")).clone(), (literal!("\n")).clone(), (literal!("\n")).clone(), (literal!("    $(\"#yaxisform\").html(\"\");\n")).clone(), (literal!("    $(\"#yaxisform\").dform(jsonform);\n")).clone(), (literal!("    $(\"#yaxisform\").change(updatefun);\n")).clone(), (literal!("    $(\"#xaxisform\").html(\"\");\n")).clone(), (literal!("    $(\"#xaxisform\").dform(jsonformX);\n")).clone(), (literal!("    $(\"#xaxisform\").change(updatefun);\n")).clone(), (literal!("    $(\"#plotdiv\").calculate();\n")).clone(), (literal!("\n")).clone(), (literal!("}, false);\n")).clone(), (literal!("}\n")).clone(), (literal!("wworker.postMessage({basename: basename, preload: true})\n")).clone(), (literal!("```\n")).clone(), (literal!("\n")).clone(), (literal!("</div>\n")).clone(), (literal!("\n")).clone(), (literal!("<div class = \"col-md-1\">\n")).clone(), (literal!("</div>\n")).clone(), (literal!("\n")).clone(), (literal!("\n")).clone(), (literal!("<div class = \"col-md-7\">\n")).clone(), (literal!("\n")).clone(), (literal!("<!-- Nav tabs -->\n")).clone(), (literal!("<ul class=\"nav nav-tabs\" id=\"mytab\">\n")).clone(), (literal!("  <li class=\"active\"><a href=\"#model\" data-toggle=\"tab\">Model</a></li>\n")).clone(), (literal!("  <li><a href=\"#results\" data-toggle=\"tab\">Results</a></li>\n")).clone(), (literal!("</ul>\n")).clone(), (literal!("\n")).clone(), (literal!("<!-- Tab panes -->\n")).clone(), (literal!("<div class=\"tab-content\">\n")).clone(), (literal!("  <!-- Model pane -->\n")).clone(), (literal!("  <div class=\"tab-pane active\" id=\"model\">\n")).clone(), (literal!("\n")).clone(), (literal!("<img src=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".svg\" style=\"width:100%; background-color:#ffffff; border:2px solid gray\" />\n")).clone(), (literal!("\n")).clone(), (literal!("  </div>\n")).clone(), (literal!("\n")).clone(), (literal!("  <!-- Results pane -->\n")).clone(), (literal!("  <div class=\"tab-pane\" id=\"results\">\n")).clone(), (literal!("\n")).clone(), (literal!("</br>\n")).clone(), (literal!("\n")).clone(), (literal!("<div id=\"yaxisform\" style=\"width:15em; position:relative\"> </div>\n")).clone(), (literal!("\n")).clone(), (literal!("```js id=plotdiv\n")).clone(), (literal!("if (typeof(header) != \"undefined\") {\n")).clone(), (literal!("    $(\"#mytab a:last\").tab(\"show\"); // Select last tab\n")).clone(), (literal!("    yidx = header.indexOf(graphvar);\n")).clone(), (literal!("    xidx = header.indexOf(graphvarX);\n")).clone(), (literal!("    // pick out the column to plot\n")).clone(), (literal!("    series = x.slice(1).map(function(x) {return [x[xidx], x[yidx]];});\n")).clone(), (literal!("    plot([series]);\n")).clone(), (literal!("}\n")).clone(), (literal!("```\n")).clone(), (literal!("\n")).clone(), (literal!("<div id=\"xaxisform\" class=\"center-block\" style=\"text-align:center; width:15em; position:relative\"> </div>\n")).clone(), (literal!("\n")).clone(), (literal!("\n")).clone(), (literal!("  </div>\n")).clone(), (literal!("</div>\n")).clone(), (literal!("\n")).clone(), (literal!("</div>\n")).clone(), (literal!("</div>\n")).clone(), (literal!("\n")).clone(), (literal!("\n")).clone(), (literal!("## Comments\n")).clone(), (literal!("\n")).clone(), (literal!("This simulation model is from a [Modelica](http://modelica.org) model.\n")).clone(), (literal!("Modelica is a language for simulating electrical, thermal, and\n")).clone(), (literal!("mechanical, systems. [OpenModelica](http://openmodelica.org) was used\n")).clone(), (literal!("to compile this model to C. Then, [Emscripten](http://emscripten.org/)\n")).clone(), (literal!("was used to compile the C code to JavaScript.\n")).clone(), (literal!("\n")).clone(), (literal!("For more information on compiling OpenModelica to JavaScript, see\n")).clone(), (literal!("[here](https://github.com/tshort/openmodelica-javascript).\n")).clone(), (literal!("\n")).clone(), (literal!("The user interface was created in\n")).clone(), (literal!("[mdpad](http://tshort.github.io/mdpad/). See\n")).clone(), (literal!("[")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".md](")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".md) for the Markdown code\n")).clone(), (literal!("for this page.")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

