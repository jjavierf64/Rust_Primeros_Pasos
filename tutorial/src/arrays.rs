// Fixed Length
// Same Data Type

pub fn run(){
    let mut numbers: [i32 /* Tipo */; 5/* Longitud */] = [1,2,3,4,5];

    print!("{:?}", numbers);

    
    //Reassign
    numbers[0]=55;

    // Valor único
    println!("{}", numbers[0]);

    let mut j = 0;
    for i in numbers{

        println!("Num {} from the array is {}",j ,i);
        j= j+1;
    }

    // Size of Byte

    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));


    // Slice
    let slice: &[i32] = &numbers[0..3];

    println!("Slice {:?}", slice );
}