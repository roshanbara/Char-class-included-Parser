#![allow(non_snake_case)]

use std::ops::Deref;
use std::rc::Rc;
use std::collections::HashSet;
use std::str;
use std::io;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct RegEx;

#[derive(Debug)]
pub enum Regex {
    Empty(),
    Eps(),
    Letter(u8),
    CharClass(Vec<bool>),
    Or(Rc<Regex>, Rc<Regex>),
    Concat(Rc<Regex>, Rc<Regex>),
    Star(Rc<Regex>)
}


use crate::Regex::{Empty, Eps, Letter, CharClass, Or, Concat, Star};

// Finds whether a regular expression is nullable
fn findLambda (regexp: &Rc<Regex>) -> Rc<Regex> {
    match regexp.deref() {
        Empty() => Rc::new(Empty()),
        Eps()   =>  Rc::new(Eps()),
        Letter(_) => Rc::new(Empty()),
        CharClass(_) => Rc::new(Empty()),
        Or(r1, r2) => {
            let s1 = findLambda(r1);
            let s2 = findLambda(r2);
            if matches!(*s1.deref(), Eps()) || matches!(*s2.deref(), Eps()) {
                Rc::new(Eps())
            } else {
                Rc::new(Empty())
            }
        },
        Concat(r1, r2) => {
            let s1 = findLambda(r1);
            let s2 = findLambda(r2);
            if matches!(*s1.deref(), Eps()) && matches!(*s2.deref(), Eps()) {
                Rc::new(Eps())
            } else {
                Rc::new(Empty())
            }
        },
        Star(_)    => Rc::new(Eps())
    }
}

// Finds P set : all the starting letters possible in the language of the regular expression
fn constructP (regexp: &Rc<Regex>) -> HashSet<u8> {
    match regexp.deref() {
        Or(r1, r2)  => {
            let s1 = constructP(r1);
            let s2 = constructP(r2);
            let union_s: HashSet<_> = s1.union(&s2).collect();
            let mut hset: HashSet<u8> = HashSet::new();
            for x in union_s {
                hset.insert(x.clone());
                // println!("Hello11 : {}", *x);
            }
            // println!("End11");
            hset
        },
        Concat(r1, r2) => {
            let s = findLambda(r1);
            match s.deref() {
                Empty() => {
                    // println!("Go");
                    constructP(r1)
                    
                }
                Eps() => {
                    let s1 = constructP(r1);
                    let s2 = constructP(r2);
                    let union_s: HashSet<_> = s1.union(&s2).collect();
                    let mut hset: HashSet<u8> = HashSet::new();
                    for x in union_s {
                        hset.insert(x.clone());
                        // println!("Hello : {}", *x);
                    }
                    // println!("End");
                    hset
                }
                _   => {
                    let hset: HashSet<u8> = HashSet::new();
                    hset
                }
            }
        },
        Star(r1) => constructP(r1),
        Letter(x) => {
            let mut hset: HashSet<u8> = HashSet::new();
            hset.insert(*x);
            hset
        },
        _ => {
            let hset: HashSet<u8> = HashSet::new();
            hset
        }

    }
}

// Finds D set : all the terminating letters possible in the language of the regular expression
fn constructD (regexp: &Rc<Regex>) -> HashSet<u8> {
    match regexp.deref() {
        Or(r1, r2)  => {
            let s1 = constructD(r1);
            let s2 = constructD(r2);
            let union_s: HashSet<_> = s1.union(&s2).collect();
            let mut hset: HashSet<u8> = HashSet::new();
            for x in union_s {
                hset.insert(x.clone());
                // println!("Hello11 : {}", *x);
            }
            // println!("End11");
            hset
        },
        Concat(r1, r2) => {
            let s = findLambda(r2);
            match s.deref() {
                Empty() => {
                    // println!("Go");
                    constructD(r2)
                    
                }
                Eps() => {
                    let s1 = constructD(r1);
                    let s2 = constructD(r2);
                    let union_s: HashSet<_> = s1.union(&s2).collect();
                    let mut hset: HashSet<u8> = HashSet::new();
                    for x in union_s {
                        hset.insert(x.clone());
                        // println!("Hello : {}", *x);
                    }
                    // println!("End");
                    hset
                }
                _   => {
                    let hset: HashSet<u8> = HashSet::new();
                    hset
                }
            }
        }
        Star(r1) => constructD(r1),
        Letter(x) => {
            let mut hset: HashSet<u8> = HashSet::new();
            hset.insert(x.clone());
            hset
        },
        _ => {
            let hset: HashSet<u8> = HashSet::new();
            hset
        }

    }
}

