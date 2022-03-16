pub fn run(){
    let mut hello = String::from("Hellfregdtrto");

    println!("length of hello = {}", hello.len());

    hello.push('.');
    hello.push_str(", wod!");
    println!("Capacity of hello = {}", hello.capacity());
    println!("{}", hello);
    // assert_eq!(hello.capacity(), 20);
    let mut s = String::from("kjfndskjfffndskjnfhjrd");
    s.push_str("kjfndskjfffndskjnfhjrd");
    println!("{}", s);
    println!("Capacity of s = {}", s.capacity());

}