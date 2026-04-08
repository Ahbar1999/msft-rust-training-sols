
macro_rules! map {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut m = std::collections::HashMap::new();
            // for each pair 'key', 'value' matched before, expand the following statement 
            $( m.insert($key, $value); )*
            
            m
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let m = map! {
            "hello" => "world",
            "foo" => "bar",
        };

        assert_eq!(m.get("hello"), Some(&"world"));
        assert_eq!(m.get("foo"), Some(&"bar"));
        assert_eq!(m.get("bar"), None);
    }

    #[test]
    fn create_empty() {
        let mut m2 = map!();
        
        assert_eq!(m2.len(), 0);

        m2.insert("hello", "world");
        
        assert_eq!(m2.get("hello"), Some(&"world"));
    }
}
