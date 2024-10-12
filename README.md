# Abusing Rust's type system into implementing Boolean logic
Rust is a modern systems programming language focused on performance, reliability, and memory safety. 
Its strong, static type system prevents common errors like null pointer dereferences and data races at compile time, ensuring stability and security. 

Rust achieves memory safety without garbage collection, making it suitable for resource-constrained environments and performance-critical applications.

However, Rust's type system is Turing-complete, and can be used as a dynamically-typed programming language, similar to Haskell or Prolog. This isn't exactly ideal, but it is an interesting experiment nonetheless. 

This project attempts to abuse Rust's type system into becoming a more verbose version of Prolog while implementing a simple boolean logic evaluator. 

# Additional Notes
```
Types: Sets of Values
Functions: Maps of types: A -> B

Traits: Sets of Types
Traits (Associated Types):       Map: Type -> Type
Traits (Associated Constants):   Map: Type -> Value

Pointers: Map: value -> value
```
# References and Credit
- A special thanks to the folks in the Rust Discord server that helped me implement this *interesting* piece of software.
- A special thank to @Sammy99Jsp for teaching me how Rust's type system works! 