// Finds F set : all the letter-pairs possible in the language of the regular expression
fn constructF (regexp: &Rc<Regex>) -> HashSet<(u8, u8)> {
    match regexp.deref() {
        Or(r1, r2)  => {
            let s1 = constructF(r1);
            // for x in &s1 {
            //     println!("in s1 : {:?}", *x);
            // }
            // println!("End or s1");
            let s2 = constructF(r2);
            // for x in &s2 {
            //     println!("in s2 : {:?}", *x);
            // }
            // println!("End or s2");
            let union_s: HashSet<_> = s1.union(&s2).collect();
            let mut hset: HashSet<(u8, u8)> = HashSet::new();
            for x in union_s {
                hset.insert(x.clone());
                // println!("Finial union or: {:?}", *x);
            }
            // println!("End10");
            hset
        },
        Concat(r1, r2) => {
            let s1 = constructF(r1);
            // for x in &s1 {
            //     println!("in s1 : {:?}", *x);
            // }
            // println!("End concat s1");
            let s2 = constructF(r2);
            // for x in &s2 {
            //     println!("in s2 : {:?}", *x);
            // }
            // println!("End concat s2");
            let mut hset0: HashSet<(u8, u8)> = HashSet::new();
            let union_helper: HashSet<_> = s1.union(&s2).collect();
            for x in union_helper {
                hset0.insert(x.clone());
                // println!("in union helper concat : {:?}", *x);
            }
            // println!("End union helper concat");
            let hs1 = constructD(r1);
            let hs2 = constructP(r2);
            let mut hset1: HashSet<(u8, u8)> = HashSet::new();
            for x in hs1 {
                for y in &hs2 {
                    hset1.insert((x.clone(), y.clone()));
                }
            }
            // for x in &hset1 {
            //     println!("in hset1 : {:?}", *x);
            // }
            // println!("End concat hset1");
            let union_s: HashSet<_> = hset0.union(&hset1).collect();
            let mut hset: HashSet<(u8, u8)> = HashSet::new();
            for x in union_s {
                hset.insert(x.clone());
                // println!("Hello12 : {:?}", *x);
            }
            // for x in &hset {
            //     // println!("in hset : {:?}", *x);
            // }
            // println!("Final concat hset");
            hset
        }
        Star(r1) => {
            let s1 = constructF(r1);
            // for x in &s1 {
            //     println!("in s1 : {:?}", *x);
            // }
            // println!("End star s1");
            let hs1 = constructD(r1);
            let hs2 = constructP(r1);
            let mut hset0: HashSet<(u8, u8)> = HashSet::new();
            for x in hs1 {
                for y in &hs2 {
                    hset0.insert((x.clone(), y.clone()));
                }
            }
            let union_s: HashSet<_> = s1.union(&hset0).collect();
            let mut hset: HashSet<(u8, u8)> = HashSet::new();
            for x in union_s {
                hset.insert(x.clone());
                // println!("Hello12 : {:?}", *x);
            }
            hset
        },
        _ => {
            let hset: HashSet<(u8, u8)> = HashSet::new();
            hset
        }
    }
}

// Checks whether a state is a final state
fn checkstate (curr: usize, final_states: &HashSet<u8>) -> bool {
    let mut res = false;
    for x in final_states {
        if (*x as usize) == curr {
            res = true;
            break;
        }
    }
    // println!("Checking curr state : {}", curr);
    res
}

