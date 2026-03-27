use compiler::lexer::lexer;

fn main() {
    let res: Vec<String>;

    res = lexer(String::from("addi r2, r2, r7 # WAZAAA"));

    for i in 0..res.len() {
        println!("{}", res[i])
    }
}
