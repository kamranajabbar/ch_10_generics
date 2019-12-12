// Generics //
//mod without_generic;
//mod without_generic_fn;

//mod no_generic_fn;
//mod generic_fn;

//mod one_generic_in_structs;
//mod more_generic_in_structs;

//mod one_generic_in_method;
//mod more_generic_in_method;

//mod performance_code_generic;

// Traits //
//mod traits_with_custome_and_default_impl;
//mod traits_with_custome_impl;
mod traits_with_multiple_methods;

fn main() {
    // Generics //
    //without_generic::run();
    //without_generic_fn::run();

    //no_generic_fn::run();
    //generic_fn::run();

    //one_generic_in_structs::run();
    //more_generic_in_structs::run();

    //one_generic_in_method::run();
    //more_generic_in_method::run();

    //performance_code_generic::run();
    //performance_code_generic::run_updated();

    // Traits //
    //traits_with_custome_and_default_impl::run();
    //traits_with_custome_impl::run();
    traits_with_multiple_methods::run();
}