// Checks string against a regular expression by executing the Glushkov-NFA
fn checkstr(s: &str, nfa: &Vec<Vec<u8>>, final_states: &HashSet<u8>, curr: usize, idx: usize, state_letter: &Vec<Vec<bool>>) -> bool {
    let mut res = false;
    let pass = checkstate(curr, final_states);
    
    if pass && idx == s.len() {
        println!("Reached Final State {}", curr);
        res = true;
    } else if !pass && idx == s.len() {
        res = false;
    } else {
        for i in &nfa[curr as usize] {
            // println!("Curr state = {} Checking state {} idx = {}, judgement of final ={}, final states : {:?}", curr,  *i, idx, checkstate(curr, final_states), final_states);
            if state_letter[(i-1) as usize][s.chars().nth(idx).unwrap() as usize] {
                println!("Char encountered: {}, curr state: {}, Going to State: {}", s.chars().nth(idx).unwrap(), curr, *i);
                res = res || checkstr(s, &nfa, &final_states, *i as usize, idx+1, &state_letter);
            }
            if res {
                break;
            }
        }
    }    
    res
}

// Generates the augmented regular expression e' from given regular expression e
fn augment(regexp: &Rc<Regex>, cnt: &mut u8) -> Rc<Regex> {
    match regexp.deref() {
        Letter(_) => {
            *cnt = *cnt + 1;
            // println!("cnt {}", a);
            // println!("cnt {}", ((*cnt - 1)*10 + a));
            let x1 = *cnt - 1;
            let val = x1 as u8;
            // let ret = Rc::new(Letter(format!("{}{}", a, val)));
            let ret = Rc::new(Letter(val));
            ret
        },
        CharClass(_) => {
            *cnt = *cnt + 1;
            // println!("cnt {}", a);
            // println!("cnt {}", ((*cnt - 1)*10 + a));
            let x1 = *cnt - 1;
            let val = x1 as u8;
            // let ret = Rc::new(Letter(format!("{}{}", a, val)));
            let ret = Rc::new(Letter(val));
            ret
        }
        Or(r1, r2) => Rc::new(Or(augment(r1, cnt), augment(r2, cnt))),
        Concat(r1, r2) => Rc::new(Concat(augment(r1, cnt), augment(r2, cnt))),
        Star(r1) => Rc::new(Star(augment(r1, cnt))),
        Empty() => Rc::new(Empty()),
        Eps() => Rc::new(Eps())
    }
}

// // Counts number of states in the generated NFA
// fn findstates(regexp: &Rc<Regex>, cnt: &mut u8) -> u8 {
//     match regexp.deref() {
//         Letter(_) => {
//             *cnt = *cnt + 1;
//             *cnt
//         },
//         Or(r1, r2) => {
//             let mut a = findstates(r1, cnt);
//             let b = findstates(r2, &mut a);
//             b
//         },
//         Concat(r1, r2) => {
//             let mut a = findstates(r1, cnt);
//             let b = findstates(r2, &mut a);
//             b
//         },
//         Star(r1) => findstates(r1, cnt),
//         _ => *cnt
//     }
// }

// // Generates a Vector of char of the letter labelled states
fn addstates(regexp: &Rc<Regex>, state_letter: &mut Vec<Vec<bool>>) {
    match regexp.deref() {
        Letter(a) => {
            let t0 = a.clone() as u8;
            let mut v1: Vec<bool> = vec![false; 256];
            v1[t0 as usize] = true;
            state_letter.push(v1.clone());
        },
        CharClass(a) => {
            state_letter.push(a.clone());
        }
        Or(r1, r2) => {
            addstates(r1, state_letter);
            addstates(r2, state_letter)
        },
        Concat(r1, r2) => {
            addstates(r1, state_letter);
            addstates(r2, state_letter)
        },
        Star(r1) => addstates(r1, state_letter),
        _ => print!("")
    }
}

// Generates Number
fn getNUM(token: &pest::iterators::Pair<Rule>, val: &mut u32) -> u32 {
    let mut tmp0 = token.clone().into_inner();
    match token.as_rule() {
        Rule::NUM   =>  {
            getNUM(&tmp0.next().unwrap(), val)
        },
        Rule::Number    =>  {
            *val = *val*10 + token.as_str().parse::<u32>().unwrap();
            getNUM(&tmp0.next().unwrap(), val)
        },
        Rule::Integer   =>  {
            *val = *val*10 + token.as_str().parse::<u32>().unwrap();
            *val
        },
        _   => *val
    }
}

