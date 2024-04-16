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
// bool, char);

fn main() {
    let mut a: i32 = 1;
    let b = a;
    // Aqui não é uma referênciação da variável a, e sim uma cópia do valor que está nela
    // que no caso é 1. E devido a isso, a variável b é independente da variável a. Essa
    // variável apenas irá pegar o primeiro valor assinalado a variável a; A na memória
    // e B na memória são DISTINTOS
    a = 5;
    println!("{a}, {b}");
}
