
# What's important about the semantic ir?

- All items (Structs, Functions, Globals) are addressed by their full, absolute path (That is, starting with their library or application name) in the `concrete_scope`


- Every expression gets annotated with the type it returns. Every variable get's annotated ewith the type it can hold


- `concrete scope`: During the ast traversal, a scope of "concrete" functions and "concrete" structs is created.
  That means:
  Functions like
      dot(a, b) = (zip a b) : map (((a, b)) => a+b) : product
  will be made into:
      dot(a List_Int, b List_Int) -> Int {
          return <Int> product(
              <List_Int> map(
                  <List_Tuple_Int_Int> zip(List_Int a, List_Int b)
                  <Closure_Tuple_Int_Int->Int> [](n) => <Int>Add(<Int>n.0, <Int>n.1)
              )
          )
      }
  concretly, all names will be absolute paths, therefore it's more likely to see
    std.iterators.dot(a std.collections.List_std.types.Int, ...) -> std.types.Int { ... }

  There are _no_ generics left here. Though it's helpful have the name of the name not in form of a string, but rather
  type AbsoluteIdentifier
  - name String <!-- full absolute path -->
  - generics []AbsoluteIdentifier <!-- list of all generic arguments -->


- during ast traversal, all intermediate results will start filling up the `concrete scope`.
  That is, because as a by-product of semantic analyses for generic functions, a concrete version of the function
  (and all called functions) needs to be assembled anyway.


- it's supposed to be easy to derive assembly/machine-code from the `concrete scope`


- the `concrete scope` should be extendable at any given moment. This way it is usable in a `repl`


- if a function* (key=AbsoluteIdentifier) is already in the `concrete scope`, the Identifier can be returned without more computing
- if a function* (key=AbsoluteIdentifier) is not already in the scope, the `concrete scope` will be filled with the concrete function.
  The AbsoluteIdentifier will be returned. That is, given the Ast describes correct solar code. Otherwise semantic analysis will yield an error. An Error is returned instead.

>   * that means that the function under it's absolute path with the exact given generic types is already in `concrete scope`


- Given new `solar_parser::Ast` it should be trivial to use this library (possibly along with preexisting `concrete_scope`) to derive new (mutated) `concrete_scope`


- Given the `concrete_scope` it's easy to derive (jit-)compiled functions (matter for another `solar` crate)


- No Modules are hierarchies exist in the `concrete_scope`. Every name is unique. No privat functions or structs or fields exist.




```
type ConcreteScope
- functions Map (AbsoluteIdentifier, ConcreteFunction)
- structs Map (AbsoluteIdentifier, ConcreteStruct)
- globals Map (AbsoluteIdentifier, ConcreteValue)

