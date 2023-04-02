fn main() {
    println!("Cálculo de Médias. Entre com as notas de 4 provas.");
    let mut notas = [0.0; 4];
    let mut soma = 0.0;
    for i in 0..4 {
        println!("Entre com a nota da prova {}: ", i + 1);
        let mut nota = String::new();
        std::io::stdin().read_line(&mut nota).expect("Erro ao ler a nota");
        let nota: f32 = nota.trim().parse().expect("Erro ao converter a nota");
        notas[i] = nota;
        soma += nota;
    }
    let media = soma / 4.0;
    println!("Média: {}", media);
}
