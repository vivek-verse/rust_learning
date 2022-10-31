#[macro_export]
macro_rules! vivek {
    ($($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! map {
    // $ [identifier] : [fragment-specifier]
    ($key:ty, $val:ty) => {{
        let map: HashMap<$key, $val> = HashMap::new();
        map
    }};

    ($($key:expr => $val:expr), *) => {{
        let mut map = HashMap::new();
        $(map.insert($key, $val);)*
        map
    }};
}
