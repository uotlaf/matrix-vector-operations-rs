use std::io::{stdin, stdout, Write};

macro_rules! read_line {
    ($i:expr) => {
        stdout().flush().unwrap();
        $i.clear();
        stdin().read_line(&mut $i).expect("Falha ao ler a string");
    };
}

fn multiplicar_matriz(matriza: &Vec<Vec<i32>>, matrizb: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // matriza.col == matrizb.lin
    assert_eq!(matriza[0].len(), matrizb.len());
    let mut matriz = vec![vec![0; matrizb[0].len()]; matriza.len()];

    for i in 0..matriza.len() {
        for j in 0..matrizb[0].len() {
            for k in 0..matriza[0].len() {
                matriz[i][j] += matriza[i][k] * matrizb[k][j];
            }
        }
    }
    matriz
}

fn transposta_matriz(matriz: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut transposta = vec![vec![0; matriz.len()]; matriz[0].len()];
    for i in 0..matriz.len() {
        for j in 0..matriz[0].len() {
            transposta[j][i] = matriz[i][j];
        }
    }
    transposta
}

fn imprimir_matriz(matriz: &Vec<Vec<i32>>) {
    for x in matriz.iter() {
        for y in x.iter() {
            print!("\t{} ", *y);
        }
        println!();
    }
}

fn main() {
    let mut matriza : Vec<Vec<i32>> = Vec::new();
    let mut matrizb : Vec<Vec<i32>> = Vec::new();
    loop {
        println!("\033[2J \033[H");
        println!(" - MENU - ");
        println!("1.Ler a Matriz A");
        println!("2.Ler a Matriz B");
        println!("3.Mostrar a Matriz A");
        println!("4.Mostrar a Matriz B");
        println!("5.Multiplicar a matriz A pela matriz B");
        println!("6.Calcular a transposta da matriz A");
        println!("7.Multiplicar a matriz A pela transposta da matriz B");
        println!("0.Sair");
        print!("Digite a opção que deseja: ");
        let mut resposta = String::new();
        read_line!(resposta);

        match resposta.trim().parse::<i32>() {
            Ok(0) => { break },
            Ok(1) => {
                'altura: loop {
                    print!("Digite a altura da matriz A: ");
                    let mut altura = String::new();
                    read_line!(altura);
                    match altura.trim().parse::<usize>() {
                        Ok(altura) => {
                            matriza = Vec::with_capacity(altura);
                            for y in 0..matriza.capacity() {
                                matriza.push(Vec::with_capacity(altura));
                                for x in 0..matriza[y].capacity() {
                                    'itens: loop {
                                        print!("Digite um número para o item na posição {x}:{y}: ");
                                        let mut item = String::new();
                                        read_line!(item);
                                        match item.trim().parse::<i32>() {
                                            Ok(item) => {
                                                matriza[y].push(item);
                                                break 'itens
                                            },
                                            Err(_) => {
                                                println!("Erro: Digite um valor inteiro!")
                                            }
                                        }
                                    }
                                }
                            }
                            break 'altura;
                        } ,
                        Err(_) => {
                            println!("Erro: Digite um valor inteiro!")
                        }
                    }
                }
            },
            Ok(2) => {
                'altura: loop {
                    print!("Digite a altura da matriz B: ");
                    let mut altura = String::new();
                    read_line!(altura);
                    match altura.trim().parse::<usize>() {
                        Ok(altura) => {
                            matrizb = Vec::with_capacity(altura);
                            for y in 0..matrizb.capacity() {
                                matrizb.push(Vec::with_capacity(altura));
                                for x in 0..matrizb[y].capacity() {
                                    'itens: loop {
                                        print!("Digite um número para o item na posição {x}:{y}: ");
                                        let mut item = String::new();
                                        read_line!(item);
                                        match item.trim().parse::<i32>() {
                                            Ok(item) => {
                                                matrizb[y].push(item);
                                                break 'itens
                                            },
                                            Err(_) => {
                                                println!("Erro: Digite um valor inteiro!")
                                            }
                                        }
                                    }
                                }
                            }
                            break 'altura;
                        } ,
                        Err(_) => {
                            println!("Erro: Digite um valor inteiro!")
                        }
                    }
                }
            },
            Ok(3) => {
                println!("MATRIZ A:");
                imprimir_matriz(&matriza);
            },
            Ok(4) => {
                println!("MATRIZ B:");
                imprimir_matriz(&matrizb);
            },
            Ok(5) => {
                if matriza.len() == 0 {
                    println!("Matriz A está vazia");
                    continue;
                }
                if matrizb.len() == 0 {
                    println!("Matriz B está vazia");
                    continue;
                }
                if matriza[0].len() != matrizb.len() {
                    println!("A largura da matriz A deve ser igual ao número de colunas da matriz B!");
                    continue;
                }
                imprimir_matriz(&multiplicar_matriz(&matriza, &matrizb));
            },
            Ok(6) => {
                if matriza.len() == 0 {
                    println!("Matriz A está vazia");
                    continue;
                }
                imprimir_matriz(&transposta_matriz(&matriza));
            },
            Ok(7) => {
                if matriza.len() == 0 {
                    println!("Matriz A está vazia");
                    continue;
                }
                if matrizb.len() == 0 {
                    println!("Matriz B está vazia");
                    continue;
                }
                if matriza[0].len() != matrizb.len() {
                    println!("A largura da matriz A deve ser igual ao número de colunas da matriz B!");
                    continue;
                }
                imprimir_matriz(&multiplicar_matriz(&matriza, &transposta_matriz(&matrizb)));
            },
            Err(_) => { println!("Digite um valor que seja um inteiro"); },
            Ok(_) => { println!("Digite um valor válido")}
        }
    }
}
