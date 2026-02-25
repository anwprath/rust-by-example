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