// fn getCharRange(a: u8, b: u8, chclass: &mut Vec<bool>) {
//     for i in a..(b+1) {
//         chclass[i as usize] = true;
//     }
// }

fn getQuantified(regexp: &Rc<Regex>, a: &mut u32, b: &mut u32) -> Rc<Regex>{
    if *a>0 {
        *a = *a - 1;
        *b = *b - 1;
        let tmpregex = regexp.clone();
        Rc::new(Concat(tmpregex, getQuantified(regexp, a, b)))
    }
    else if *a == 0 && *b > 1 {
        *b = *b - 1;
        let tmpregex = regexp.clone();
        Rc::new(Concat(Rc::new(Or(tmpregex, Rc::new(Eps()))), getQuantified(regexp, a, b)))
    }
    else if *a == 0 && *b == 1 {
        *b = *b - 1;
        let tmpregex = regexp.clone();
        Rc::new(Or(tmpregex, Rc::new(Eps())))
    }
    // else if *a == 0 && *b == *a {
    else {
        // let tmpregex = regexp.clone();
        Rc::new(Eps())
    }
}

fn getLQuantified(regexp: &Rc<Regex>, a: &mut u32) -> Rc<Regex>{
    if *a>0 {
        *a = *a - 1;
        let tmpregex = regexp.clone();
        Rc::new(Concat(tmpregex, getLQuantified(regexp, a)))
    }
    else {
        let tmpregex = regexp.clone();
        Rc::new(Star(tmpregex))
    }
}

fn getCharClass(token: &pest::iterators::Pair<Rule>, charvec: &mut Vec<bool>) {
    let mut tmp0 = token.clone().into_inner();
    match token.as_rule() {
        Rule::CharClass => {
            getCharClass(&tmp0.next().unwrap(), charvec);
        },
        Rule::T5 => {
            getCharClass(&tmp0.next().unwrap(), charvec);
        },
        Rule::T6 => {
            getCharClass(&tmp0.next().unwrap(), charvec);
            getCharClass(&tmp0.next().unwrap(), charvec);
        },
        Rule::T7 => {
            getCharClass(&tmp0.next().unwrap(), charvec);
            getCharClass(&tmp0.next().unwrap(), charvec);
        },
        Rule::T8 => {
            getCharClass(&tmp0.next().unwrap(), charvec);
        }
        Rule::CharRange => {
            // let mut tmp1 = tmp0.next().unwrap().into_inner();
            // let t1 = tmp1.next().unwrap(); 
            let a = getletter(&tmp0.next().unwrap());
            // let t2: pest::iterators::Pair<Rule> = tmp1.next().unwrap(); 
            // let b = t2.as_str().chars().nth(0).unwrap() as u8;
            let b = getletter(&tmp0.next().unwrap());
            for i in a..(b+1) {
                charvec[i as usize] ^= true; 
            }
            // getCharClass(&tmp0.next().unwrap(), &mut charvec);
        },
        Rule::Letter => {
            // let tmp1 = tmp0.next().unwrap().into_inner();
            let a = token.as_str().chars().nth(0).unwrap() as u8;
            // println!("{}", a as char);
            // let a = tmp1.next().unwrap().as_str().chars().nth(0).unwrap() as u8;
            // let b = tmp1.next().unwrap().as_str().chars().nth(0).unwrap() as u8;
            charvec[a as usize] ^= true;
            // getCharClass(&tmp0.next().unwrap(), &mut charvec);
        },
        _ => println!("Strange encountered in CharClass")
    }
}

fn getletter(token: &pest::iterators::Pair<Rule>) -> u8 {
    // let mut tmp0 = token.clone().into_inner();
    match token.as_rule() {
        Rule::Letter => {
            let val = token.as_str().chars().nth(0).unwrap() as u8;
            // println!("{}", val as char);
            val
        },
        _ => {
            println!("dont know");
            1 as u8
        }
    }
}

