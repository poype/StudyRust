use thiserror::Error;

fn main() {
    let n = -166;

    let result = test_err(n);
    if result.is_ok() {
        println!("{}", result.unwrap());
    } else {
        println!("{}", result.err().unwrap());
    }
}


fn test_err(n: i32) -> Result<String, TestError> {
    if n > 100  {
        Err(TestError::TwoError("number is larger than 100".to_string()))
    } else if n > 0 {
        Ok("number is ok".to_string())
    } else {
        Err(TestError::OneError("number is less than zero".to_string()))
    }
}

#[derive(Debug, Error)]
pub enum TestError {
    #[error("one error: {0}")]
    OneError(String),

    #[error("two error: {0}")]
    TwoError(String),
}
