/// csugar-like CLI
use std::io;

use super::integration::IntegratedSolver;
use super::parser::{parse, ParseResult, Var, VarMap};

pub fn csugar_cli() {
    let mut var_map = VarMap::new();
    let mut solver = IntegratedSolver::new();

    let mut buffer = String::new();
    let stdin = io::stdin();

    let mut target_vars: Option<Vec<String>> = None;

    loop {
        buffer.clear();
        let num_bytes = stdin.read_line(&mut buffer).unwrap(); // TODO
        if num_bytes == 0 {
            // EOF
            break;
        }
        let line = buffer.trim_end();

        if line.starts_with("#") {
            assert!(target_vars.is_none());
            target_vars = Some(
                line.trim_start_matches("#")
                    .split(" ")
                    .map(String::from)
                    .collect(),
            );
            continue;
        }
        let result = parse(&var_map, line);
        match result {
            ParseResult::BoolVarDecl(name) => {
                let var = solver.new_bool_var();
                var_map.add_bool_var(name, var);
            }
            ParseResult::IntVarDecl(name, domain) => {
                let var = solver.new_int_var(domain);
                var_map.add_int_var(name, var);
            }
            ParseResult::Stmt(stmt) => solver.add_constraint(stmt),
        }
    }

    match target_vars {
        Some(target_vars) => {
            let mut bool_target = vec![];
            let mut int_target = vec![];
            for target in &target_vars {
                match var_map.get_var(target).unwrap() {
                    Var::Bool(var) => bool_target.push(var),
                    Var::Int(var) => int_target.push(var),
                }
            }
            match solver.decide_irrefutable_facts(&bool_target, &int_target) {
                Some(result) => {
                    println!("sat");
                    for target in &target_vars {
                        match var_map.get_var(target).unwrap() {
                            Var::Bool(var) => {
                                if let Some(b) = result.get_bool(var) {
                                    println!("{} {}", target, b);
                                }
                            }
                            Var::Int(var) => {
                                if let Some(i) = result.get_int(var) {
                                    println!("{} {}", target, i);
                                }
                            }
                        }
                    }
                }
                None => println!("unsat"),
            }
        }
        None => match solver.solve() {
            Some(model) => {
                println!("s SATISFIABLE");
                for (name, &var) in var_map.iter() {
                    match var {
                        Var::Bool(var) => println!("a {}\t{}", name, model.get_bool(var)),
                        Var::Int(var) => println!("a {}\t{}", name, model.get_int(var)),
                    }
                }
                println!("a");
            }
            None => println!("s UNSATISFIABLE"),
        },
    }
}
