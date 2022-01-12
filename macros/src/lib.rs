
#[macro_export]
macro_rules! hashmap {
    (,) => { dont compile this };
    
    ($($k:expr => $v:expr),* $(,)?) => {
        {
            use ::std::collections::HashMap;
            let mut hm = HashMap::new();
            $(
                hm.insert($k, $v);
            )*
            hm
        }
    };
}
