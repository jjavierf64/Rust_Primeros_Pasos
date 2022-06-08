//Traditional Structure
/* 
struct Color{ 
    red: u8,
    green: u8,
    blue: u8
}*/

// Tuple Struct
// struct Color(u8,u8,u8);

// Struct + Functions
struct Person {
    first_name: String,
    last_name: String,
    person_age: u8
}

impl Person{
    // Construct Person
    fn new(first: &str, last: &str, age: u8) -> Person{
        Person{
            first_name: first.to_string(),
            last_name: last.to_string(),
            person_age: age
        }

    //Get Full Name
    }   

    fn full_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)

    }
    // Set last name
    fn set_last_name(&mut self, last: &str){
        self.last_name = last.to_string()
    }

    // Name to Tuple
    fn to_tuple(self) -> (String, String){
        (self.first_name, self.last_name)
    }

}


pub fn run(){
    //Traditional Structure
    /*let mut c= Color{
        red: 255,
        green: 0,
        blue: 0
    };


    println!("Color: {:?}", (c.red, c.green, c.blue));
    
    c.red = 200;
    
    println!("Color: {:?}", (c.red, c.green, c.blue));*/
    
    
    /* let mut c= Color(255, 0, 0);
    
    println!("Color: {:?}", (c.0, c.1, c.2));
    
    c.0 = 200;
    
    println!("Color: {:?}", (c.0, c.1, c.2));
     */




     let mut p= Person::new("John", "Doe", 88);
     println!("Person\n  Name: {}\n  Age: {}", p.full_name(), p.person_age);

     p.set_last_name("Williams");

     println!("Person\n  Name: {}\n  Age: {}", p.full_name(), p.person_age);
     
    p.person_age = 9;

     println!("Person\n  Name: {}\n  Age: {}", p.full_name(), p.person_age);

    println!("{:?}",p.to_tuple());

}