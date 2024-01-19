
//Esse exemplo utiliza o mesmo codigo do exemplo de preencher_matriz 
/**Só que dessa vez será requisitado um número que cria uma matriz de tamanho n
 * que vai ter armazenada o nome e as notas dos alunos.
**/

use std::io::{self,stdin,stdout};

fn preencher_nomes(nomes: &mut Vec<String>){
    println!("Digite o nome: ");
    let mut nome: String = String::new();
    io::stdin().read_line(&mut nome).expect("Error ao digitar nome");
    if let Some('\n')=nome.chars().next_back() {
        nome.pop();
    }
    if let Some('\r')=nome.chars().next_back() {
        nome.pop();
    }
    nomes.push(nome);

}

//função que preenche a matriz de duas dimensões a partir de um numero digitado inicialmente 
fn preencher_notas(matrix: &mut Vec<Vec<f32>>, numero:i32, nomes_alunos: &mut Vec<String>, numero_unidade:i32) {
   
   
    for _ in 0..numero{
        //aqui cria um primeiro vetor para colocar dentro de uma posição do vetor de duas dimensões
        //Essa parte a baixo também serve para limpar o vetor que foi criado para que eu possa utilizar o mesmo na próxima interação
        let mut lista: Vec<f32> = Vec::new();
        //Essa função preenche a matriz com o nome de alunos para salvar
        preencher_nomes( nomes_alunos);
        println!("Digite as notas: ");
        for _ in 0..numero_unidade{
            //aqui é colocado os números no vetor que é limpo a cada interação do for mais externo
            let mut dado_01: String = String::new();
            io::stdin().read_line(&mut dado_01).expect("Não foi digitado algo valido");
            let vetor_numero:f32 = dado_01.trim().parse::<f32>().expect("Error você não digitou um número");
            lista.push(vetor_numero);
            //Aqui eu recomento um println!("{:?}", lista); para uma melhor compreenção do que está acontecendo durante as interações
        }
        //essa linha coloca o vetor com nome lista no vetor de duas dimensões que está alocando a matriz
        matrix.push(lista);
    }
}


fn main() {
    //aqui é criado o vetor da matriz
    let mut matrix: Vec<Vec<f32>> = Vec::new();
    let mut alunos_nome: Vec<String> = Vec::new();
    //aqui faz uma requisição da quantidade de alunos 
    let mut dado: String = String::new();
    println!("Digite a quantidade de alunos: ");
    io::stdin().read_line(&mut dado).expect("Não foi digitado algo valido");
    let numero:i32 = dado.trim().parse().expect("A entrada não foi um número valido");
    
     //aqui faz uma requisição da quantidade unidades com notas
    let mut unidade_notas: String = String::new();
    println!("Digite a quantidade de unidades: ");
    io::stdin().read_line(&mut unidade_notas).expect("Não foi digitado algo valido");
    let numero_unidade:i32 = unidade_notas.trim().parse().expect("A entrada não foi um número valido");

    
    //Esse trecho do código chama a função de criação e mostra ela
    
    preencher_notas(&mut matrix, numero, &mut alunos_nome, numero_unidade); 
    println!("Notas dos alunos: "); 
    for (i,nome) in alunos_nome.iter().enumerate() {
        println!("Nome do aluno: {nome}, Suas notas {:?}", matrix[i]);
    
    }
}
