mod variables;
mod data_types;
mod functions;
mod control_flow;

fn main() {
    // variables::test_immutable();
    // variables::run_mutable();
    // 
    // println!("the value is {}", variables::THREE_HOURS_IN_SECONDS);
    // variables::variables_shadowing();
    // 
    // println!("---- start data type ----");
    // data_types::run_char();
    // 
    // println!("---- start print tuple ----");
    // data_types::run_tuple();
    // 
    // println!("---- start print array ----");
    // data_types::run_array();
    // 
    // data_types::test_out_of_index();

    // println!("---- function usage ----");
    // functions::test_expression();

    println!("run control flow");

    control_flow::run_if_flow();
    control_flow::run_if_elseif();

    control_flow::run_loop_with_break();
    control_flow::run_loop_with_loop_label();
    control_flow::run_with_while();
    control_flow::run_with_for();
    control_flow::run_with_for2();
}
