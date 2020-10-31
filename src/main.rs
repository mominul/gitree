use run_script::{run, ScriptOptions};

const SCRIPT: &str = include_str!("git-subtree.sh");

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (code, mut output, error) = run(SCRIPT, &args, &ScriptOptions::new()).unwrap();

    // Replace 'git subtree' with `gitree` in the output.
    output = output.replace("git subtree", "gitree");

    print!("{}", output);
    eprint!("{}", error);
    std::process::exit(code);
}
