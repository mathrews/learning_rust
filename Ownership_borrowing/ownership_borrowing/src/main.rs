// O rust possui o OBRM/RAII (Ownership Borrowing Resource Managment).
// O borrow_checker no rust é uma ferramenta que checa na hora da
// compilação se o código está fazendo um bom gerenciamento de memória.
// Se não estiver, ele barra a compilação. E o papel dele é justamente
// impedir que o código que está com mal gerenciamento de memória compile.
// O borrowchecker é integramente acoplado ao nível da função em si.
// Qualquer erro que esteja em uma função, o borrowchecker irá perceber.
//
// Tipos copy: tipos que podem ser copiados automaticamente e implicitamente,
// tipos que são alocados na stack memory pelo fato de ser uma tarefa rápida
// e barata de se fazer; são todos os valores primitivos escalares no Rust (i32, f64,
// bool, char, f64);
//
// Exemplo: Se houvesse um array que dentro houvesse apenas tipos copy,
// a operação de atribuí-lo a outra variável, fazendo assim um copy,
// funcionaria.
//
// Alguns dos problemas que o borrowchecker evita:
//
// Referências inválidas ou nulas: O borrow checker impede que referências
// se tornem inválidas, garantindo que todas as referências apontem para
// áreas de memória válidas.

// Dangling pointers: Ponteiros que apontam para áreas de memória
// que foram liberadas são uma fonte comum de erros em linguagens de baixo nível.
// O borrow checker garante que todas as referências permaneçam válidas
// durante todo o tempo de vida.
//
// Double free errors: Tentativas de liberar a mesma área de memória mais de
// uma vez podem levar a comportamentos indefinidos. O borrow checker garante
// que a liberação de memória aconteça apenas quando não há mais referências
// a ela.
//
// Race conditions: Concorrência de acesso à memória pode levar a resultados
// imprevisíveis ou corrupção de dados. O borrow checker ajuda a prevenir
// esses problemas, garantindo que apenas uma parte do código possa acessar
// um recurso de memória de cada vez.
//
// Memory leaks: Vazamentos de memória ocorrem quando a alocação de memória não
// é liberada quando não é mais necessária. O borrow checker ajuda a evitar esse
// problema, garantindo que a memória seja liberada quando todas as referências
// a ela saem de escopo.
//
// Regras de Ownership em Rust:
// 1 - Cada valor tem um dono (owner);
// 2 - Só pode ter UM único dono;
// 3 - Quando o dono sai de escopo, o valor é limpo;
// 4 - A posse (ownership) pode ser movida a outro dono;
//
// Regras de Borrowing em Rust:
// 1 - Podemos ter uma única referência caso ela seja mutável
// 2 - Podemos ter várias quando todas sã imutáveis
//
// OBS: AS REGRAS DE OWNERSHIP E BORROWING NÃO SE APLICAM AOS TIPOS COPY;

fn main() {
    println!("{:-^80}", "Copy");
    // Copy types:
    let mut a: i32 = 4;
    let b = a;
    // Aqui não é uma referênciação da variável a, e sim uma cópia do valor que está nela
    // que no caso é 1. E devido a isso, a variável b é independente da variável a. Essa
    // variável apenas irá pegar o primeiro valor assinalado a variável a; A na memória
    // e B na memória são DISTINTOS
    a = 5;
    println!("A: {a}, B: {b}");

    println!("{:-^80}", "Reference e Deference");
    // Reference e Deference:
    let a = 1;
    // Aqui não há uma cópia, e sim uma REFERÊNCIA àquela variavel.
    // Isso só acontece opcionalmente ao colocar o & antes da expressão.
    let b = &a;
    println!("A: {a}, B: {b}");
    // Se a função/macro não fizesse a deferência
    // automaticamente como a macro println!("") faz: (* antes da variável)
    println!("A: {}, B: {}", a, *b);

    println!("{:-^80}", "Ownership");
    // Owmership Exemplos:
    // Exemplo 1-

    // string heap
    // Nesse exemplo, lemos como se a string atribuida a
    // variável "a" fosse POSSE da mesma que é dona da
    // string; -> ownership
    let a = String::from("Mateus"); // -> No copy
    let b = a; // Após a transferencia, a let A invalida.
               // Na variaável b acontece uma Move semantic. Devido a variavel A
               // ser dona da string e por outros fatores, a prorpriedade da variável
               // A passa para a variável B e esta se torna responsável pela limpeza
               // da string atribuída a variável A. Mas se quisermos que o a continue
               // existindo, podemos emprestá-lo ao b utilizando o operador & (borrowing);
    println!("A: (invalidada), B: {b}");

    println!("{:-^80}", "Borrowing");
    // Borrowing Exemplos:
    // Exemplo 1 -
    let a = String::from("Mateus");
    let b = &a; // empréstimo de valor
    println!("A: {a}, B: {b}");

    println!("{:-^80}", "Copy em funções");
    // Copy em funções:
    let name = "Mateus"; // &str - static
    say_hello(name); // -> O valor da let name será copiado para dentro da fn
    say_goodbye(name);

    println!("{:-^80}", "Ownership em funcs");
    // Ownership em funcs:
    let name = String::from("Mateus");
    say_hello_ownership(name.clone()); // Aqui o valor foi CLONADO
    say_goodbye_ownership(name); // Aqui o valor foi movido mudando o owner para a let text

    println!("{:-^80}", "Borrowing em funcs");
    // Borrowing em funcs:
    let mut name = String::from("Teteus");
    say_hello_borrowing(&name);
    say_goodbye_borrowing(&name);
    to_uppercase(&mut name); // mut borrow
    add_prefix(&mut name);
    println!("{name}");
    println!("{:-^80}", "fim");
    // Nessas duas chamadas, o valor da let name foi EMPRESTADO;
}

fn say_hello(text: &str) {
    println!("Hello {text}");
}

fn say_goodbye(text: &str) {
    println!("Goodbye {text}");
}

// Ownership em Funcs:
fn say_hello_ownership(text: String) {
    println!("Hello, {text}");
}

fn say_goodbye_ownership(text: String) {
    println!("Goodbye, {text}");
}

fn say_hello_borrowing(text: &String) {
    println!("Hello {text}");
}

fn say_goodbye_borrowing(text: &String) {
    println!("Goodbye {text}");
}

fn to_uppercase(text: &mut String) {
    *text = text.to_uppercase(); // Precisamos derreferenciar com o *
                                 // se quisermos mudar o argumento mutável, porém isso depende mais
                                 // dos casos de uso.
}

fn add_prefix(text: &mut String) {
    *text = format!("FOO_{text}"); // Precisamos derreferenciar com o *
                                   // se quisermos mudar o argumento mutável, porém isso depende mais
                                   // dos casos de uso.
}
