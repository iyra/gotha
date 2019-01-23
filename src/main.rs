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
        Some(x) => {print!(" [cdr]-> "); printexpr(*x, nils); },
        None => if nils { print!("-> nil") }
    }
}

fn parse(ins: &mut dyn Iterator<Item=char>, m: Expr, in_expr: i32) -> Expr {
    match ins.next() {
        Some(nc) => {
            if nc.is_whitespace() {
                if in_expr > 0 {
                    match m.car {
                        Some(mcar) => {
                            let nextexpr = parse(ins, Expr { car: None, cdr: None }, in_expr);
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
                        None => return parse(ins, Expr { car: None, cdr: None }, in_expr)
                    }
                } else {
                    return m;
                }
            }
            else if nc == '(' {
                match m.car {
                    None => {
                        let nextexpr = parse(ins, Expr { car: None, cdr: None }, in_expr+1);
                        return parse(ins, Expr { car: Some(Value {
                            valtype: Valtype::Exprval,
                            whatvalue: None,
                            intvalue: None,
                            exprvalue: Some(Box::new(nextexpr))}),
                        cdr: None,}, in_expr);
                      
                    },
                    Some(ev) => {
                        eprintln!("error: unexpected ( in car");
                        process::exit(1);
                    },
                }
            }
            else if nc == ')' {
                if in_expr > 0 {
                    return m;
                }
                else {
                    process::exit(1);
                }
            }
            else {
                match m.car {
                    None => return parse(ins, Expr {
                        car: Some(Value {
                            valtype: Valtype::Whatval,
                            whatvalue: Some(nc.to_string()),
                            intvalue: None,
                            exprvalue: None, }),
                        cdr: None,}, in_expr),
                    Some(ev) => 
                        match ev.whatvalue {
                            Some(evw) => return parse(ins, Expr {
                                car: Some(Value {
                                    valtype: Valtype::Whatval,
                                    whatvalue: Some([evw, nc.to_string()].join("")),
                                    intvalue: None,
                                    exprvalue: None, }),
                                cdr: None,}, in_expr),
                            None => {
                                eprintln!("no value found but we have something in car");
                                process::exit(1);
                            }
                        }
                }
            }
        },
        None => { return m; }
    }
}


fn main() {
    println!("Hello, world!");
    printexpr(parse(&mut "( a   lambda)".chars(), Expr{car:None, cdr:None}, 0), false);
}

