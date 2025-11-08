#[cfg(test)]
mod tests {
    
    
    use monnify_rs::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn sub_works(){
        let result  = sub(3,1);
        assert_eq!(result,2);
    }
}
