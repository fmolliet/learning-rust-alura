// Em todo lugar que tiver essa constante, ele fara a substituição pelo valor
// Constante global
// As constantes não possuem um endereço de memória.
const PI:f32 = 3.14;

// Static é a mesma coisa que uma constante porem ela pode ser mutavel porem é inseguro
// Toda static e const é obrigatório ter seu tipo
static mut VARIAVEL_GLOBAL:u8 = 1;

// O Rust tem escopo de bloco
fn escopo(){
    // println! é uma macro do rust, não é uma função
    println!("PI = {}", PI);

    // é um tipo complexo
    //let essa_string:&'static str = "Meu nome";


    // Você diz para o Rust que você vai lidar com essa static que pode ser alterada
    unsafe {
        println!("variavel_global = {}", VARIAVEL_GLOBAL);
    }

    // i8 - Inteiro de 8 bytes vai de -128 a 127
    // u8 - Inteiro de 8 bytes mas sem o sinal (somente positivo) logo vai de 0 até 255
    // Outros tipos primitivos: https://doc.rust-lang.org/std/index.html#primitives
    //let variavel:i32 = 300;
    // Você pode redeclarar uma variavel
    let variavel:i32 = 301;
    println!("variavel = {}, tamanho {} bytes", variavel, std::mem::size_of_val(&variavel));
    // Criando decimal
    let decimal:f32 = 2.5;
    println!("variavel = {}, tamanho {} bytes", decimal, std::mem::size_of_val(&decimal));
    // criando uma variavel booleana
    // Obs: toda variavel declarada de rust é imutável para virar mutavel é necessário colocar um mut
    //let mut booleana:bool = false;
    let booleana = true;
    println!("variavel = {}, tamanho {} bytes", booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'C';
    println!("variavel = {}, tamanho {} bytes", letra, std::mem::size_of_val(&letra));

}

// Retorno é definido com uma seta
fn soma(a:i32, b:i32 ) -> i32{
    // Pesquisar sobre operadores e operações matemáticas
    // tudo é uma expressão
    // mas quando tem um ; no final ele vai ignorar o retorno da expressao 
    // por exempl se a macro for retornar algo o ; ignora e descarta
    println!("{} + {} = {}", a, b , a+b);
    // Ele vai retornar a soma de a + b mas da para usar return tbm
    a + b
}


// Se não é especificado o retorno então ela não retorna nada
fn sombra(){
    let a = 123;
    // Escopo anonimo, sem um if ou while
    {
        let b = 456;
        // Shadowing (ele coloca uma sobra sobre a variavel a) é perigoso e seguro
        let a = 777;
        println!("dentro, b={}", b);
        println!("dentro, a={}", a);
    }
    //println!("Fora, b={}", b);
    println!("Fora, a={}", a);
}

fn condicionais(){
     
    let idade:u8 = 18;
    
    let eh_maior =  idade >= 18;
    let responsavel_autorizou = true;
    // tem regras de precedencia logo && executa primeiro que ou
    if eh_maior {
        println!("Pode entrar na balada");
    } else if  idade >16 && responsavel_autorizou {   
        println!("Pode entrar com assinatura do responsável");
    } else {
        println!("Não pode entrar na balada");
    }
    
    
    let condicao = if eh_maior { "maior" } else { "menor" };
    
    println!("É {} de idade", condicao);
    
    match_statement();
}

// Pattern Matching
fn match_statement() {
    let linguagem = "PHP";
    
    let proposito = match linguagem {
        "PHP"=> "Web",
        "Kotlin"=> "Android",
        "Python"=> "Data Science",
        _ => "Desconhecido"
    };
    
    println!("O proposito de {} eh {}", linguagem, proposito);
}

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

fn main(){
    escopo();
    sombra();
    
    println!("Soma = {}", soma(2, 4) );
    //println!("decimal = {}", decimal)
    condicionais();
    repeticoes();

}