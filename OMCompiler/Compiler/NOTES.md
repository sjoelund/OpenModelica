MetaModelicaBuiltin.rs (manually written? semi-manually?)
Need a parser to get started...

We need a Rust version of Susan (could be a stand-alone tool) that produces Rust code.

But first, we should rip out the old frontend.
The MetaModelica compiler could become a separate binary that is not part of omc.exe anymore.
The mos-scripting needs to use the NF if it does not already.
Then the old frontend should be able to be removed...
