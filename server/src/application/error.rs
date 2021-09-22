pub enum ErrorType {
  Unavailable
}

pub struct Error {
  pub error_type: ErrorType,
  pub message: String,
}