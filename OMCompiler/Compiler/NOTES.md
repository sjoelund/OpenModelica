MetaModelicaBuiltin.rs (manually written? semi-manually?)
Need a parser to get started...

We need a Rust version of Susan (could be a stand-alone tool) that produces Rust code.

But first, we should rip out the old frontend.
The MetaModelica compiler could become a separate binary that is not part of omc.exe anymore.
The mos-scripting needs to use the NF if it does not already.
Then the old frontend should be able to be removed...

You cannot pattern-match lists straight off. Might need to implement List as CONS/EMPTY to make the compiler work
as expected. And then rewrite this later.

* Write Parser first, then AbsynToRust.rs?

* Add a new option for LoadCompilerSources to mark Old Frontend packages.
  Then make sure we _can_ compile an OMC version without it. And perhaps
  test that to see what breaks other than MetaModelica. Perhaps port those
  to NF?

* There are Julia bindings for Rust. I wonder if it could be optionally
  loaded (in case people do not have Julia).

# Function calling

Rust does not have default arguments (for functions; it does for structs)
Rust does not have named arguments (for functions; it does for structs)

```rs
use bon;
#[bon::builder(start_fn = foo_named)]
pub fn foo(important_arg: u32, optional: Option<u32>) -> String {
  let optional = optional.unwrap_or(100);
  format!("{}, {}", important_arg, optional)
}
```

```rs
mod bla;

fn main() {
println!("{}", bla::foo(1, Some(100)));
println!("{}", bla::foo_named().important_arg(1).optional(15).call());
}
```
