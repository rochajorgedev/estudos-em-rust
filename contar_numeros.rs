use std::io;

fn quantidade_numeros(numero:i32) ->i32{
    let mut resultado:i32 = 0;
    let mut digito:i32 = numero;
    while digito != 0 {
        resultado += digito%10;
        digito = digito/10;
    }


    return resultado;
}


//Esse codigo soma os numeros dentro de um numero digitado
// por exemplo 12345 o resultado é 15 

fn main() {
    let mut dado = String::new();
    println!("Digite um número: ");
    io::stdin().read_line(&mut dado).expect("Não foi digitado algo valido");
    let numero:i32 = dado.trim().parse().expect("A entrada não foi um número valido");


    println!("A soma dos numeros {} é: {}",numero, quantidade_numeros(numero))
}
