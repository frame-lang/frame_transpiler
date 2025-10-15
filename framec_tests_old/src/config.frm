#[codegen.rust.features.generate_hook_methods:bool="true"]
#[codegen.rust.code.state_enum_suffix:str="Nation"]
#[codegen.rust.code.state_var_name:str="nation"]
#[codegen.rust.code.transition_hook_method_name:str="oh_its_a_transition"]
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
