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
        Some(x) => {print!("-> "); printexpr(*x, nils); },
        None => if nils { print!("-> nil") }
    }
}

// fn parse(ins: &mut dyn Iterator<Item=char>, m: Expr, in_expr: i32) -> (Expr, &mut dyn Iterator<Item=char>) {
//     println!("{:?}", m);
//     //let inslength = ins.count();
//     match m.car {
//         Some(x) => { // car is already starting to be filled or is done
//             match ins.next() {
                
//                 Some(nc) => {
//                         if !nc.is_whitespace() {
//                             match x.valtype {
//                                 Valtype::Whatval => match x.whatvalue {
//                                     Some(v) => {
//                                         if nc == ';' {
//                                             let ipoint = 0;
//                                             for (i, c) in ins.enumerate() {
//                                                 if c == '\n' {
//                                                     ipoint = i;
//                                                 }
//                                             }
//                                             if ipoint > 0 {
//                                                 let g = ins.count();
//                                                 let pass_iter = ins.skip(ipoint).take(g);
//                                             return parse(&mut pass_iter, Expr {
//                                                         car: Some(Value {
//                                                             valtype: Valtype::Whatval,
//                                                             whatvalue: Some(v),
//                                                             intvalue: None,
//                                                             exprvalue: None,
//                                                         }),
//                                                 cdr: m.cdr}, in_expr)
//                                             }
                                                
//                                             return (Expr {
//                                                 car: Some(Value {
//                                                     valtype: Valtype::Whatval,
//                                                     whatvalue: Some(v),
//                                                     intvalue: None,
//                                                     exprvalue: None,
//                                                 }),
//                                                 cdr: m.cdr}, &mut "".chars())
//                                         }
//                                         if nc == ')' {
//                                             if in_expr > 0 {
//                                                 return (Expr {
//                                                     car: Some(Value {
//                                                         valtype: Valtype::Whatval,
//                                                         whatvalue: Some(v),
//                                                         intvalue: None,
//                                                         exprvalue: None,
//                                                     }),
//                                                     cdr: m.cdr}, ins)
//                                             } else {
//                                                 eprintln!("error: unexpected )");
//                                                 process::exit;
//                                             }
//                                         }
//                                         if nc != '(' {
//                                             /* stop people putting ( in the middle of a symbol... */
//                                             parse(&mut ins.skip(1).take(ins.count()), Expr {
//                                                 car: Some(Value {
//                                                     valtype: Valtype::Whatval,
//                                                     whatvalue: Some([v, nc.to_string()].join("")),
//                                                     intvalue: None,
//                                                     exprvalue: None,
//                                                 }),
//                                                 cdr: m.cdr}, in_expr)
//                                         } else {
//                                             eprintln!("error: unexpected (");
//                                             process::exit(1);
//                                         }
                                        
