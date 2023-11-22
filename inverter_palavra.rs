use std::io;

fn reverter(palavra: &str) -> String{
    palavra.chars().rev().collect()
}



fn main() {

    let mut dado: String = String::new();
    println!("Digite uma palavra: ");
    io::stdin().read_line(&mut dado).expect("Não foi digitado algo valido"); 
    
    print!("A palavra original: {}E o resultado: {}",dado, reverter(&dado));

}
