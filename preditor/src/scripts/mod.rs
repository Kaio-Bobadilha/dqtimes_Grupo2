// Submodulos
// Importa os tipos de dados e erro que definimos em 'src/'

use crate::data::Dataset;
use crate::error::PredictionError;

//Declarar e exportar publicamente os submódulos
pub mod linear_regression;
// Definir um Trait (Interface)
pub trait Predictor {
    /// Treina o modelo com os dados fornecidos.
    /// Retorna Ok(()) em sucesso ou um PredictionError em falha.
    fn train(&mut self, data: &Dataset) -> Result<(), PredictionError>;

    /// Realiza uma previsão com base em novas entradas.
    /// Retorna um Vetor de predições (f64) ou um PredictionError.
    fn predict(&self, inputs: &Dataset) -> Result<Vec<f64>, PredictionError>;
}