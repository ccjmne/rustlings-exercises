fn borrowing() {
    let s = "Hello";
    {
        let mut s = String::from(s);
        s += "World";
        println!("{}", s);
    }
    println!("{}", s);
    // --------------------------------------------------------------------------------------------
    let mut x = 5;
    let y = x;
    x += 2;
    println!("{}", x + y);
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");
    // --------------------------------------------------------------------------------------------
    let ss = s;
    println!("{s}, {ss}");
}

fn main() {
    borrowing()
}