// Parses a given pair to AST
fn parse_to_AST(token: &pest::iterators::Pair<Rule>) -> Rc<Regex> {
    let mut tmp0 = token.clone().into_inner();
    // println!("Yes : {:#?}", token);
    match token.as_rule() {
        Rule::Regex   => {
            // println!("in Exp");
            parse_to_AST(&tmp0.next().unwrap())
        },
        Rule::Or=> {
            // println!("Concat - ");
            let r1 = parse_to_AST(&tmp0.next().unwrap());
            let r2 = parse_to_AST(&tmp0.next().unwrap());
            Rc::new(Or(r1, r2))
        },
        Rule::T0    => {
            // println!("in T0 ");
            parse_to_AST(&tmp0.next().unwrap())
        },
        Rule::Concat=> {
            // println!("Concat - ");
            let r1 = parse_to_AST(&tmp0.next().unwrap());
            let r2 = parse_to_AST(&tmp0.next().unwrap());
            Rc::new(Concat(r1, r2))
        },
        Rule::T1    => {
            // println!("in T1 ");
            parse_to_AST(&tmp0.next().unwrap())
        },
        Rule::Star  => {
            // println!("Star - ");
            Rc::new(Star(parse_to_AST(&tmp0.next().unwrap())))
        },
        Rule::Plus  => {
            // println!("Plus - ");
            let tmp1 = parse_to_AST(&tmp0.next().unwrap());
            let tmp2 = tmp1.clone();
            Rc::new(Concat(tmp1, Rc::new(Star(tmp2))))
        },
        Rule::QMark  => {
            // println!("Plus - ");
            let tmp1 = parse_to_AST(&tmp0.next().unwrap());
            Rc::new(Or(tmp1, Rc::new(Eps())))
        },
        Rule::Quantifier    =>  {
            let tmp1 = parse_to_AST(&tmp0.next().unwrap());
            let mut tnum1: u32 = 0;
            let mut num1 = getNUM(&tmp0.next().unwrap(), &mut tnum1);
            let mut tnum2: u32 = 0;
            let mut num2 = getNUM(&tmp0.next().unwrap(), &mut tnum2);
            if num1>num2 {
                println!("In Quantifiers, left boundary cannot be greater than right boundary");
            }
            getQuantified(&tmp1, &mut num1, &mut num2)
        },
        Rule::LQuantifier    =>  {
            let tmp1 = parse_to_AST(&tmp0.next().unwrap());
            let mut tnum1: u32 = 0;
            let mut num1 = getNUM(&tmp0.next().unwrap(), &mut tnum1);
            // let mut tnum2: u32 = 0;
            // let mut num2 = getNUM(&tmp0.next().unwrap(), &mut tnum2);
            getLQuantified(&tmp1, &mut num1)
        },
        Rule::UQuantifier    =>  {
            let tmp1 = parse_to_AST(&tmp0.next().unwrap());
            let mut tnum1: u32 = 0;
            let mut tnum2: u32 = 0;
            let mut num2 = getNUM(&tmp0.next().unwrap(), &mut tnum2);
            getQuantified(&tmp1, &mut tnum1, &mut num2)
        },
        Rule::FQuantifier    =>  {
            let tmp1 = parse_to_AST(&tmp0.next().unwrap());
            let mut tnum2: u32 = 0;
            let mut num2 = getNUM(&tmp0.next().unwrap(), &mut tnum2);
            let mut tnum1: u32 = num2;
            getQuantified(&tmp1, &mut tnum1, &mut num2)
        },
        Rule::T2    => {
            // println!("in T2 ");
            parse_to_AST(&tmp0.next().unwrap())
        },
        
        Rule::Paren    => {
            // println!("in Paren ");
            parse_to_AST(&tmp0.next().unwrap())
        },

        Rule::T3    => {
            // println!("in T3 ");
            parse_to_AST(&tmp0.next().unwrap())
        },

        Rule::T4    => {
            // println!("in T4 ");
            parse_to_AST(&tmp0.next().unwrap())
        },

        Rule::CharClass => {
            let mut charvec:Vec<bool> = vec![false; 256];
            getCharClass(&tmp0.next().unwrap(), &mut charvec);
            Rc::new(CharClass(charvec))
        },
        Rule::NegCharClass => {
            let mut charvec:Vec<bool> = vec![true; 256];
            getCharClass(&tmp0.next().unwrap(), &mut charvec);
            Rc::new(CharClass(charvec))
        },
        // Rule::T5    => {
        //     // println!("in T2 ");
        //     parse_to_AST(&tmp0.next().unwrap())
        // },
        // Rule::T6    => {
        //     // println!("in T2 ");
        //     parse_to_AST(&tmp0.next().unwrap())
        // },
        Rule::Letter => Rc::new(Letter(token.as_str().chars().nth(0).unwrap() as u8)),
        _ => {
            println!("Empty Generated");
            Rc::new(Eps())
        }
    }
}

