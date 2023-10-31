fn num_primo(num:i32)-> bool{
    if num == 1 || num == 0 {
        return false; 
    }
    let mut aux:i32 = 0;
    for i in 1..num{
        if num%i == 0{
            aux+=1;
        }
    }
   
    if aux >= 2 {
        return false;
    }
    return true;
    

}



fn main() {

    for i in 0..50{
        if num_primo(i){
            println!("{} é um numero primo.",i);
        }else{
            println!("{} não é um numero primo.",i);
        }
    }



}
