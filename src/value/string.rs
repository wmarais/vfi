use super::Value;


pub trait StackValue {
    fn from_ne_bytes(bytes: &[u8]) -> Self;
    fn to_ne_bytes(&self) -> Vec<u8>;
}


impl StackValue for i32 {
    fn from_ne_bytes(bytes: &[u8]) -> Self {
        i32::from_ne_bytes(bytes.try_into().unwrap())
    }

    fn to_ne_bytes(&self) -> Vec<u8> {
        (*self).to_ne_bytes().to_vec()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        

    }
}