fn getStateLabels(token: &pest::iterators::Pair<Rule>, state_labels: &mut Vec<String>) {
    let mut tmp0 = token.clone().into_inner();
    // println!("Yes : {:#?}", token);
    match token.as_rule() {
        Rule::Regex   => {
            // println!("in Exp");
            getStateLabels(&tmp0.next().unwrap(), state_labels);
        },
        Rule::Or=> {
            // println!("Concat - ");
            getStateLabels(&tmp0.next().unwrap(), state_labels);
            getStateLabels(&tmp0.next().unwrap(), state_labels);
        },
        Rule::T0    => {
            // println!("in T0 ");
            getStateLabels(&tmp0.next().unwrap(), state_labels);
        },
        Rule::Concat=> {
            // println!("Concat - ");
            getStateLabels(&tmp0.next().unwrap(), state_labels);
            getStateLabels(&tmp0.next().unwrap(), state_labels);
        },
        Rule::T1    => {
            // println!("in T1 ");
            getStateLabels(&tmp0.next().unwrap(), state_labels);
        },
        Rule::Star  => {
            // println!("Star - ");
            getStateLabels(&tmp0.next().unwrap(), state_labels);
        },
        Rule::Plus  => {
            // println!("Plus - ");
            getStateLabels(&tmp0.next().unwrap(), state_labels);
        },
        Rule::QMark  => {
            // println!("Plus - ");
            getStateLabels(&tmp0.next().unwrap(), state_labels);
        },
        Rule::Quantifier    =>  {
            getStateLabels(&tmp0.next().unwrap(), state_labels);
        },
        Rule::LQuantifier    =>  {
            getStateLabels(&tmp0.next().unwrap(), state_labels);
        },
        Rule::UQuantifier    =>  {
            getStateLabels(&tmp0.next().unwrap(), state_labels);
        },
        Rule::FQuantifier    =>  {
            getStateLabels(&tmp0.next().unwrap(), state_labels);
        },
        Rule::T2    => {
            // println!("in T2 ");
            getStateLabels(&tmp0.next().unwrap(), state_labels);
        },
        
        Rule::Paren    => {
            // println!("in Paren ");
            getStateLabels(&tmp0.next().unwrap(), state_labels);
        },

        Rule::T3    => {
            // println!("in T3 ");
            getStateLabels(&tmp0.next().unwrap(), state_labels);
        },

        Rule::T4    => {
            // println!("in T4 ");
            getStateLabels(&tmp0.next().unwrap(), state_labels);
        },

        Rule::CharClass => {
            let s1 = format!("[{}]", get_classlabel(&tmp0.next().unwrap()));
            state_labels.push(s1);
        },
        Rule::NegCharClass => {
            let s1 = format!("[^{}]", get_classlabel(&tmp0.next().unwrap()));
            state_labels.push(s1);
        },
        // Rule::T5    => {
        //     // println!("in T2 ");
        //     parse_to_AST(&tmp0.next().unwrap())
        // },
        // Rule::T6    => {
        //     // println!("in T2 ");
        //     parse_to_AST(&tmp0.next().unwrap())
        // },
        Rule::Letter => {
            state_labels.push(token.as_str().to_string());
        },
        _ => {
            println!("Empty Generated");
        }
    }
}

