// Texto e caracteres:
// String remete a possibilidade de concatenar/juntar caracteres
// formando uma palavra só como se fosse uma `corda`. Por isso o
// nome STRING;

use std::io;

fn main() {
    let l0 = 'M'; // tipo char
    let l1 = 'a';
    let l2 = 't';
    let l3 = 'e';
    let l4 = 'u';
    let l5 = 's';
    println!("{l0}{l1}{l2}{l3}{l4}{l5}");

    // Trabalhando com strings de tamanho fixo:
    let nome = ['M', 'a', 't', 'e', 'u', 's'];
    let nome_in_string: String = nome.iter().collect();
    println!("{:?}", nome_in_string);

    // &str = stringslice/string reference;
    // A variável nome será uma referência àquela porção de texto que é declarada
    // a ela. Por debaixo dos panos, essa variável é = [inicio_da_string, length].
    // Não se pode alterar essa referência;
    let nome: &str = "Mateus de Souza";
    println!("{nome}");

    // Trabalhando com strings dinâmicas:
    // Heap String
    let mut s = String::new();
    s.push('M');
    s.push_str("ateus");
    s.push_str(" de Souza");
    println!("{s}");

    let mut nome: String = "".to_string();
    nome.push_str("Mateus de Souza");
    println!("{nome}");

    let mut nome: String = String::from("Mateus");
    nome.push_str(" Albuquerque");
    println!("{nome}");

    let nome = ['M', 'a', 't', 'e', 'u', 's'];
    let s = String::from_iter(nome);
    println!("{s}");

    let mut nome_pai: String = "Fernando".into(); // precisa do tipo explicito
    nome_pai.push_str(" César Carneiro");
    println!("{nome_pai}\n");

    leitura_do_term();
}

fn leitura_do_term() {
    println!("{:-^100}", "leitura_do_term");
    let mut s = String::new();
    println!("Digite um texto: ");

    io::stdin()
        .read_line(&mut s)
        .expect("Error reading console");

    println!("Você digitou {s}");

    println!("Tamanho em bytes do texto digitado: {}", s.trim().len());

    println!(
        "Quantidade de letras digitadas: {}",
        s.trim().chars().count()
    );

    println!("Nome em maiusculo: {}", s.to_uppercase());

    println!("Mudanças no nome: {}", s.replace("a", "e"));

    println!("{:-^100}", "Calculadora");
    let mut s = String::new();

    let banner = "Digite uma sequência de numbers\nseparados por vírgula\nexemplo: 1,2,3,4";
    println!("{banner}");

    io::stdin()
        .read_line(&mut s)
        .expect("Error reading console");

    // Eliminando espaços:
    // Maneira 1:
    fn clean(c: &str) -> &str {
        c.trim()
    }

    let nums: Vec<&str> = s.split(",").map(clean).collect();
    println!("Você digitou {:?}", nums);

    // Maneira 2:
    let nums: Vec<&str> = s.split(",").map(|c| c.trim()).collect();
    println!("Você digitou {:?}", nums);

    println!("{}", "-".repeat(100));
}
