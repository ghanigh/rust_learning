fn main () {
    let x: i32 = 42; // Nombres entier 32 bits
    let y = 3.14; // Rust déduit que c'est un f64 (flottant)
    let is_active: bool = true;
    let letter: char = 'R';

    println!("x = {}, y = {}, is_active = {}, letter = {}", x, y, is_active, letter);
}


