#[codegen.javascript.code.public_domain:bool="true"]
#[codegen.javascript.code.public_state_info:bool="true"]
#[codegen.javascript.code.generate_import_export:bool="true"]
#StateParams
    -interface-
    Next
    Prev
    Log

    -machine-
    $Init
        |Next| -> $Split(1) ^

    $Split [val:u32]
        |Next| -> $Merge(val val+1) ^
        |Prev| -> $Merge(val+1 val) ^
        |Log| got_param("val" val) ^

    $Merge [left:u32 right:u32]
        |Next| -> $Split(left+right) ^
        |Prev| -> $Split(left*right) ^
        |Log|
            got_param("left" left)
            got_param("right" right)
            ^

    -actions-
    got_param [name:&String val:u32]

    -domain-
    var param_log = `[]`
##
