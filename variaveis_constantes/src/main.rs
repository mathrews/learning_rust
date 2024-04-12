// Rust não possui garbage colector, porém ele possui um mecanismo chamado RAII.
// "Resource aquisition is initialization"
// Esse é o significado da sigla, e basicamente significa que uma variável
// ou uma função precisam ter seus escopos bem definidos para saberem onde
// iniciam e onde termina sua utilização.

// O compilador rust possui um mecanismo de inferência de tipos que advinha
// pelo texto qual o tipo daquela variável mesmo o rust sendo uma linguagem de
// tipagem estática.

// A constante pode ser definida em qualquer escopo do programa incluindo 
// o escopo principal.

use core::str;

const SECONDS_IN_MINUTE: u32 = 60; // u32 é unsigned, que é basicamente igual
// a módulo na matemática.
const MINUTES_IN_HOUR: u32 = 60;
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

fn main() {
    let mut total: i32 = 30;
    println!("Trabalhou: {total}. (escopo externo)");
    total = 44;
    {
        let total = total + 69;
        println!("Trabalhou: {total} (escopo interno)");
    } // variable shadowing dentro de escopo interno 
    println!("Trabalhou: {total}. (escopo externo após mudança)");

    let total: &str = "quarenta horas"; // variable shadowing
    println!("Trabalhou: {total}. (escopo externo variable shadowing)");

    fn inner() {
        let total = 20;
        println!("Não trabalhou {total} semanais");
    }
    inner();

    fn calc_hour_to_min() {
        let all_hours = 44;
        println!("Trabalhou {} minutos.", all_hours * MINUTES_IN_HOUR);
        fn min_to_secs(total: u32){
            println!("Trabalhou {} segundos.", total * SECONDS_IN_HOUR);
        }
        min_to_secs(all_hours);
    }
    calc_hour_to_min();
}
