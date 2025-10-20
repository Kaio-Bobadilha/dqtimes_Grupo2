// Definição de erros

use thiserror::Error;

// Definimos nosso tipo de erro público
#[derive(Error, Debug)]
pub enum PredictionError {
    #[error("Dimensões de entrada incompatíveis: esperado {expected}, recebido {received}")]
    DimensionMismatch {
        expected: usize,
        received: usize,
    },

    #[error("O modelo ainda não foi treinado.")]
    ModelNotTrained,

    #[error("Formato de dados inválido: {0}")]
    DataFormatError(String),

    // Podemos "embrulhar" outros erros, como erros de I/O
    #[error("Erro de I/O: {0}")]
    IoError(#[from] std::io::Error),
}