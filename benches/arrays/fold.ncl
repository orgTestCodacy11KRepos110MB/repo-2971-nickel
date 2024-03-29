let letter | Num -> string.CharLiteral = fun n =>
  string.code "a" + (n % 26)
  |> string.from_code in

{
  right = {
    strings = {
      run = fun n =>
        array.fold (fun x acc => acc ++ x) "" (array.generate (fun n => letter n) n)
    },
    nums = {
      run = fun n =>
        array.fold (fun x acc => x*acc + (x - acc)) 0 (array.generate (fun n => n/2) n)
    },
    arrays = {
      run = fun n =>
        array.fold (fun x acc => acc @ [x]) [] (array.generate (fun n => [n]) n)
    },
  },
  left = {
    strings = {
      run = fun n =>
        array.foldl (fun acc x => acc ++ x) "" (array.generate (fun n => letter n) n)
    },
    nums = {
      run = fun n =>
        array.foldl (fun acc x => x*acc + (x - acc)) 0 (array.generate (fun n => n/2) n)
    },
    arrays = {
      run = fun n =>
        array.foldl (fun acc x => acc @ [x]) [] (array.generate (fun n => [n]) n)
    },
  }
}
