use std::io;

fn main() {

    println!("Informe o número A: ");

    let mut numero_a: String = String::new();

    io::stdin()
        .read_line(&mut numero_a)
        .expect("Falha ao ler o valor");

    let mut maior_numero: i32 = converter_string_para_i32(numero_a);

    println!("Informe o número B: ");

    let mut numero_b = String::new();

    io::stdin()
        .read_line(&mut numero_b)
        .expect("Falha ao ler o valor");

    let mut menor_numero: i32 = converter_string_para_i32(numero_b);

    if maior_numero < menor_numero {
        maior_numero = menor_numero;
    }

    let mut numero_c = String::new();

    println!("Informe o número C: ");

    io::stdin()
        .read_line(&mut numero_c)
        .expect("Falha ao ler o valor");

    menor_numero = converter_string_para_i32(numero_c);

    if maior_numero < menor_numero {
        maior_numero = menor_numero;
    }

    println!("O maior número lido foi: {}", maior_numero);
}

fn converter_string_para_i32(uma_string: String) -> i32 {
    uma_string
        .trim()
        .parse()
        .expect("Falha ao converter o valor")
}