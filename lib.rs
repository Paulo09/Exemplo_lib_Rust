use std::collections::HashMap;

struct EstatisticasDescritivas {
    numeros: Vec<i32>,
}

impl EstatisticasDescritivas {
    fn new(numeros: Vec<i32>) -> Self {
        EstatisticasDescritivas { numeros }
    }

    fn mediana(&self) -> f64 {
        // Implementação da mediana (já fornecida em respostas anteriores)
        // ...
    }

    fn media(&self) -> f64 {
        // Implementação da média (já fornecida em respostas anteriores)
        // ...
    }

    fn moda(&self) -> Option<i32> {
        // Implementação da moda (já fornecida em respostas anteriores)
        // ...
    }

    fn calcular_estatisticas(&self) {
        println!("Mediana: {:.2}", self.mediana());
        println!("Média: {:.2}", self.media());
        match self.moda() {
            Some(moda) => println!("Moda: {}", moda),
            None => println!("Existem múltiplas modas."),
        }
    }
}
