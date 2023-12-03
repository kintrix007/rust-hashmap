mod hashmap;

fn main() {
    let mut map = hashmap::HashMap::new(3);
    map.set(String::from("Foo"), 1);
    map.set(String::from("Bar"), 2);
    map.set(String::from("Baz"), 3);
    println!("{:?}", map);

    println!("{:?}", map.get(&String::from("Foo")));
    println!("{:?}", map.get(&String::from("foo")));
    println!("{:?}", map.get(&String::from("Bar")));
}
