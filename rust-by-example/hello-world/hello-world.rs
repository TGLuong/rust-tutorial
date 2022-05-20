/*
 * this is ennffdjdksf
 * fjdksf
 * fsdjh
 */


fn main() {
    println!("hello world");
    eprintln!("error");
    println!("{} days", 31);
    println!("{0}, this is {1}, {1}, this is {1}", "alice", "bob");
    println!("{subject} {verb} {object}",
            object = "the layzy dog",
            subject = "the quick brown fox",
            verb = "jump over");
    println!("{} of {:b}", 1, 2);
    println!("{number:a<width$}", number = 1, width = 10);
    println!("my name is {0}, {1} {0}", "james", "bob");

    #[allow(dead_code)]
    struct Structure(i32);

    //println!("This struct `{}` won't print...", Structure(3));
}
