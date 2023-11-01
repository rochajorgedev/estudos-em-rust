use std::io;

fn fatorial(numero:i32) ->i32{
    let mut resultado:i32 = 1;
    for i in 1..=numero{
        resultado = resultado* i;
    }

    return resultado;
}




fn main() {
    let mut dado = String::new();
    println!("Digite um número: ");
    io::stdin().read_line(&mut dado).expect("Não foi digitado algo valido");
    let num:i32 = dado.trim().parse().expect("A entrada não foi um número valido");
    println!("O resultado de {}! é: {}",num, fatorial(num))
}
