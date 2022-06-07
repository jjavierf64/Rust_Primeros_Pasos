//Immutable String: fixed lenght
// Growable, heap allocated


pub fn run(){
    let hello="Hello"; //immutable

    let mut hello2 = String::from("Hello 2");


    //println!("data {:?}", (hello,hello2) );

    println!("length {:?}", (hello.len(),hello2.len()) );

    //hello.push(' '); // no se puede
    hello2.push_str(" World!");

    println!("length {:?}", (hello.len(),hello2.len()) );

    //Is empty
    println!("{}", hello2.is_empty());
    
    //Contains Substring
    println!("{}", hello2.contains("World"));

    //Replace
    println!("{}", hello2.replace("World", "Friend"));

    //see if empty
    println!("{}", hello2.is_empty());

    //Loop through string by whitespace

    for word in hello2.replace("World", "Friend").split_whitespace(){
        println!("{}", word);
    }

    //create string with specific capacity
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push(' ');
    s.push('b');

    // Asegurarse

    assert_eq!(3, s.len());
    assert_eq!(10, s.capacity());

}