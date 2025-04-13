use std::collections::HashMap;

pub fn flatten_with_prefix<T>(prefix: String, m: HashMap<String, T>) -> HashMap<String, T> {
    let result = HashMap::<String, T>::new();
    m.keys().for_each(|each| {
        if each.starts_with(prefix) {
            result.insert(each.strip_prefix(prefix), m.get_mut(each)?)
        }
    });
    result
}

#[macro_export]
macro_rules! value_ref_hashmap_extractor {
    ($this: ident, $data: ident, $($name: ident),+) => {
        $(
            let tmp: Result<String> = value_ref_to_type(m.get(stringify!($name)).unwrap()).map_err(Error::from);
            if let Err(e) = id {
                return Err(e);
            }
            $this.$name = tmp.unwrap().clone();
        )+
    };
}
