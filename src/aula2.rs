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

pub fn main(){
    condicionais();
}