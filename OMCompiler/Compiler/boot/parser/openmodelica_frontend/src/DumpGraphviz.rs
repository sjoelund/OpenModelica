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

use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::Graphviz;

pub fn dump(mut p: Absyn::Program) -> Result<()> {
    let mut r: Arc<Graphviz::Node>;
    r = buildGraphviz(p.clone())?;
    Graphviz::dump(r.clone())?;
    Ok(())
}

fn buildGraphviz(mut inProgram: Absyn::Program) -> Result<Arc<Graphviz::Node>> {
    let mut outNode: Arc<Graphviz::Node>;
    outNode = (match inProgram.clone() {
        Absyn::Program { classes: ref cs, .. } => {
            let mut nl: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
            nl = printClasses(cs.clone())?;
            Arc::new(Graphviz::Node::NODE { type_: (literal!("ROOT")).clone(), attributes: metamodelica::nil(), children: nl.clone() })
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outNode)
}

fn printClasses(mut inAbsynClassLst: Arc<metamodelica::List<Arc<Absyn::Class>>>) -> Result<Arc<metamodelica::List<Arc<Graphviz::Node>>>> {
    let mut outNodeLst: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
    outNodeLst = (::match_deref::match_deref! { match &(inAbsynClassLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: c, tail: cs } => {
            let mut node: Arc<Graphviz::Node>;
            let mut nl: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
            node = printClass(c.clone())?;
            nl = printClasses(cs.clone())?;
            cons(node.clone(), nl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outNodeLst)
}

fn printClass(mut inClass: Arc<Absyn::Class>) -> Result<Arc<Graphviz::Node>> {
    let mut outNode: Arc<Graphviz::Node>;
    outNode = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, restriction: r, .. } => {
            let mut rs: ArcStr = arcstr::literal!("");
            let mut nl: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
            rs = (AbsynUtil::restrString(r.clone())).clone();
            nl = printParts(parts.clone())?;
            Arc::new(Graphviz::Node::NODE { type_: (rs.clone()).clone(), attributes: metamodelica::nil(), children: nl.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outNode)
}

fn printParts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<Graphviz::Node>>>> {
    let mut outNodeLst: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
    outNodeLst = (::match_deref::match_deref! { match &(inAbsynClassPartLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: c, tail: cs } => {
            let mut node: Arc<Graphviz::Node>;
            let mut nl: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
            node = printClassPart(c.clone())?;
            nl = printParts(cs.clone())?;
            cons(node.clone(), nl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outNodeLst)
}

fn printClassPart(mut inClassPart: Arc<Absyn::ClassPart>) -> Result<Arc<Graphviz::Node>> {
    let mut outNode: Arc<Graphviz::Node>;
    outNode = 'mc: {
        let __mc_input = inClassPart.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::PUBLIC { contents: el } => {
                    let mut nl: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
                    nl = printElementitems(el.clone())?;
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!("PUBLIC")).clone(), attributes: metamodelica::nil(), children: nl.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::PROTECTED { contents: el } => {
                    let mut nl: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
                    nl = printElementitems(el.clone())?;
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!("PROTECTED")).clone(), attributes: metamodelica::nil(), children: nl.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::EQUATIONS { contents: eqs } => {
                    let mut nl: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
                    nl = printEquations(eqs.clone())?;
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!("EQUATIONS")).clone(), attributes: metamodelica::nil(), children: nl.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ClassPart::ALGORITHMS { contents: als } => {
                    let mut nl: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
                    nl = printAlgorithms(als.clone())?;
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!("ALGORITHMS")).clone(), attributes: metamodelica::nil(), children: nl.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!(" DumpGraphViz.printClassPart PART_ERROR")).clone(), attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outNode)
}

fn printElementitems(mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Arc<metamodelica::List<Arc<Graphviz::Node>>>> {
    let mut outNodeLst: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
    outNodeLst = (::match_deref::match_deref! { match &(inAbsynElementItemLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: e }, tail: el } => {
            let mut nl: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
            let mut node: Arc<Graphviz::Node>;
            node = printElement(e.clone())?;
            nl = printElementitems(el.clone())?;
            cons(node.clone(), nl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outNodeLst)
}

fn makeBoolAttr(mut r#str: ArcStr, mut flag: bool) -> Graphviz::Attribute {
    let mut outAttribute: Graphviz::Attribute;
    let mut s: ArcStr = arcstr::literal!("");
    outAttribute = Graphviz::Attribute { name: (r#str.clone()).clone(), value: (boolString(flag.clone())).clone() };
    outAttribute
}

fn makeLeaf(mut r#str: ArcStr, mut al: Arc<metamodelica::List<Graphviz::Attribute>>) -> Arc<Graphviz::Node> {
    let mut outNode: Arc<Graphviz::Node>;
    outNode = Arc::new(Graphviz::Node::NODE { type_: (r#str.clone()).clone(), attributes: al.clone(), children: metamodelica::nil() });
    outNode
}

fn printElement(mut inElement: Arc<Absyn::Element>) -> Result<Arc<Graphviz::Node>> {
    let mut outNode: Arc<Graphviz::Node>;
    outNode = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: spec, finalPrefix, .. } => {
            let mut fa: Graphviz::Attribute;
            let mut elsp: Arc<Graphviz::Node>;
            fa = makeBoolAttr((literal!("final")).clone(), finalPrefix.clone());
            elsp = printElementspec(spec.clone())?;
            Arc::new(Graphviz::Node::NODE { type_: (literal!("ELEMENT")).clone(), attributes: list![fa.clone()], children: list![elsp.clone()] })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outNode)
}

fn printPath(mut p: Arc<Absyn::Path>) -> Result<Arc<Graphviz::Node>> {
    let mut pn: Arc<Graphviz::Node>;
    let mut s: ArcStr = arcstr::literal!("");
    s = (AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone();
    pn = makeLeaf((s.clone()).clone(), metamodelica::nil());
    Ok(pn)
}

fn printElementspec(mut inElementSpec: Arc<Absyn::ElementSpec>) -> Result<Arc<Graphviz::Node>> {
    let mut outNode: Arc<Graphviz::Node>;
    outNode = 'mc: {
        let __mc_input = inElementSpec.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementSpec::CLASSDEF { class_: cl, replaceable_: repl } => {
                    let mut ra: Graphviz::Attribute;
                    let _ = printClass(cl.clone())?;
                    ra = makeBoolAttr((literal!("replaceable")).clone(), repl.clone());
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!("CLASSDEF")).clone(), attributes: list![ra.clone()], children: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementSpec::EXTENDS { path: p, .. } => {
                    let mut en: Arc<Graphviz::Node>;
                    en = printPath(p.clone())?;
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!("EXTENDS")).clone(), attributes: metamodelica::nil(), children: list![en.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementSpec::COMPONENTS { components: cs, typeSpec: tspec, .. } => {
                    let mut pn: Arc<Graphviz::Node>;
                    let mut cns: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
                    let mut s: ArcStr = arcstr::literal!("");
                    s = (Dump::unparseTypeSpec(tspec.clone())?).clone();
                    pn = makeLeaf((s.clone()).clone(), metamodelica::nil());
                    cns = printComponents(cs.clone())?;
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!("COMPONENTS")).clone(), attributes: metamodelica::nil(), children: cons(pn.clone(), cns.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!(" DumpGraphviz.printElementspec ELSPEC_ERROR")).clone(), attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outNode)
}

fn printComponents(mut inAbsynComponentItemLst: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>) -> Result<Arc<metamodelica::List<Arc<Graphviz::Node>>>> {
    let mut outNodeLst: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
    outNodeLst = (::match_deref::match_deref! { match &(inAbsynComponentItemLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: c, tail: cs } => {
            let mut n: Arc<Graphviz::Node>;
            let mut nl: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
            n = printComponentitem(c.clone())?;
            nl = printComponents(cs.clone())?;
            cons(n.clone(), nl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outNodeLst)
}

fn printComponentitem(mut inComponentItem: Arc<Absyn::ComponentItem>) -> Result<Arc<Graphviz::Node>> {
    let mut outNode: Arc<Graphviz::Node>;
    outNode = (::match_deref::match_deref! { match &(inComponentItem.clone()) {
        Deref @ Absyn::ComponentItem { component: Absyn::Component { name: n, .. }, .. } => {
            let mut nn: Arc<Graphviz::Node>;
            nn = Arc::new(Graphviz::Node::NODE { type_: (n.clone()).clone(), attributes: metamodelica::nil(), children: metamodelica::nil() });
            Arc::new(Graphviz::Node::LNODE { type_: (literal!("COMPONENT")).clone(), labelLst: list![(n.clone()).clone()], attributes: metamodelica::nil(), children: list![nn.clone()] })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outNode)
}

fn printEquations(mut inAbsynEquationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>) -> Result<Arc<metamodelica::List<Arc<Graphviz::Node>>>> {
    let mut outNodeLst: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
    outNodeLst = (::match_deref::match_deref! { match &(inAbsynEquationItemLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: eq, .. }, tail: el } => {
            let mut node: Arc<Graphviz::Node>;
            let mut nl: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
            node = printEquation(eq.clone())?;
            nl = printEquations(el.clone())?;
            cons(node.clone(), nl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outNodeLst)
}

fn printEquation(mut inEquation: Arc<Absyn::Equation>) -> Result<Arc<Graphviz::Node>> {
    let mut outNode: Arc<Graphviz::Node>;
    outNode = 'mc: {
        let __mc_input = inEquation.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_EQUALS { rightSide: e2, leftSide: e1 } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s_1: ArcStr = arcstr::literal!("");
                    s1 = (Dump::printExpStr(e1.clone())?).clone();
                    s2 = (Dump::printExpStr(e2.clone())?).clone();
                    s = (stringAppend((s1.clone()).clone(), (literal!(" = ")).clone())).clone();
                    s_1 = (stringAppend((s.clone()).clone(), (s2.clone()).clone())).clone();
                    Ok(Arc::new(Graphviz::Node::LNODE { type_: (literal!("EQ_EQUALS")).clone(), labelLst: list![(s_1.clone()).clone()], attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_PDE { domain: c1, rightSide: e2, leftSide: e1 } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s_1: ArcStr = arcstr::literal!("");
                    s1 = (Dump::printExpStr(e1.clone())?).clone();
                    s2 = (Dump::printExpStr(e2.clone())?).clone();
                    s3 = (Dump::printComponentRefStr(c1.clone())?).clone();
                    s = (stringAppend((s1.clone()).clone(), (literal!(" = ")).clone())).clone();
                    s_1 = (stringAppend((s.clone()).clone(), (s2.clone()).clone())).clone();
                    s_1 = (stringAppend((s_1.clone()).clone(), (literal!(" indomain ")).clone())).clone();
                    s_1 = (stringAppend((s_1.clone()).clone(), (s3.clone()).clone())).clone();
                    Ok(Arc::new(Graphviz::Node::LNODE { type_: (literal!("EQ_PDE")).clone(), labelLst: list![(s_1.clone()).clone()], attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_CONNECT { connector2: c2, connector1: c1 } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s_2: ArcStr = arcstr::literal!("");
                    s1 = (Dump::printComponentRefStr(c1.clone())?).clone();
                    s2 = (Dump::printComponentRefStr(c2.clone())?).clone();
                    s = (stringAppend((literal!("connect(")).clone(), (s1.clone()).clone())).clone();
                    s_1 = (stringAppend((s.clone()).clone(), (s2.clone()).clone())).clone();
                    s_2 = (stringAppend((s_1.clone()).clone(), (literal!(")")).clone())).clone();
                    Ok(Arc::new(Graphviz::Node::LNODE { type_: (literal!("EQ_CONNECT")).clone(), labelLst: list![(s_2.clone()).clone()], attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_FOR { forEquations: eqs, iterators } => {
                    let mut es: ArcStr = arcstr::literal!("");
                    let mut eqn: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
                    eqn = printEquations(eqs.clone())?;
                    es = (Dump::printIteratorsStr(iterators.clone())?).clone();
                    Ok(Arc::new(Graphviz::Node::LNODE { type_: (literal!("EQ_FOR")).clone(), labelLst: list![(es.clone()).clone()], attributes: metamodelica::nil(), children: eqn.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!("EQ_ERROR")).clone(), attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outNode)
}

fn printAlgorithms(mut inAbsynAlgorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> Result<Arc<metamodelica::List<Arc<Graphviz::Node>>>> {
    let mut outNodeLst: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
    outNodeLst = (::match_deref::match_deref! { match &(inAbsynAlgorithmItemLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: e, tail: el } => {
            let mut node: Arc<Graphviz::Node>;
            let mut nl: Arc<metamodelica::List<Arc<Graphviz::Node>>> = metamodelica::nil();
            node = printAlgorithmitem(e.clone())?;
            nl = printAlgorithms(el.clone())?;
            cons(node.clone(), nl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outNodeLst)
}

fn printAlgorithmitem(mut inAlgorithmItem: Arc<Absyn::AlgorithmItem>) -> Result<Arc<Graphviz::Node>> {
    let mut outNode: Arc<Graphviz::Node>;
    outNode = 'mc: {
        let __mc_input = inAlgorithmItem.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: alg, .. } => {
                    let mut node: Arc<Graphviz::Node>;
                    node = printAlgorithm(alg.clone())?;
                    Ok(node.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!("ALG_ERROR")).clone(), attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outNode)
}

fn printAlgorithm(mut inAlgorithm: Arc<Absyn::Algorithm>) -> Result<Arc<Graphviz::Node>> {
    let mut outNode: Arc<Graphviz::Node>;
    outNode = 'mc: {
        let __mc_input = inAlgorithm.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Algorithm::ALG_ASSIGN { .. } => {
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!("ALG_ASSIGN")).clone(), attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(Arc::new(Graphviz::Node::NODE { type_: (literal!(" DumpGraphviz.printAlgorithm ALG_ERROR")).clone(), attributes: metamodelica::nil(), children: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outNode)
}

fn variabilitySymbol(mut inVariability: Absyn::Variability) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inVariability.clone() {
        Absyn::Variability::VAR => literal!(""),
        Absyn::Variability::DISCRETE => literal!("DISCRETE"),
        Absyn::Variability::PARAM => literal!("PARAM"),
        Absyn::Variability::CONST => literal!("CONST"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn directionSymbol(mut inDirection: Absyn::Direction) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inDirection.clone() {
        Absyn::Direction::BIDIR => literal!(""),
        Absyn::Direction::INPUT => literal!("INPUT"),
        Absyn::Direction::OUTPUT => literal!("OUTPUT"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

