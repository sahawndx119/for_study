//in the name of God
//this library is for traits and functions!




pub fn get_age() {
    let mut age = String::new();
    std::io::stdin().read_line(&mut age).unwrap();

    let age = age.trim().parse::<u32>().unwrap();

    println!("got your age ! age >> {age}");
}