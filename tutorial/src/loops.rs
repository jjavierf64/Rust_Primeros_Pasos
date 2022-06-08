pub fn run(){
    let mut count = 0;

    //Infinite looop

    loop {
        count +=1;
        println!("count is: {}", count);
        
        if count>=0{
            break;
        }
    }
    
    //While (FizzBuzz)
    
    while count <=100{
        count +=1;
        if count % 15 == 0{
            println!("fizzbuzz")
        }
        else if count % 3 == 0{
            
            println!("fizz")
        }
        else if count % 5 == 0{
            
            println!("buzz")
        }
        else{
            println!("{}", count)
        }
    }


    for x in 0..200{
        if x % 15 == 0{
            println!("fizzbuzz")
        }
        else if x % 3 == 0{
            
            println!("fizz")
        }
        else if x % 5 == 0{
            
            println!("buzz")
        }
        else{
            println!("{}", x)
        }
    }

}