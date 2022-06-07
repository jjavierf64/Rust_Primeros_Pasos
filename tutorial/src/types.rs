/*
----Primitive Types----
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
u -> sin negativos
i -> con negativos

Floats: f32, f64

Character char

Boolean bool

Tuples

Arrays fixed sized


*/

pub fn run(){
    
    let x = 1; //Default i32

    let y = 2.5; //Default f64

    let z: i64 = 616517716514673543;

    //find max size

    println!("Max i32: {}\nMax i64: {}", std::i32::MAX, std::i64::MAX);


    // Boool

    let is_active: bool = true;
    let is_greater: bool = x > x+1 ;
    let a1 = 'a';
    let cara = '\u{1F600}';
    
    
    println!("{:?}",(x,y,z,is_active, is_greater,a1,cara));

}