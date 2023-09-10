//Varibales hold data  or references to data
// by defualt the var iables are immutagle
// rust is a block scoped language

pub fn run() {
    let name = "brad";
    let mut age = 37;
    println!("my name is {n}, and I'm {a}", n = name, a = age);
    //age=38 will make an error of immutable type
    const ID: i32 = 001;
    println!("id = {}", ID);

    //addigns data at one time
    let (my_name, my_age) = ("brad", 37);
}