//                                     },
//                                     None => {
//                                         eprintln!("error: value type set to what but found nothing there");
//                                         process::exit(1);
//                                     }
//                                 },
//                                 Valtype::Exprval => {                     
//                                         return (Expr {
//                                             car: Some(x),
//                                             cdr: m.cdr}, ins)
//                                 },
//                                 _ => {
//                                     eprintln!("error: unexpected value type in tree");
//                                     process::exit(1);
//                                 }
//                             }
//                         } else { // started collecting an int but now we have whitespace
//                                 if in_expr > 0{
//                                     let (nextexpr, ni) = parse(&mut ins.skip(1).take(ins.count()),
//                                                          Expr{car:None, cdr:None}, in_expr+1);
//                                     return (Expr {
//                                         car: Some(x),
//                                         cdr: Some(Box::new(nextexpr))
//                                     }, ni);  
//                                 }
//                             else {return (Expr { car: Some(x), cdr: None }, ins) }
//                         }
//                     },
//                     None => {
//                         if in_expr > 0 {
//                             eprintln!("error: unexpected end of input");
//                         }
//                         //process::exit(1);
//                         return (Expr {
//                             car: Some(x),
//                             cdr: m.cdr,
//                         }, ins);
//                     }
//                 }
//         },
//         None => { // car has not been filled yet
//             match ins.next() {
//                 Some(nc) => {
//                     if nc == '\'' {
//                         let (nextexpr, ni) = parse(&mut ins.skip(1).take(ins.count()),
//                                                    Expr{car:None, cdr:None}, in_expr+1);
//                         parse(&mut ni.skip(1).take(ins.count()), Expr {
//                             car: Some(Value {
//                                 valtype: Valtype::Exprval,
//                                 whatvalue:None,
//                                 intvalue: None,
//                                 exprvalue: Some(Box::new(Expr{
//                                     car:Some(Value {
//                                         valtype: Valtype::Whatval,
//                                         whatvalue:Some("quote".to_string()),
//                                         intvalue: None,
//                                         exprvalue:None,
//                                     }),
//                                     cdr: Some(Box::new(nextexpr)),
//                                 }))}),
//                             cdr: None}, in_expr)
//                     } else if nc == '(' {
//                         let (nextexpr, ni) = parse(&mut ins.skip(1).take(ins.count()),
//                                                    Expr{car:None, cdr:None}, in_expr+1);
//                         parse(&mut ni.skip(1).take(ins.count()), Expr {
//                             car: Some(Value {
//                                 valtype: Valtype::Exprval,
//                                 whatvalue:None,
//                                 intvalue: None,
//                                 exprvalue: Some(Box::new(nextexpr)),
//                             }),
//                             cdr: None }, in_expr)
//                     } else if nc == ';' {
//                         for (i, c) in ins.enumerate() {
//                             if c == '\n' {
//                                 return parse(&mut ins.skip(i).take(ins.count()), Expr {car:None, cdr:None}, in_expr)
//                             }
//                         }
//                         return (Expr {car:None, cdr:None}, &mut "".to_string().chars()) /* just a comment */
//                     } else {
//                         parse(&mut ins.skip(1).take(ins.count()), Expr {
//                             car: Some(Value {
//                                 valtype: Valtype::Whatval,
//                                 whatvalue:Some(nc.to_string()),
//                                 intvalue: None,
//                                 exprvalue: None,
//                             }),
//                             cdr: None }, in_expr)
//                     }
//                 },
//                 None => {
//                     eprintln!("error: expected more string");
//                     process::exit(1);
//                 }
//             }
//         }
//     }
// }
/* ((+ x 2) 3) */
fn parse(ins: &mut dyn Iterator<Item=char>, m: Expr, in_expr: i32) -> Expr {
    match ins.next() {
        Some(nc) => {
            println!("nc is {}", nc);
            if nc.is_whitespace() {
                if in_expr > 0 {
                    println!("!!");
                    let nextexpr = parse(ins, Expr { car: None, cdr: None }, in_expr);
                    println!("in whitespace & expr, parsing with cdr={:?}", nextexpr);
                    return parse(ins, Expr { car: m.car, cdr: Some(Box::new(nextexpr)) }, in_expr-1);
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
                    println!("saw ), returning car:{:?}", m.car);
                    return m;
                }
                else {
                    eprintln!("error: found ) even though not in expression");
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
    //println!("{:?}", parse("'(a)".to_string(), Expr{car:None, cdr:None}, 0));
    printexpr(parse(&mut "((+ x 2))) 3)".chars(), Expr{car:None, cdr:None}, 0), false);
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

/* Expr {
    car: Some(Value { valtype: Exprval, whatvalue: None, intvalue: None, exprvalue: Some(Expr {
        car: Some(Value { valtype: Exprval, whatvalue: None, intvalue: None, exprvalue: Some(Expr {
            car: Some(Value { valtype: Whatval, whatvalue: Some("lambda"), intvalue: None, exprvalue: None }),
            cdr: Some(Expr {
                car: Some(Value { valtype: Exprval, whatvalue: None, intvalue: None, exprvalue: Some(Expr {
                    car: Some(Value { valtype: Whatval, whatvalue: Some("x"), intvalue: None, exprvalue: None }),
                    cdr: None }) }),
                cdr: Some(Expr {
                    car: Some(Value { valtype: Exprval, whatvalue: None, intvalue: None, exprvalue: Some(Expr {
                        car: Some(Value { valtype: Whatval, whatvalue: Some("+"), intvalue: None, exprvalue: None }),
                        cdr: Some(Expr { car: Some(Value { valtype: Whatval, whatvalue: Some("x"), intvalue: None, exprvalue: None }),
                                         cdr: Some(Expr { car: Some(Value { valtype: Whatval, whatvalue: Some("2"), intvalue: None, exprvalue: None }),
                                                          cdr: None }) }) }) }),
                    cdr: None }) }) }) }),
        cdr: Some(Expr {
            car: Some(Value { valtype: Exprval, whatvalue: None, intvalue: None, exprvalue: Some(Expr {
                car: Some(Value { valtype: Whatval, whatvalue: Some("quote"), intvalue: None, exprvalue: None }),
                cdr: Some(Expr { car: Some(Value { valtype: Whatval, whatvalue: Some("3"), intvalue: None, exprvalue: None }),
                                 cdr: None }) }) }),
            cdr: None }) }) }),
    cdr: None }

(((lambda -> (x) -> (+ -> x -> 2 -> nil) -> nil) -> (quote -> 3 -> nil) -> nil) -> nil)*/

