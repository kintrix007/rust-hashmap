mod hashmap;

fn main() {
    let mut map = hashmap::HashMap::new(3);
    map.set(&String::from("foo"), 1);
    map.set(&String::from("bar"), 2);
    map.set(&String::from("baz"), 3);
    map.set(&String::from("baz"), 4);
    map.set(&String::from("Hello!"), 42);
    println!("{:?}", map);

    println!("{:?}", map.get(&String::from("Foo")));
    println!("{:?}", map.get(&String::from("foo")));
    println!("{:?}", map.get(&String::from("Bar")));

    for (k, v) in map.iter() {
        println!("{}: {}", k, v);
    }

    map.remove(&String::from("foo"));
    println!("{:?}", map);
    map.remove(&String::from("bar"));
    println!("{:?}", map);
    map.remove(&String::from("baz"));
    println!("{:?}", map);
}
