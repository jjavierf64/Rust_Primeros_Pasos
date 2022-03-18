# Rust_Primeros_Pasos

## Lo más Básico

Las funciones de Rust se definen como 

```rust
fn main(){
...
}
```

Existen Macros y Funciones. Los macros se llaman con un `!`al final.



La librería que permite input del usuario y output es `io`, la cual proviene de la librería estándar `std` . Para más info de std revisar [std::prelude - Rust](https://doc.rust-lang.org/stable/std/prelude/index.html)



Las librerías se importan de la manera

```rust
use std::io;
```





### Variables

Por defecto, las variables son inmutables, por lo que para volverlas mutables se les debe añadir `mut` al definirlas.

```rust
let variable=1; //inmutable
let mut variable2=2; //mutable
```

Para definir un String se puede declarar la variable usando `String::new()` lo que crea un string vacío en una variable.



[Programming a Guessing Game - The Rust Programming Language](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html#receiving-user-input)