fn get_classlabel(token: &pest::iterators::Pair<Rule>) -> String {
    let mut tmp0 = token.clone().into_inner();
    match token.as_rule() {
        Rule::T5 => {
            get_classlabel(&tmp0.next().unwrap())
        },
        Rule::T6 => {
            let tmp1 = get_classlabel(&tmp0.next().unwrap());
            let tmp2 = get_classlabel(&tmp0.next().unwrap());
            let s1 = format!("{}{}", tmp1, tmp2);
            s1
        },
        Rule::T7 => {
            let tmp1 = get_classlabel(&tmp0.next().unwrap());
            let tmp2 = get_classlabel(&tmp0.next().unwrap());
            let s1 = format!("{}{}", tmp1, tmp2);
            s1
        },
        Rule::T8 => {
            get_classlabel(&tmp0.next().unwrap())
        },
        Rule::CharRange => {
            let tmp1 = get_classlabel(&tmp0.next().unwrap());
            let tmp2 = get_classlabel(&tmp0.next().unwrap());
            let s1 = format!("{}-{}", tmp1, tmp2);
            s1
        },
        Rule::Letter => {
            // let tmp1 = get_classlabel(&tmp0.next().unwrap());
            let s1 = token.as_str().to_string();
            s1
        },
        _   =>    {
            let s1 = "".to_string();
            s1
        }
    }
}


fn main() {

    println!("Enter a RegEx:");
    let mut tmp1 = String::new();
    io::stdin().read_line(&mut tmp1).expect("failed to readline");
    let regex_input = tmp1.trim();
    // let regex_input = "[abc-z0-1]{,1}";
    // let regex_input = "bbc[^5-9]*[abc-z0-1]{3, 4}";
    // let regex_input = "bbc[^5-9]*[abc-z0-1]{3, 4}3?dd";
    // let regex_input = "[abc-z0-1]{,1}";



    // println!("Enter a string:");
    // println!("Enter a string:");
    println!("Enter a string:");
    let mut tmp2 = String::new();
    io::stdin().read_line(&mut tmp2).expect("failed to readline");
    let s: &str= tmp2.trim();
    // let s: &str = "bbc333mn03dd";
    
    // Generate pair for the regex
    let mut pairs = RegEx::parse(Rule::Regex, regex_input).unwrap_or_else(|e| panic!("{}", e));
    // println!("Hello: {:#?}", pairs);
    let mut pairs1 = pairs.clone();

    let mut all_state_labels: Vec<String> = Vec::new();

    getStateLabels(&pairs1.next().unwrap(), &mut all_state_labels);
    // print!("State Labels: {{ 1: {}", all_state_labels[0]);
    // for i in 1..all_state_labels.len() {
    //     print!(", {}: {}", i+1, all_state_labels[i]);
    // }
    // println!("}}");

    println!("State Labels: {:?}", all_state_labels);


    // Parse the pair to an AST
    let x = parse_to_AST(&pairs.next().unwrap());

    // println!("Hello: {:#?}", x);
    
    // println!("Test starts");
    // Comment from here
    let mut cnt = 1;
    let a = augment(&x, &mut cnt); 
    // println!("Test Ends");

    let no_of_states = cnt;

    println!("Augmented enum form: {:?}", a);
    
    // Generate P, D, F sets
    let P_set = constructP(&a);
    let D_set = constructD(&a);
    let F_set = constructF(&a);

    // Getting the letter labels for states
    let mut state_letter: Vec<Vec<bool>> = Vec::new();
    addstates(&x, &mut state_letter);

    
    // // Generating the NFA in the form of Adjacency Matrix
    let mut array: Vec<Vec<u8>> = vec![Vec::new(); no_of_states.into()];
    for x in &P_set {
        array[0].push(x.clone());
    }
    for x in &F_set {
        array[x.0 as usize].push(x.1);
    }
    
    println!("Pset = {:?}", P_set);
    println!("Dset = {:?}", D_set);
    println!("Fset = {:?}", F_set);
    println!("Number of States = {}", no_of_states);
    println!("NFA Adjacency Matrix : {:?}", array);

    // for i in 0..no_of_states-1 {
    //     println!("Size[{}] : {}", i, state_letter[i as usize].len());
    // }
    

    if s.len() == 0 && matches!(findLambda(&a).deref(), Eps()) {
        println!("Accepted");
    }
    let res = checkstr(s, &array, &D_set, 0, 0, &state_letter);
    if res {
        println!("Accepted");
    } else {
        println!("Rejected");
    }
}
