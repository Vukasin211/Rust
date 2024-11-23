use std::io;

fn main() {
    println!("Start!");

    //Deklaracija stringa
    let mut inputString = String::new();

    println!("Unesite string: ");
    //Ucitavanje unosa sa tastature
    //Kreira se mutable referenca na zeljeni string
    io::stdin().read_line(&mut inputString).expect("failed to read line");

    //Stampanje unetog stringa
    println!("Stampanje: {}", inputString);
    
}
