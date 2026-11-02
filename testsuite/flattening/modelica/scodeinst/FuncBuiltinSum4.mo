// name: FuncBuiltinSum4
// keywords: sum product
// status: correct
//
// Checks that sum and product of an array returned by a function are not
// expanded into one call per element.
//

model FuncBuiltinSum4
  function f
    input Real x;
    output Real y[3] = {x, 2*x, 3*x};
  end f;

  Real x = time;
  Real r1 = sum(f(x));
  Real r2 = product(f(x));
end FuncBuiltinSum4;

// Result:
// function FuncBuiltinSum4.f
//   input Real x;
//   output Real[3] y = {x, 2.0 * x, 3.0 * x};
// end FuncBuiltinSum4.f;
//
// class FuncBuiltinSum4
//   Real x = time;
//   Real r1 = sum(FuncBuiltinSum4.f(x));
//   Real r2 = product(FuncBuiltinSum4.f(x));
// end FuncBuiltinSum4;
// endResult
