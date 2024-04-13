// STATIC: Tamanho fixo, conteudos armazenados: binário do programa,
// variáveis static e strings literais, sua lifetime é o programa inteiro,
// seu cleanup acontece quando o programa acaba.
static _dia_do_meu_niver: &str = "09/11/2007"; // exemplo de variável static

// STACK: Tamanho dinâmico e valores fixos, conteudos armazenados: 
// function arguments,
// variáveis locais (variáveis definidas dentro do escopo da função),
// thread executada no programa e stackframe das funções. Stack overflow 
// é o nome do erro que acontece quando a stack estoura. Seu lifetime é 
// até a função acabar e o cleanup acontece quando a função retorna. Porém,
// suas variáveis, threads, dentre outras coisas, são de tamanho FIXO. Se 
// a intenção for armazenar variáveis de tamanhos dinâmicos, é necessário usar
// a memória HEAP.

// HEAP: Usada quando precisamos armazenar valores de tamanhos dinâmicos. 
// Conteúdos armazenados na heap: Valores que vão além das funções,
// Valores compartilhados entre threads, valores grandes, valores de 
// tamanhos variados. Seu tamanho é dinamico indo até o limite do computador.
// lifetime definido pelo programador ou pela linguagem. Cleanup feito de até 
// três formas: manualmente, ou Garbage Collector ou RAII;

fn main() {
    // variáveis armazenadas na memória stack
    let _x = 5;
    let _y = 6;
    let _numbers: [i32;3] = [1, 2, 3];

    // let _users = get_users(); //função fictícia. Essa variável seria armazenada na heap
    // pois seu tamanho é dinâmico.
} // stackframe da função main
