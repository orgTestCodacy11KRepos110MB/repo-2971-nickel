let test_cases = {
  destructuring_let_binding_preserves_types = 
    let some_record : { a : Num, b: Str, c: Num -> Num } = { a = 1, b = "test", c = fun n => n } in
    let { a, b, c } = some_record in
    { a_num = a, a_str = b, the_id_fn = c } : { a_num : Num, a_str: Str, the_id_fn: forall a. a -> a },

  destructuring_function_args_preserves_types = 
    let dstrct : { a: Num, b: Str } -> { num : Num, str : Str } = 
      fun { a, b } => { num = a, str = b }
    in
    let r :  { a: Num, b: Str } = { a = 1, b = "" } in
    dstrct r,

} in
# If the above typechecks, then we're good.
true