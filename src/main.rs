mod rustfck;

fn main() {
    let argv: Vec<String> = std::env::args().into_iter().collect();

    let source = std::fs::read_to_string(&argv[1]);

    if let Ok(s) = source {
        let interpreter = rustfck::Interpreter::from_source(&s);
        if let Err(e) = interpreter.and_then(|mut i| i.run()) {
            println!("panic! {}", e);
        }
    } else if let Err(e) = source {
        println!("panic! {}", e);
    }
}
