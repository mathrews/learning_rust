// Funções em rust são definidas pela keyword "fn". No nome da function
// sempre será utilizado o pafrão snake_case. Quando parametrizamos uma
// função, precisamos obrigatoriamente colocar o tipo pois o compilador
// não fará inferência. Os nomes dos parâmetros também seguirão o padrão
// snake_case. Em Rust, todos os argumentos deverão obrigatoriamente ser
// usados pois não há como atribuir um valor padrão se eles forem omitidos.
// Se quisermos mais de um argumento e atribuir valores default a eles,
// teremos que passar uma struct como parametro, ou também é possível
// construir macros que resolverm o problema. Não é possível nomear os
// parâmetros passados para as funções.
// O rust usa pattern match em tudo, devido a isso, os parâmetros são
// posicionalmente definidos.
//
// Expressions vs Statements: Statements são keywords que por si só não
// produzem valores de retorno, como por exemplo o "let". Expressions são
// todas as construções da linguagem que produzem valores de retorno, como
// por exemplo o numero 5, chamado de expressão numérica literal. Expressions
// também são operações aritméticas, como 5 * 5, e expressões de comparação
// como 10 > 4.

fn main() {
    println!("Hello, world!");
    say_hello_string(String::from("Mateus"), "red");
    say_hello_str("Calvo");

    // Este bloco de código é uma expressão que pode ser atribuída a uma variável.
    // Esta variável é do tipo "unit type" pois não retorna nada, mas se colocarmos
    // uma expressão final que retorne algo, e mudarmos o tipo da variárel, o bloco de código
    // inteiro se tornará uma expressão que irá retornar uma variável.
    let bloco: &str = {
        // for n in 0..80 {
        //     println!("{}", "-".repeat(80 - n));
        // }
        // for n in 0..80 {
        //     println!("{}", "-".repeat(0 + n));
        // }
        println!("{}", "-".repeat(80));
        let idade: u8 = 16;
        say_hello_string(String::from("Calvão"), "brilhante");
        println!("E tem {idade} anos.");
        "Meu nome é Mateus"
        // isso por si só é uma experssão literal de string
        // e não se deve colocar ponto e virgula após a seção
        // pois isso invalida o retorno e torna-a em uma espécie
        // de statement.
    };
    println!("{:?}", bloco);
    println!("{}", "-".repeat(80));

    let res: i32 = add_numbers(5, 4);
    println!("{res}");
    println!("{}", "-".repeat(80));

    // Closures / função anônima:
    let input: &str = "55 66 77 88 99 100 110";

    let result: Vec<i32> = input
        .split(' ')
        .map(|element| element.parse::<i32>().unwrap())
        .map(|element| element * 2)
        .collect();

    println!("{:?}", result);
    println!("{}", "-".repeat(80));
}

// fn convert_to_number(n: &str) -> i32 {
//     n.parse().unwrap()
// }

fn add_numbers(x: i32, y: i32) -> i32 {
    // Return é uma statement por si só, e nesse caso não é preciso
    // declarar esta statemente pois x + y já é uma expression que retorna
    // o que a função pede.

    // Early returns para guard clauses: Em early returns, precisamos definir
    // o statement return para definir onde é o early return e em que condição
    // está sendo retornado
    if x == 0 {
        return y;
    };
    if y == 0 {
        return x;
    };
    if x == 0 && y == 0 {
        return 0;
    }

    x + y
}

fn say_hello_string(name: String, color: &str) {
    println!("Hello {name}, your color {color}");
}

fn say_hello_str(name: &str) {
    println!("Hello {name}");
}
