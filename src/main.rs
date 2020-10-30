use run_script::{run, ScriptOptions};

const SCRIPT: &str = include_str!("git-subtree.sh");

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (code, output, error) = run(SCRIPT, &args, &ScriptOptions::new()).unwrap();
    println!("{}", output);
    eprintln!("{}", error);
    std::process::exit(code);
}
