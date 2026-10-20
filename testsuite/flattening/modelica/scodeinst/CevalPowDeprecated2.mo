// name: CevalPowDeprecated2
// keywords:
// status: incorrect
// cflags: --strict
//
// MLS 10.6.7: a negative base with a Real exponent that has an integer value
// is an error with --strict.
//

model CevalPowDeprecated2
  constant Real a = 2;
  constant Real b = -1;
  constant Real x = b^a;
end CevalPowDeprecated2;

// Result:
// Error processing file: CevalPowDeprecated2.mo
// [flattening/modelica/scodeinst/CevalPowDeprecated2.mo:13:3-13:24:writable] Error: The exponentiation b ^ a raises the negative base -1.0 to the Real exponent 2.0. Treating a Real exponent with an integer value as an Integer is deprecated; use an Integer exponent.
//
// # Error encountered! Exiting...
// # Please check the error message and the flags.
//
// Execution failed!
// endResult
