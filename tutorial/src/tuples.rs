// Values with diff types

pub fn run(){

    let person: (&str, &str, i8)=("JAVIER","CR",21);


    println!("{} is from {} and is {}", person.0, person.1, person.2);

}