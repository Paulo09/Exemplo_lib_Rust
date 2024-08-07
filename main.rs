use estatisticas_descritivas_structs::EstatisticasDescritivas;

fn main() {
    let numeros = vec![1, 1, 2, 3, 4, 4, 4, 5, 6, 0];
    let estatisticas = EstatisticasDescritivas::new(numeros);
    estatisticas.calcular_estatisticas();
}
