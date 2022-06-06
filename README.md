# Rust_Primeros_Pasos

## Lo más Básico

Las funciones de Rust se definen como

```rust
fn main(){
...
}
```

Existen Macros y Funciones. Los macros se llaman con un `!`al final.

La librería que permite input del usuario y output es `io`, la cual proviene de la librería estándar `std` . Para más info de std revisar [std::prelude - Rust](https://doc.rust-lang.org/stable/std/prelude/index.html).

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

Para definir un String se puede declarar la variable usando `String::new()` lo que crea un string vacío en una variable. En este caso la sintaxis `::` declara que `new` es una función de `String`.

Para recibir el Input de parte del usuario, se usa el código:

```rust
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

Donde la primera línea llama a la función `stdin()` del módulo `io`. Luego el método `.readline()` se encarga de leer la línea de código del usuario y asignarla al agumento `&mut guess`. Dicho método no sustituye la variable, sino que le apenda el nuevo string. Se utiliza ese argumento con el `&mut` para decirle a rust que debe usar la referencia en lugar de escribir el código en un nuevo espacio de la memoria.

La última línea funciona como un handle de error.

Todas las 3 líneas de código pueden resumirse como:

```rust
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

Para imprimir información con variables intermedio, se usan `{}`.

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

[Programming a Guessing Game - The Rust Programming Language](https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html#generating-a-secret-number)
