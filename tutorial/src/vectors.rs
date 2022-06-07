// Resizable Length
// Same Data Type

pub fn run(){
    let mut numbers: Vec<i32> /* Tipo */= vec![1,2,3,4,5];

    println!("{:?}", numbers);

    
    //Reassign
    numbers[0] = 55;

    // Valor único
    println!("{}", numbers[0]);

    //Añadir y remover

    numbers.push(5);
    numbers.push(6);
    numbers.pop(); //Elimina el último


    let mut j = 0;
    for i in numbers.iter(){

        println!("Num {} from the array is {}",j ,i);
        j= j+1;
    }


    // Loop y Mutar
    for x in numbers.iter_mut(){
        *x *= 2;

    }
    println!("{:?}", numbers);



    // Size of Byte

    //println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));


    // Slice
    //let slice: &[i32] = &numbers[0..3];

    //println!("Slice {:?}", slice );
}