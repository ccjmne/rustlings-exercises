fn ownership() {
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
    // --------------------------------------------------------------------------------------------
    let mut s3 = String::from("Hello");
    println!("{s3}");
    s3 = asdf(s3);
    println!("{s3}");
    fn asdf(var: String) -> String {
        var
    }
}

fn borrowing() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of \"{s1}\" is {len}.");
    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    // --------------------------------------------------------------------------------------------
    let mut s = String::from("Hello");
    change(&mut s);
    fn change(s: &mut String) {
        s.push_str(", world");
    }
    println!("{s}");

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r2}");
    s.push_str("world");
    let r3 = &mut s;
    r3.push_str("world");
    println!("{r3}");
    // --------------------------------------------------------------------------------------------
    // fn dangle() -> &String {
    //     &String::from("Hello")
    // }
    fn no_dangle() -> String {
        String::from("Hello")
    }
}

fn main() {
    println!("OWNERSHIP ---------------------------------------");
    ownership();
    println!("BORROWING ---------------------------------------");
    borrowing();
}
