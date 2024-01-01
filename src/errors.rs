use std::fmt;

#[derive(Debug, Clone)]
pub struct StrError{
    text:String
}

impl StrError {
    pub fn new(text:String)-> StrError{
        StrError{text}   
    }
}

impl fmt::Display for StrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.text)
    }
}