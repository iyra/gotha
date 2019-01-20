use std::process;

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

fn parse(ins: String, m: Expr, in_expr: i32) -> (Expr, String) {
    println!("{:?}", m);
    match m.car {
        Some(x) => { // car is already starting to be filled or is done
                match ins.chars().next() {
                    Some(nc) => {
                        if !nc.is_whitespace() {
                            match x.whatvalue {
                                Some(v) => {
                                    if nc == ')' {
                                        if in_expr > 0 {
                                        return (Expr {
                                            car: Some(Value {
                                                valtype: Valtype::Whatval,
                                                whatvalue: Some(v),
                                                intvalue: None,
                                                exprvalue: None,
                                            }),
                                            cdr: m.cdr}, ins)
                                        } else {
                                            eprintln!("error: unexpected )");
                                            process::exit;
                                        }
                                    }
                                    if nc != '(' {
                                        /* stop people putting ( in the middle of a symbol... */
                                        parse(ins.chars().skip(1).take(ins.chars().count()).collect(), Expr {
                                            car: Some(Value {
                                                valtype: Valtype::Whatval,
                                                whatvalue: Some([v, nc.to_string()].join("")),
                                                intvalue: None,
                                                exprvalue: None,
                                            }),
                                            cdr: m.cdr}, in_expr)
                                    } else {
                                        eprintln!("error: unexpected (");
                                        process::exit(1);
                                    }
                                    
                                },
                                None => {
                                    eprintln!("error: value type set to what but found nothing there");
                                    process::exit(1);
                                }
                            }
                        } else { // started collecting an int but now we have whitespace
                            if in_expr > 0{
                                    let (nextexpr, ni) = parse(ins.chars().skip(1).take(ins.chars().count()).collect(),
                                                         Expr{car:None, cdr:None}, in_expr+1);
                                    return (Expr {
                                        car: Some(x),
                                        cdr: Some(Box::new(nextexpr))
                                    }, ni);  
                                }
                            else {return (Expr { car: Some(x), cdr: None }, ins) }
                        }
                    },
                    None => {
                        if in_expr > 0 {
                            eprintln!("error: unexpected end of input");
                        }
                        //process::exit(1);
                        return (Expr {
                            car: Some(x),
                            cdr: m.cdr,
                        }, ins);
                    }
                }
        },
        None => { // car has not been filled yet
            match ins.chars().next() {
                Some(nc) => {
                    if nc == '(' {
                        let (nextexpr, ni) = parse(ins.chars().skip(1).take(ins.chars().count()).collect(),
                                                   Expr{car:None, cdr:None}, in_expr+1);
                        //let (cdrexp, ni2) = parse(ni.chars().skip(1).take(ins.chars().count()).collect(), Expr{car:None, cdr:None}, in_expr);
                        parse(ni.chars().skip(1).take(ins.chars().count()).collect(), Expr {
                            car: Some(Value {
                                valtype: Valtype::Exprval,
                                whatvalue:None,
                                intvalue: None,
                                exprvalue: Some(Box::new(nextexpr)),
                            }),
                            cdr: None }, in_expr)
                    } else {
                        parse(ins.chars().skip(1).take(ins.chars().count()).collect(), Expr {
                            car: Some(Value {
                                valtype: Valtype::Whatval,
                                whatvalue:Some(nc.to_string()),
                                intvalue: None,
                                exprvalue: None,
                            }),
                            cdr: None }, in_expr)
                    }
                },
                None => {
                    eprintln!("error: expected more string");
                    process::exit(1);
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", parse("(ab)".to_string(), Expr{car:None, cdr:None}, 0));
}
