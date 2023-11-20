use std::io;

fn soma_numeros(numeros: &mut Vec<i32>) -> i32{
    return numeros.iter().sum();
}



fn main() {

    let mut vetor: Vec<i32> = Vec::new();
    
    let mut dado: String = String::new();
    println!("Digite um número: ");
    io::stdin().read_line(&mut dado).expect("Não foi digitado algo valido");
    let numero:i32 = dado.trim().parse().expect("A entrada não foi um número valido");
    
    for _ in 0..numero{
        let mut dado_01: String = String::new();
        io::stdin().read_line(&mut dado_01).expect("Não foi digitado algo valido");
        let vetor_numero:i32 = dado_01.trim().parse().expect("Error você não digitou um número");
        vetor.push(vetor_numero);
    }

    
    println!("{:?}", soma_numeros(&mut vetor));

}
