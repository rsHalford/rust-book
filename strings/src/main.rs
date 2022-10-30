fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    s1.push('!');
    println!("s1 is {}", s1);

    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");

    let t = format!("{}-{}-{}", t1, t2, t3);
    println!("{}", t);

    for c in s1.chars() {
        println!("{}", c);
    }

    for b in t.bytes() {
        println!("{}", b);
    }
}
