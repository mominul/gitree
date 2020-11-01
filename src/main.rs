use run_script::{run, ScriptOptions};

const SCRIPT: &str = include_str!("git-subtree.sh");

fn main() {
    let mut args = std::env::args();

    // The script calls itself recursively, so give the path of this executable via environment variable.
    let exe_path = args.next().unwrap();
    std::env::set_var("GITREE", exe_path);

    let args = args.collect();
    let (code, mut output, mut error) = run(SCRIPT, &args, &ScriptOptions::new()).unwrap();

    // Replace 'git subtree' with `gitree` in the output.
    output = output.replace("git subtree", "gitree");
    error = error.replace("git subtree", "gitree");

    print!("{}", output);
    eprint!("{}", error);
    std::process::exit(code);
}
