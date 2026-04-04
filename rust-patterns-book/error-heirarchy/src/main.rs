use thiserror::Error;
use serde::{Serialize, Deserialize};

#[derive(Error, Debug)]
pub enum FileProcError {    // represents types of errors that can occur during our file processing
    #[error("JSON error: {0}")] 
    Json(#[from] serde_json::Error),
    
    #[error("IO error: {0}")] 
    IO(#[from] std::io::Error),

    #[error("CSV error: {0}")] 
    Csv(#[from] csv::Error),
    
    #[error("validation failed for file: {0}")]
    Validation(String), // handle this manually from Result<T, E> by using downcast_ref() 
}

fn main() {
    // println!("Hello, world!");
}

#[derive(Serialize, Deserialize)]
struct dummy {}

#[cfg(test)]
pub mod tests {
    use super::*;

    // this function necessarily return FileProcError::Io
    fn test_io_error() -> Result<(), FileProcError> {
        let content = std::fs::read_to_string("nonexistentfile.txt")?;

        Ok(())
    }
    
    // this function necessarily return FileProcError::Json
    fn test_json_error() -> Result<(), FileProcError> {
        let body = "wrong format";
        
        // this will fail!
        let content: dummy = serde_json::from_str(&body)?;

        Ok(())
    }

    fn test_csv_error() -> Result<(), FileProcError> {
        let bad_csv_data = "Name,Email,Phone\n
                        John Doe,john@example.com,555-0199\n
                        Jane Smith,jane@example.com\n";

        let mut reader = csv::Reader::from_reader(bad_csv_data.as_bytes());
        
        for result in reader.records() {
            // this should get intercepted and return FileProcResult::Csv
            result?;
        } 

        Ok(())
    }

    fn test_validation_error() -> Result<(), FileProcError> {
        Err(FileProcError::Validation("valid failed because of xyz".to_owned())) 
    }

    #[test]
    fn basics() {
        match test_json_error() {
            Err(FileProcError::Json(msg)) => {
                println!("{:?}", msg); 
            },
            _ => {
                assert!(false);
            }
        };
    
        match test_csv_error() {
            Err(FileProcError::Csv(msg)) => {
                println!("{:?}", msg); 
            },
            _ => {
                assert!(false);
            }
        };

        match test_io_error() {
            Err(FileProcError::IO(msg)) => {
                println!("{:?}", msg);
            },
            _ => {
                assert!(false);
            }
        };

        match test_validation_error() {
            Err(FileProcError::Validation(msg)) => {
                println!("{:?}", msg);
            },
            _ => {
                assert!(false);
            }
        };
    }
}
