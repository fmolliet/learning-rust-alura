// While
fn repeticoes() {
    let multiplicador:u8 = 5;
    
    let mut contador:u8 = 0;
    while contador < 10 {
        contador += 1;
        // Faz com que o Rust Pule o loop atual
        if contador == 5 {
            continue;
        }
        
        println!("{} x {} = {}", multiplicador, contador, multiplicador*contador);
    }
    
    contador = 0;
    // ex de while true
    loop {
        contador += 1;
        
        println!("{} x {} = {}", multiplicador, contador, multiplicador*contador);
        // Faz com que o rust termine a execução
        if contador == 10 {
            break;
        }
        
    }
    
    loop_for();
} 

// For each
fn loop_for(){
    let multiplicador:u8 = 5;
    // 1 .. 11 é igual gerar um range de numeros de 1 a 10 
    //  Intervalos por padrão não incluem o número final. Para isso faríamos 1..=10.
    // pode usar tambem 1..=10
    for i in 1..11 {
        println!("{} x {} = {}", multiplicador, i, multiplicador*i);
    }
}



pub fn main(){
    repeticoes();
}