// name: CevalPowDeprecated1
// keywords:
// status: correct
//
// MLS 10.6.7: a negative base with a Real exponent that has an integer value
// is deprecated. Integer exponents are not affected.
//

model CevalPowDeprecated1
  constant Real a = 2;
  constant Real b = -1;
  constant Integer n = 3;
  constant Real x1 = b^a;
  constant Real x2 = (-1.0)^2.0;
  constant Real x3 = b^n + (-2)^2 + b^(n - 1);
  constant Real x4 = -1.0^2.0;
end CevalPowDeprecated1;

// Result:
// class CevalPowDeprecated1
//   constant Real a = 2.0;
//   constant Real b = -1.0;
//   constant Integer n = 3;
//   constant Real x1 = 1.0;
//   constant Real x2 = 1.0;
//   constant Real x3 = 4.0;
//   constant Real x4 = -1.0;
// end CevalPowDeprecated1;
// [flattening/modelica/scodeinst/CevalPowDeprecated1.mo:14:3-14:32:writable] Warning: The exponentiation (-1.0) ^ 2.0 raises the negative base -1.0 to the Real exponent 2.0. Treating a Real exponent with an integer value as an Integer is deprecated; use an Integer exponent.
// [flattening/modelica/scodeinst/CevalPowDeprecated1.mo:13:3-13:25:writable] Warning: The exponentiation b ^ a raises the negative base -1.0 to the Real exponent 2.0. Treating a Real exponent with an integer value as an Integer is deprecated; use an Integer exponent.
//
// endResult
