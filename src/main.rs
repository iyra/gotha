use std::process;

#[derive(Debug)]
struct Value {
    valtype: u32,
    intvalue: Option<u32>,
    exprvalue: Option<Box<Expr>>,
}

#[derive(Debug)]
struct Expr {
    car: Option<Value>,
    cdr: Option<Box<Expr>>,
}

fn parse(ins: String, m: Expr) -> Expr {
    println!("{:?}", m);
    match m.car {
        Some(x) => { // car is already starting to be filled or is done
            if x.valtype == 0 {
                /* integer */
                match ins.chars().next() {
                    Some(nc) => {
                        match nc.to_digit(10) {
                            Some(p) => {
                                match x.intvalue {
                                    Some(v) => {
                                        let new_val = v * 10 + p;
                                        parse(ins.chars().skip(1).take(ins.chars().count()).collect(), Expr {
                                            car: Some(Value {
                                                valtype: 0,
                                                intvalue: Some(new_val),
                                                exprvalue: None,
                                            }),
                                            cdr: m.cdr})
                                    },
                                    None => {
                                        eprintln!("error: value type set to int but no int found in expr");
                                        process::exit(1);
                                    }
                                }
                            },
                            None => { // started collecting an int but now we have something else...
                                if nc.is_whitespace() {
                                    let nextexpr = parse(ins.chars().skip(1).take(ins.chars().count()).collect(),
                                                         Expr{car:None, cdr:None});
                                    return Expr {
                                        car: m.car,
                                        cdr: Some(Box::new(nextexpr))
                                    };
                                } else {
                                    eprintln!("error: unexpected character");
                                    process::exit(1);
                                }
                                
                            }
                        }
                    },
                    None => {
                        eprintln!("error: unexpected end of input");
                        process::exit(1);
                    }
                }
            } else {
                eprintln!("error: unrecognised value type");
                process::exit(1);
            }
        },
        None => { // car has not been filled yet
            match ins.chars().next() {
                Some(nc) => {
                    match nc.to_digit(10) {
                        Some(p) =>
                            parse(ins.chars().skip(1).take(ins.chars().count()).collect(), Expr {
                                car: Some(Value {
                                    valtype: 0,
                                    intvalue: Some(p),
                                    exprvalue: None,
                                }),
                                cdr: None }),
                        None => {
                            eprintln!("error: could not parse number");
                            process::exit(1);
                        }
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
    parse("1".to_string(), Expr{car:None, cdr:None});
}
