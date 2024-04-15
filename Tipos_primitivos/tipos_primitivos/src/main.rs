// Rust é uma linguagem estaticamente tipada
// Há uma divisão entre duas categorias de tipos: escalares e compostos.

// tipos escalares (scalar types):
// são tipos onde vamos armazenar apenas um valor dentro de uma escala conhecida: inteiros,
// floating, boolean, char;
// Permitem a comparação direta entre valores;

// tipos compostos (compound types):
// Serve, para agregar multiplos valores.
// tipos: Tupla (tuple) ex: (5, true, 42.1); Matriz (array) ex: [1, 2, 3, 4];

fn main() {
    // faz inferência para i32 se não houver tipo explicito. Dependendo do numero
    // que nossas variáveis ocuparem, podemos usar tipos que ocupam menos espaço na memória.
    // Podemos especificar o tipo usando o underline e o tipo após o valor literal.

    // PRIMITIVOS ESCALARES:
    // inteiros
    let x: i32 = 5;
    let y: i32 = x * 200;
    println!("{x}, {y}");

    // O Rust nos permite inserir underlines nos literais grandes para
    // dividí-los e ser mais fácil de ler.
    let x2 = 5;
    let y2 = 199_400_560;
    println!("{x2}, {y2}");

    // O rust nos permite também armazenar valores de tipos em outras bases diferentes.
    let hexa = 0xDEDEDE;
    let octal = 0o16;
    let binar = 0b1111_0000;
    let byte = b'A';
    println!("{hexa}, {octal}, {binar}, {byte}");

    // floating
    let x3: f64 = 42.1; // aqui ele também já faz a inferência de tipo;
    println!("{x3}");

    // boolean
    let booleano: bool = true; // internamente, o true é 1, e o false é 0; Também faz a inferência
                               // de tipo;
    println!("{booleano}");

    // char
    // No tipo char, é possível armazenar qualquer caractere de até quatro bytes da tabela unicode,
    // e só pode usar o tipo com aspas simples. A variável char também faz inferência de tipo.
    let letra: char = 'M';
    println!("{letra}");

    // PRIMITIVOS COMPOSTOS:
    // tuple: tipo que possui um tamanho fixo de elementos dentro dela. É de tipagem heterogênea.
    // Não existe um nome para o tipo na sintaxe da linguagem. Apesar de ser tamanho fixo, seus
    // index podem ser variáveis;
    let mut numbers: (i32, i32, f64) = (1, 2, 3.5);
    println!("{:#?}", numbers); // :? significa modo debugging
    println!("{:?}", numbers.0);
    println!("{:?}", numbers.1);
    println!("{:?}", numbers.2);

    numbers.0 = 200;
    let (a, b, c) = numbers; //desestruturação da tupla
    println!("{a}, {b}, {c}");

    numbers = (50, 100, 300.0);
    println!("{:?}", numbers);

    // array: possui tamanho variável e pode possuir apenas um tipo específico para todos os
    // elementos. Faz inferência de tipo. Podemos especificar o tipo dos elementos dentro do array
    // e quantos espaços ele irá ter. O rust sabe a quantidade de elementos que possui dentro de um
    // array. Se tentar acessar elementos que não estão nos limites do array, o rust em tempo de
    // compilação dará um erro "out of bound".;
    let mut numbers: [i32; 4] = [1, 2, 3, 4];
    println!("{:#?}", numbers); // variable shadowing
    println!("{:?}", numbers[0]);
    numbers[1] = 50;
    println!("{:?}", numbers);

    // slice
    println!(
        "{:?}, {:?}, {:?}, {:?}",
        &numbers[0..3],
        &numbers[0..=3],
        &numbers[0..4],
        &numbers[..=3]
    );
}
