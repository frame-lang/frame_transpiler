#[feature:codegen.rust.features.generate_hook_methods="true"]
#[code:codegen.rust.code.state_enum_suffix="Nation"]
#[code:codegen.rust.code.state_var_name="nation"]
#[code:codegen.rust.code.transition_hook_method_name="oh_its_a_transition"]
#Config
    -interface-
    Next
    -machine-

    $A
        |Next| -> $B ^

    $B
        |Next| -> $A ^

    -actions-

    -domain-
    var tape:Log = `vec![]`
##
