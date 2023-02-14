mod aula1;
mod aula2;
mod aula3;
mod aula4;
mod services;
use services::{service};

fn main(){
    aula1::main();
    aula2::main();
    aula3::main();
    aula4::main();
    service::jooj();
}