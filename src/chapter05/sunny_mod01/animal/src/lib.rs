pub mod animal;
#[cfg(test)]
mod tests {
    use crate::animal::*;
    #[test]

    fn use_dog(){
        assert_eq!(true,dog::is_dog());
    } 
}
