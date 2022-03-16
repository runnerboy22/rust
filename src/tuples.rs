pub fn run (){
    let person: (&str, &str, i8) = ("John", "Doe", 33);
    println!("{} is {} years old.", person.0, person.2);
}