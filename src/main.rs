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
        Some(x) => {Expr{car:None,cdr:None} /* dummy placeholder */ } // car is already starting to be filled or is done
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
