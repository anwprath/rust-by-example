Learning rust, because why not. Ref: https://doc.rust-lang.org/rust-by-example/ 

Not a furry though.



# Short Notes

##  Printing stuff

- Use `#[derive(Debug)]` trait to make a struct printable
  - `{:?}` for normal (ugly?) print
  - `{:#?}` for pretty print
  - `{}` will use fmt::Display
- `impl fmt::Display for <StructName>` to customize display output
  ```rust
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
   ```
---

##  Primitives
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
- Declaring arrays:
  - `let ys: [i32; 500] = [0; 500];`
  - `let xs: [i32; 5] = [1, 2, 3, 4, 5];`
- Slices: of type `&[T]`
---

## Custom types

- Structs: Just like Go, with actual pattern matching
  - `struct Unit;` -> Fieldless struct, all are equivalent

- Tuple struct: `struct Pair(i32, f32);` Equivalent to named tuples

- Enums: Very weird but fun here


## Variable Bindings

- `let`, `let mut`
- Blocks can shadow variables
- variables can be redeclared in the same block - dumb af idk why

## Types

### Casting
- No implicit conversions
- Explicit conversion: `<variable> as <intended type>;`
- Type inference:
  ```rust
  // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
  ```
### Type alias:
  - Types must have UpperCamelCase names, or the compiler will raise a warning.
    - `type NanoSecond = u64;`
    - `let nanoseconds: NanoSecond = 5 as u64;`
