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

fn parse(ins: String, m: Expr, in_expr: i32) -> (Expr, String) {
    println!("{:?}", m);
    match m.car {
        Some(x) => { // car is already starting to be filled or is done
                match ins.chars().next() {
                    Some(nc) => {
                        if !nc.is_whitespace() {
                            match x.valtype {
                                Valtype::Whatval => match x.whatvalue {
                                    Some(v) => {
                                        if nc == ';' {
                                            for (i, c) in ins.chars().enumerate() {
                                                if c == '\n' {
                                                    return parse(ins.chars().skip(i).take(ins.chars().count()).collect(), Expr {
                                                        car: Some(Value {
                                                            valtype: Valtype::Whatval,
                                                            whatvalue: Some(v),
                                                            intvalue: None,
                                                            exprvalue: None,
                                                        }),
                                                        cdr: m.cdr}, in_expr)
                                                }
                                            }
                                            return (Expr {
                                                car: Some(Value {
                                                    valtype: Valtype::Whatval,
                                                    whatvalue: Some(v),
                                                    intvalue: None,
                                                    exprvalue: None,
                                                }),
                                                cdr: m.cdr}, "".to_string())
                                        }
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
                                },
                                Valtype::Exprval => {                     
                                        return (Expr {
                                            car: Some(x),
                                            cdr: m.cdr}, ins)
                                },
                                _ => {
                                    eprintln!("error: unexpected value type in tree");
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
                    if nc == '\'' {
                        let (nextexpr, ni) = parse(ins.chars().skip(1).take(ins.chars().count()).collect(),
                                                   Expr{car:None, cdr:None}, in_expr+1);
                        parse(ni.chars().skip(1).take(ins.chars().count()).collect(), Expr {
                            car: Some(Value {
                                valtype: Valtype::Exprval,
                                whatvalue:None,
                                intvalue: None,
                                exprvalue: Some(Box::new(Expr{
                                    car:Some(Value {
                                        valtype: Valtype::Whatval,
                                        whatvalue:Some("quote".to_string()),
                                        intvalue: None,
                                        exprvalue:None,
                                    }),
                                    cdr: Some(Box::new(nextexpr)),
                                }))}),
                            cdr: None}, in_expr)
                    } else if nc == '(' {
                        let (nextexpr, ni) = parse(ins.chars().skip(1).take(ins.chars().count()).collect(),
                                                   Expr{car:None, cdr:None}, in_expr+1);
                        parse(ni.chars().skip(1).take(ins.chars().count()).collect(), Expr {
                            car: Some(Value {
                                valtype: Valtype::Exprval,
                                whatvalue:None,
                                intvalue: None,
                                exprvalue: Some(Box::new(nextexpr)),
                            }),
                            cdr: None }, in_expr)
                    } else if nc == ';' {
                        for (i, c) in ins.chars().enumerate() {
                            if c == '\n' {
                                return parse(ins.chars().skip(i).take(ins.chars().count()).collect(), Expr {car:None, cdr:None}, in_expr)
                            }
                        }
                        return (Expr {car:None, cdr:None}, "".to_string()) /* just a comment */
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
    //println!("{:?}", parse("'(a)".to_string(), Expr{car:None, cdr:None}, 0));
    printexpr(parse("((lambda (x) (+ x 2)) '3)".to_string(), Expr{car:None, cdr:None}, 0).0, false);
}


/*(Expr {
    car: Some(Value { valtype: Exprval, whatvalue: None, intvalue: None, exprvalue: Some(Expr {
        car: Some(Value { valtype: Whatval, whatvalue: Some("quote"), intvalue: None, exprvalue: None}),
        cdr: Some(Expr {
            car: Some(Value { valtype: Whatval, whatvalue: Some("a"), intvalue: None, exprvalue: None }),
            cdr: None }) }) }),
    cdr: None }, "")*/

/*(Expr {
    car: Some(Value { valtype: Exprval, whatvalue: None, intvalue: None, exprvalue: Some(Expr {
        car: Some(Value { valtype: Whatval, whatvalue: Some("quote"), intvalue: None, exprvalue: None}),
        cdr: Some(Expr {
            car: Some(Value { valtype: Exprval, whatvalue: None, intvalue: None, exprvalue: Some(Expr {
                car: Some(Value { valtype: Whatval, whatvalue: Some("a"), intvalue: None, exprvalue: None }),
                cdr: None }) }),
            cdr: None }) }) }),
    cdr: None }, "")

(quote -> (a -> N) -> N) -> N*/
