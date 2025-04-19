#![allow(dead_code)]
use rand::Rng;
use std::collections::HashMap;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn random_string(count: u128) -> String {
    let mut rng = rand::rng();

    return (0..count)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
}

pub fn flatten_with_prefix<T: Copy + Clone>(
    prefix: String,
    m: HashMap<String, T>,
) -> HashMap<String, T> {
    let mut result = HashMap::<String, T>::new();
    m.keys().for_each(|each| {
        if each.starts_with(prefix.as_str()) {
            match m.get(each) {
                Some(data) => {
                    result.insert(
                        each.strip_prefix(prefix.as_str())
                            .unwrap_or(&each)
                            .to_string(),
                        *data,
                    );
                }
                None => (),
            }
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
