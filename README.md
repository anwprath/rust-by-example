Learning rust, because why not. Ref: https://doc.rust-lang.org/rust-by-example/ 

Not a furry though.



# Short Notes

### Printing stuff

- Use `#[derive(Debug)]` trait to make a struct printable
  - `{:?}` for normal (ugly?) print
  - `{:#?}` for pretty print
  - `{}` will use fmt::Display
- `impl fmt::Display for <StructName>` to customize display output
  ```rust
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
   ```


### Primitives
- `let` -> cannot be mutated
- `let mut` -> for mutable variables
- Basic types:
  - Scalars: i32,i64,f64,bool...
  - `[i32;5]` -> array of size 5
  - Tuple: (i32,bool,3)
- Tuples
```rust
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
```