fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let mut iter = vec.iter();

    if let Some(val) = iter.next() {
        println!("{:?}", val);
    }
    if let Some(val) = iter.next() {
        println!("{:?}", val);
    }
    if let Some(val) = iter.next() {
        println!("{:?}", val); //this will not be executed
    } else {
        println!("Iterator exhausted");
    }
    //Alternative solution using collect
    let values: Vec<_> = vec.iter().collect();
    println!("{:?}", values);
} 