#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub cds);

pub mod ast;
use ast::traits::ast_term::ASTTerm;

fn main() {
    let module = cds::ModuleParser::new()
        .parse(
            "
            service ServiceName : a, b, c {
                entity EntityName {
                    a : Integer;
                }

                action magicAction ( testParam: String) returns Integer;

                function magicFunction () returns Float;
            }

            type testType : t234;

            entity TestEntity {
                a : String;
                b : String;
                c : testType;
            }

            service ServiceName2 {

            }
        ",
        )
        .unwrap();

    println!("{}", module.convert_to_json());
}
