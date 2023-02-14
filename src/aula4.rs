fn ownership() {
    // Por que alocamos a string na Heap? 
    // Pois ela poderia causar um stack overflow
    // Strings dinamicas são alocadas na Heap
    // Para ter uma referencia mutavel é necessário definir como mutavel
    let mut string = String::from("fabio");
    // & empresta a string (Borrowing)
    // necessário passar como &mut para ser mutavel e alterado no metodo
    rouba(&mut string);
    println!("{}", string);
    
    patter_matching();
    erros();
}

// Vira o dono do valor na memória no heap, ownership
// ele move os valores
// Se adicionamos o &, ele irá emprestar a string
// Adicionamos o mut para se tornar mutavel e alterar o valor
fn rouba( string: &mut String){
    string.push_str(" jooj");
    print!("{}", string);
}

fn patter_matching(){
    for x in 1..=20 {
        // Aplicamos o Pattern matcher dentro do println!
        println!("{}: {}",x , match x {
            1 => "Pouco",
            // É possivel ter mais de um valor com o pipe "|" 
            2 | 3 => "um pouquinho",
            // É possivel utilizar os Ranges tambem
            4..=10 => "um bocado",
            // é possivel colocar um if
            _ if x % 2 == 0 => "Uma boa quatidade",
            // usamos _ para definir como coringa, ou qualquer coisa
            _ => "Muito"
        } )
    }
}

fn erros(){
    // Fazemos match com base no retorno do metodo
    match result() {
        Ok(s) => println!("String de sucesso = {}", s),
        Err(numero) => println!("Codigo de erro = {}", numero)
    }
     // Panic é uma macro do rust que emite um erro que não é recuperável
    // panic!("Erro proposital");
    // Força um erro
    // let v = vec![1,2,3];
    // v[4];
}

// Result, definir que pode ter mais de um resultado
// Damos preferencia a result para que seja tratado por quem for chamar
fn result( ) -> Result<String, u8> {
    // Retornamos oka que deu tudo certo
    //Ok(String::from("Tudo deu certo"))
    // Ou o numero de um erro
    Err(1)
}

fn main(){
    ownership();
}