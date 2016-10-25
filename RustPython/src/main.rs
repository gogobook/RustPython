//extern crate eval;
//use eval::eval::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

struct VirtualMachine<'a> {
    // TODO: We are using Option<i32> in stack for handline None return value
    // We need 1 stack per frame
    stack: Vec<Option<i32>>, 
    environment: HashMap<&'a str, Option<i32>>,
}

impl<'a> VirtualMachine<'a> {
    fn new() -> VirtualMachine<'a> {
        VirtualMachine {
            stack: vec![],
            environment: HashMap::new(),
        }
    }
    // The Option<i32> is the return value of the frame, remove when we have implemented frame
    // TODO: read the op codes directly from the internal code object
    fn exec(&mut self, code: Code) -> Option<i32> {
        let mut ret = None;
        for op in code.op_codes {
            // println!("{:?}", op);
            // TODO: convert this to enum?
            match op {
                ("LOAD_CONST", Some(consti)) => {
                    // println!("Loading const at index: {}", consti);
                    self.stack.push(code.consts[consti as usize]);
                },
                // TODO: universal stack element type
                ("LOAD_CONST", None) => {
                    // println!("Loading const at index: {}", consti);
                    self.stack.push(None);
                },
                ("PRINT_ITEM", None) => {
                    // TODO: Print without the Some(...)
                    match self.stack.pop().unwrap() {
                        Some(x) => print!("{}", x),
                        None    => print!("None")
                    }
                },
                ("PRINT_NEWLINE", None) => {
                    print!("\n");
                },
                ("RETURN_VALUE", None) => {
                    ret = self.stack.pop().unwrap();
                    break;
                },
                _ => {
                    println!("Unrecongnized op code!");
                }
            }
        }
        ret
    }
}

#[derive(PartialEq, Debug)]
struct Code<'a> {
    consts: Vec<Option<i32>>,
    names: Vec<&'a str>,
    op_codes: Vec<(&'a str, Option<i32>)>
}

fn parse_bytecode(s: &str) -> Code {
    let lines: Vec<&str> = s.split('\n').collect();

    let (metadata, ops) = lines.split_at(2);

    // Parsing the first line CONSTS
    let consts_str: &str = metadata[0]; // line 0 is empty
    let values_str = &consts_str[("CONSTS: (".len())..(consts_str.len()-1)];
    let values: Vec<&str> = values_str.split(", ").collect();
    // We need better type definition here
    let consts: Vec<Option<i32>>= values.into_iter().map(|x| x.parse::<i32>().ok()).collect();

    // Parsing the op_codes
    let op_codes: Vec<(&str, Option<i32>)>= ops.into_iter()
                                               .map(|x| x.trim())
                                               .filter(|x| x.len() > 0)
                                               .map(|x| {
                                                   let op: Vec<&str> = x.split(", ").collect();
                                                   println!("{:?}", op);
                                                   (op[0], op[1].parse::<i32>().ok())
                                               }).collect();
    

    Code {
        consts: consts,
        op_codes: op_codes,
        names: vec![] // FIXME!
    }

}

fn main() {
    // TODO: read this from args
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(filename).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let code = parse_bytecode(&s);

    let mut vm = VirtualMachine::new();
    vm.exec(code);
}

#[test]
fn test_parse_bytecode() {

    let input = "CONSTS: (1, None, 2)
VARNAMES: ()
SetLineno, 1
LOAD_CONST, 2
PRINT_ITEM, None
PRINT_NEWLINE, None
LOAD_CONST, None
RETURN_VALUE, None
        ";
    let expected = Code { // Fill me with a more sensible data
        consts: vec![Some(1), None, Some(2)], 
        names: vec![],
        op_codes: vec![
            ("SetLineno", Some(1)),
            ("LOAD_CONST", Some(2)),
            ("PRINT_ITEM", None),
            ("PRINT_NEWLINE", None),
            ("LOAD_CONST", None),
            ("RETURN_VALUE", None)
        ]
    };

    assert_eq!(expected, parse_bytecode(input));
}

#[test]
fn test_vm() {

    let code = Code {
        consts: vec![Some(1), None, Some(2)], 
        names: vec![],
        op_codes: vec![
            ("LOAD_CONST", Some(2)),
            ("PRINT_ITEM", None),
            ("PRINT_NEWLINE", None),
            ("LOAD_CONST", None),
            ("RETURN_VALUE", None)
        ]
    };
    let mut vm = VirtualMachine::new();
    assert_eq!(None, vm.exec(code));
}
