fn main(){

let mut contador = 0;
loop {
    contador += 1; 
    if contador == 3 {
        break;
    }
    println!("Loop: {}", contador)
    }
}