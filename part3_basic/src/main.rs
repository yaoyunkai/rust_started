use regex_syntax::hir::Hir;
use regex_syntax::parse;

mod control_flow;
mod data_types;
mod double_linked_list;
mod double_linked_list_with_unsafe;
mod functions;
mod variables;

fn main() {
    // variables::runner();
    double_linked_list::foo();

    let hir = parse("a|b").unwrap();
    assert_eq!(
        hir,
        Hir::alternation(vec![
            Hir::literal("a".as_bytes()),
            Hir::literal("b".as_bytes()),
        ])
    );
}
