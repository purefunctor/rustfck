mod rustfck;

fn main() {
    let result = rustfck::interpret("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
", "");
    if let Err(e) = result {
        println!("panic! {}", e);
    };
}
