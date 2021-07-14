use std::io;
fn fah_to_cel(choice : u8, deg : f64) -> f64{
    if choice==1{ // choice==1 means cel to fahren
        ((deg-32.00)/9.00)*5.00
    }
    else{
        ((9.0*deg)/5.0)+32.0
    }
}
fn main() {
    println!("Enter choice\n1.) Fahrenheit to Celsius\n2.) Celsius to Fahrenheit");
    let mut ch = String::new();
    io::stdin().read_line(&mut ch).expect("Error");
    let ch : u8 = match ch.trim().parse(){
        Ok(num) => num,
        Err(_) => 0,
    };

    if ch==1 {
        println!("Enter Fahrenheit in degrees");
        let mut deg = String::new();
        io::stdin().read_line(&mut deg).expect("Enter valid values");
        let deg : f64 = match deg.trim().parse(){
            Ok(num) => num,
            Err(_) => 0.0,
        };
        println!("Result = {}", fah_to_cel(ch, deg));
    }
    else {
        println!("Enter Celsius in degrees");
        let mut deg = String::new();
        io::stdin().read_line(&mut deg).expect("Enter valid values");
        let deg : f64 = match deg.trim().parse(){
            Ok(num) => num,
            Err(_) => 0.0,
        };
        println!("Result = {}", fah_to_cel(ch, deg));
    }
}
