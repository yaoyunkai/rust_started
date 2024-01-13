mod generic_start;
mod trait_usage;
mod hash_usage;
mod struct_trait_usage;

fn main() {
    // generic_start::run_1();
    // generic_start::run_with_struct();

    // trait_usage::run_trait();
    // hash_usage::run_hash();

    trait_usage::run_trait();
    struct_trait_usage::run_1();
}
