use std::collections::HashMap;

use radyx::Radyx;

fn main() {
    divan::main();
}

#[divan::bench(args = [1000, 2500, 5000, 10000, 25000, 50000, 100000, 1000000])]
fn large_scale_insertions(u: u64) {
    let mut node: Radyx<'_, String> = Radyx::default();
    let mut value_map = HashMap::new();

    for i in 0..u {
        let k = format!("/key{}", i);
        let v = format!("Value{}", i);
        value_map.insert(k, v);
    }

    for (k, v) in value_map.iter() {
        node.insert(&k, v.to_string());
    }

    for i in 0..u {
        let key = format!("/key{}", i);
        assert_eq!(Some(&format!("Value{}", i)), node.get(&key));
    }
}
