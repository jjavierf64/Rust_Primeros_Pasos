pub fn run(){
    greetings("Hello", "José") ;

    let get_sum = add(5, 5) ;
    println!("{}", get_sum);

    //Closure

    let n3: i32 = 2;
    let add_nums = |n1: i32, n2: i32| n1 + n2*n3;

    println!("{}", add_nums(5,15));

}

fn greetings(greet: &str, name: &str){
    println!("{} {}", greet, name)
}

fn add(n1: i32, n2: i32) -> i32{
    n1 + n2
}