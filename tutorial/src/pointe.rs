pub fn run(){
 
    // Primitive Array
    
    let arr1 = [1,2,3];
    let arr2 = &arr1; // También se puede usar en primitivos
    
    println!("{:?}", (arr1, arr2));
    
    // Vectors
    
    let vec1 = vec![1,2,3];
    let vec2 = &vec1; // Es necesario utilizar & para indicar "Referencia" cuando no son primitivos
    
    println!("{:?}", (&vec1, &vec2));




}