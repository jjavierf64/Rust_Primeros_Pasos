pub fn run() {

    let name = "JAVIER";
    let mut age = 21;

    println!("My name is {} and I'm {}.", name,age);

    age=22;
    println!("My name is {} and I'm {}.", name,age);


    // Define Constant

    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assing Multiple Variables

    let (myname, myage) = ("JEREZ", "37"); 

    println!("{}{}",myage,myname);

}