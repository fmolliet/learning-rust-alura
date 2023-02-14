
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


pub fn main(){
    escopo();
    sombra();
    println!("Soma = {}", soma(2, 4) );
}