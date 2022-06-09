pub fn run(){
    // Normal
    let a = 10;
    let b= a;
    println!("a: {}\nb: {}\n\n",a,b);

    // Move
    // Funciona sólo con tipos complejos como String

    let x = String::from("Vacas");
    let y= x;

    // println!("x: {}\ny: {}\n\n",x,y); // Este no funciona porque y toma la referencia de x y la elimina
    // Es decir que cambia de Ownership

    // Clonación
    let x = y.clone();
       
    println!("x: {}\ny: {}\n\n",x,y); // Ahora funciona porque está clonando tanto la referencia como el valor
    
    // Sin clonar (REFERENCES)
    
    
    let m = String::from("Toros");
    
    take_ownership(m);
    
    // let n = m;  // No funciona porque se toma ownership del anterior
    
    
    let k = String::from("Ovejas");
    
    dont_take_ownership(&k); // Esta vez no toma Ownership, sino que es una referencia
    
    
    let n = &k;
    let l = &k;
    

    println!("k: {}\nn: {}\nl: {}\n\n",k,n,l);
    
    //k = String::from("Chupacabras");// No funciona porque &k Está referenciado en otras variables
    
    // pluralizar(&mut n); no funka
    
    // println!("k: {}\nn: {}\nl: {}\n\n",k,n,l);
    



    // Editable

    let mut s = String::from("Abeja");

    pluralizar(&mut s);
    
    
}

fn take_ownership(string: String){
    println!("Take Ownership from {}", string);
    
}
fn dont_take_ownership(string: &String){
    println!("Do Not Take Ownership from {}", string);
    
}
fn pluralizar(string: &mut String){
    string.push_str("s");
    
}