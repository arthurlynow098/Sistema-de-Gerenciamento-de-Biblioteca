fn main () {
    let animais = vec! ["Cachorro", "Gato", "Pássaro", "Peixe"];
    for (indice, animal) in animais.iter().enumerate() {
        println!("{}: {}", indice, animal);
    }
}