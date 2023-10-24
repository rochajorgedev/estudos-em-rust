use std::io;

fn requisicao_numero() -> i32{
    let mut dado = String::new();
    println!("Digite o numero: ");
    io::stdin().read_line(&mut dado).expect("A entrada não foi valida");
    let num:i32 = dado.trim().parse().expect("A entrada não foi um número valido");
    return num;
}
//multiplicação por recursividade
fn multiplicacao(num_1:i32, num_2:i32) -> i32{
    if num_2 == 0{
        return 0;
    }
    return num_1 + multiplicacao(num_1,num_2-1);
}
//soma comum
fn soma(num_1:i32, num_2:i32) -> i32{
    return num_1+num_2;
}
//subtração comum

fn subtracao(num_1:i32, num_2:i32) -> i32{
    return num_1 - num_2;
}
//divisão comum
fn divisao(num_1:i32, num_2:i32) -> i32{
    if num_2 <= 0 {
        return 0;
    }
    return num_1/num_2;
}
//calculo de potência por recursividade
fn pot(base:i32,expo:i32)-> i32{
    if expo == 0 {
        return  1;
    }
    return base*pot(base, expo-1);
}


fn main() -> io::Result<()> {
    
    loop {
    println!("Digite uma opção:");
    println!("1) Potência\n2) Multiplicação\n3) Subatração\n4) Divisão\n5) Soma\nDigite 6 ou mais para sair");
    
    let mut dado= String::new();
    io::stdin()
        .read_line(&mut dado)?;
    let opcao:i32 = dado.trim().parse().expect("A entrada não foi um número valido");
    match  opcao{
        1 => println!("O resultado é: {}",pot(requisicao_numero(),requisicao_numero())),
        2 => println!("O resultado é: {}",multiplicacao(requisicao_numero(),requisicao_numero())),
        3 => println!("O resultado é: {}", subtracao(requisicao_numero(), requisicao_numero())),
        4 =>{
            let resultado = divisao(requisicao_numero(), requisicao_numero());
            if resultado != 0 {
                println!("O resultado é: {}",resultado);
                
            }else {
                println!("Não é possivel fazer essa divisão");
            }
        } ,
        5 => println!("O resultado é: {}", soma(requisicao_numero(), requisicao_numero())),
        _=> break,
    }
        
    }

    return Ok(());
}
