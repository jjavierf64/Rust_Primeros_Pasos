pub fn run(){  
    // pub: public
    // fn: function
    // run: nombre
    println!("Hello from print.rs!");

    //Formato de variables y numeros
    println!("Name: {} \nNumber: {}", "JAVIER", 2); 
    
     //Argumentos de posicion
    println!("Name: {0} \nAge: {2}\nLastname: {1}\nFullname:{1},{0}", "JAVIER", "JEREZ", 20);
    
    //Argumentos con nombre
    println!("Name: {nombre} \nAge: {edad}\nLastname: {apellido}\nFullname:{apellido},{nombre}", nombre="JAVIER", apellido="JEREZ", edad=20); 
    
    //Placeholder
    println!("{:?}", (12,true,"hello"));

    println!("{}+{}={}",10,20,10+20);

    //Placeholder
    println!("{all} in Binary: {all:b}    Hex: {all:x}   Octal: {all:o}", all=10)
}