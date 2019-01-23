use std::process;
use std::str;
#[derive(Debug)]
enum Valtype {
    Whatval,
    Intval,
    Exprval,
}

#[derive(Debug)]
struct Value {
    valtype: Valtype,
    whatvalue: Option<String>,
    intvalue: Option<u32>,
    exprvalue: Option<Box<Expr>>,
}

#[derive(Debug)]
struct Expr {
    car: Option<Value>,
    cdr: Option<Box<Expr>>,
}

fn printexpr(m: Expr, nils: bool) {
    match m.car {
        Some(x) =>
            match x.valtype {
                Valtype::Whatval => print!("{}", match x.whatvalue {Some(v)=>v, None=>"???".to_string()}),
                Valtype::Intval => match x.intvalue {Some(v)=>print!("{}",v), None=>print!("(int)???")},
                Valtype::Exprval => { print!("(");
                                      match x.exprvalue {Some(v)=>printexpr(*v, nils), None=>print!("(expr)???")}
                                      print!(")") },
            }
        None => if nils { print!("[car nil]") }
    }
    match m.cdr {
        Some(x) => {print!(" "); printexpr(*x, nils); },
        None => if nils { print!("-> nil") }
    }
}

fn parse(ins: &mut dyn Iterator<Item=char>, m: Expr, in_expr: i32, in_str: bool, escape: bool, in_comment: bool) -> Expr {
    let escapable_chars = ['"', '\\', ';'];
    match ins.next() {
        Some(nc) => {
            println!("char = {}", nc);
            if escape && !escapable_chars.contains(&nc) {
                eprintln!("error: tried to escape a non-escapable character: {}", nc);
                process::exit(1);
            }

            
            if nc == ';' && !in_str {
                return parse(ins, m, in_expr, in_str, escape, true);
            } else if nc.is_whitespace() && !in_str {
                if (in_comment && nc == '\n' && !escape) || !in_comment {
                    if in_expr > 0 {
                        match m.car {
                            Some(mcar) => {
                                let nextexpr = parse(ins, Expr { car: None, cdr: None }, in_expr, in_str, escape, false);
                                //println!("nextexpr: {:?} {:?}", nextexpr, Some(mcar));
                                match nextexpr.car {
                                    Some(nec) => return Expr { car: Some(mcar), cdr: Some(Box::new(Expr{car:Some(nec), cdr:nextexpr.cdr})) },
                                    None => match nextexpr.cdr {
                                        None => return Expr { car: Some(mcar), cdr: None },
                                        Some(necdr) => return Expr { car: Some(mcar), cdr: Some(necdr) }
                                    },
                                }
                            },
                            /* deal with the case of an opening paren followed by whitespace */
                            None => return parse(ins, Expr { car: None, cdr: None }, in_expr, in_str, escape, false)
                        }
                    } else {
                        return m;
                    }
                } else {
                    return parse(ins, m, in_expr, in_str, escape, true)
                }
            }
            else if nc == '(' && !in_str && !in_comment {
                match m.car {
                    None => {
                        let nextexpr = parse(ins, Expr { car: None, cdr: None }, in_expr+1, in_str, escape, false);
                        return parse(ins, Expr { car: Some(Value {
                            valtype: Valtype::Exprval,
                            whatvalue: None,
                            intvalue: None,
                            exprvalue: Some(Box::new(nextexpr))}),
                                                 cdr: None,}, in_expr, in_str, escape, false);
                        
                    },
                    Some(ev) => {
                        eprintln!("error: unexpected ( in car");
                        process::exit(1);
                    },
                }
            }
            else if nc == ')' && !in_str && !in_comment {
                if in_expr > 0 {
                    return m;
                }
                else {
                    process::exit(1);
                }
            }
            else if !in_comment {
                match m.car {
                    None => return parse(ins, Expr {
                        car: Some(Value {
                            valtype: Valtype::Whatval,
                            whatvalue: if (nc == '"' && !escape) ||  (nc == '\\' && !escape) { Some("".to_string()) } else { Some(nc.to_string())  },
                            intvalue: None,
                            exprvalue: None, }),
                        cdr: None,}, in_expr, if nc == '"' && !escape { !in_str } else { in_str }, if nc == '\\' { !escape } else { false }, false),
                    Some(ev) => 
                        match ev.whatvalue {
                            Some(evw) => return parse(ins, Expr {
                                car: Some(Value {
                                    valtype: Valtype::Whatval,
                                    whatvalue: if (nc == '"' && !escape) ||  (nc == '\\' && !escape) { Some(evw)  } else { Some([evw, nc.to_string()].join("")) },
                                    intvalue: None,
                                    exprvalue: None, }),
                                cdr: None,}, in_expr, if nc == '"' && !escape { !in_str } else { in_str }, if nc == '\\' { !escape } else { false }, false),
                            None => {
                                eprintln!("{:?}", ev);
                                eprintln!("error: car is overloaded with previous expression... are you missing a space?");
                                process::exit(1);
                            }
                        }
                }
            }
            else {
                return parse(ins, m, in_expr, in_str, escape, true);
            }
        },
        None => {
            if in_expr == 0 {
                return m;
            } else {
                eprintln!("error: unmatched parens by the end of input");
                process::exit(1);
            }
        }
    }
}


fn main() {
    println!("Hello, world!");
    printexpr(parse(&mut "(\"q;abc\np\")".chars(), Expr{car:None, cdr:None}, 0, false, false, false), false);
}

