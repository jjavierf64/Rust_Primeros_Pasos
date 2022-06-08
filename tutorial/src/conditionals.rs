pub fn run(){
    let age:u8 = 5;
    let check_id: bool = false;
    let false_id: bool = false;



    // if else

    if age >= 18{
        println!("Mayor de edad");
    }
    else if age < 18 && false_id | check_id{
        println!("Mayor de edad. Todo bien");
    }
    else if age< 18 && check_id {
        println!("Menor de edad");
    }
    else {
        println!("Menor de edad");

    }

    let mayor_de_edad = if age>=18{true}else{false};


    println!("{}", mayor_de_edad);

    println!("{}", true||false);


    

}