fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("v value : {:?}",v);

    let mut v2 = vec![1, 2, 3, 4, 5];

    let  first: &i32 = &v2[0];

    v2.push(6);

    println!("The first element is: {first}");
}
