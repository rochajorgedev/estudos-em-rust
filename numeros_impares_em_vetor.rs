use std::io;

fn numeros_impares(vetor: Vec<i32>){
    for valor in vetor{
        if valor%2 !=0 {
            print!("{} ", valor);
        }
    }

}


fn preencher(vetor: &mut Vec<i32>, numero:i32) {
    for _ in 0..numero{
        let mut dado_01: String = String::new();
        io::stdin().read_line(&mut dado_01).expect("Não foi digitado algo valido");
        let vetor_numero:i32 = dado_01.trim().parse().expect("Error você não digitou um número");
        vetor.push(vetor_numero);
    }

}


fn main() {

    let mut vetor: Vec<i32> = Vec::new();
    
    let mut dado: String = String::new();
    println!("Digite um número: ");
    io::stdin().read_line(&mut dado).expect("Não foi digitado algo valido");
    let numero:i32 = dado.trim().parse().expect("A entrada não foi um número valido");
    
    preencher(&mut vetor, numero); 
    println!("Vetor original: ");
    println!("{:?}", vetor);
    println!("O resultado: ")
    numeros_impares(vetor.clone());

}
