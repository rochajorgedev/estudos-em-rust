use std::io;


//função que preenche a matriz de duas dimensões a partir de um numero digitado inicialmente 
fn preencher(matrix: &mut Vec<Vec<i32>>, numero:i32) {
    let mut lista: Vec<i32> = Vec::new();
    //aqui cria um primeiro vetor para colocar dentro de uma posição do vetor de duas dimensões
    for _ in 0..numero{
        //Essa parte a baixo serve apenas para limpar o vetor que foi criado para que eu possa utilizar o mesmo na próxima interação
        lista = Vec::new();

        for _ in 0..numero{
            //aqui é colocado os números no vetor que é limpo a cada interação do for mais externo
            let mut dado_01: String = String::new();
            io::stdin().read_line(&mut dado_01).expect("Não foi digitado algo valido");
            let vetor_numero:i32 = dado_01.trim().parse().expect("Error você não digitou um número");
            lista.push(vetor_numero);
            //Aqui eu recomento um println!("{:?}", lista); para uma melhor compreenção do que está acontecendo durante as interações
        }
        //essa linha coloca o vetor com nome lista no vetor de duas dimensões que está alocando a matriz
        matrix.push(lista);
    }
}


fn main() {
    //aqui é criado o vetor da matriz
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    //aqui faz uma requisição da quantidade da matriz 
    let mut dado: String = String::new();
    println!("Digite um número: ");
    io::stdin().read_line(&mut dado).expect("Não foi digitado algo valido");
    let numero:i32 = dado.trim().parse().expect("A entrada não foi um número valido");
    
    //Esse trecho do código chama a função de criação e mostra ela
    println!("Digite os números internos da matriz: ");
    preencher(&mut matrix, numero); 
    println!("Resultado da matriz: ");
    println!("{:?}", matrix);
}
