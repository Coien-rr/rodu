#[derive(Debug)]
pub enum Opts {
    Help,
    Version,
    Detail,
    Add,
    Done,
    List,
    Invalid,
}

impl Opts {
    pub fn new(cmd: &String) -> Opts {
        match cmd.as_str() {
            "-h" | "--help" | "help" | "-?" => return Opts::Help,
            "-v" | "--version" => return Opts::Version,
            "--detail" => return Opts::Detail,
            "done" => return Opts::Done,
            "ls" => return Opts::List,
            "add" => return Opts::Add,
            _ => return Opts::Invalid,
        }
    }

    pub fn is_meta(&self) -> bool {
        match self {
            Self::Help => display_help(),
            Self::Version => display_version(),
            Self::Detail => display_detail(),
            _ => return false,
        }
        true
    }

    pub fn do_task(&self) {}
}

fn display_help() {
    println!("Usage:");
    println!("  todo [options]");

    println!("META OPTIONS");
    println!("  -?, -h, --help        show list of command-line options");
    println!("  -v, --version         show version of todo");
    println!("  --detail              show build detail of todo");

    println!("TODO OPTIONS");
    println!("  [taskName]            add Todo Task Directly");
    println!("  ls                    list all Todo Items");
    println!("  done [taskName]       done target task");
}

fn display_version() {
    println!("todo v{}", env!("CARGO_PKG_VERSION"));
    print!("BuildInfo: Commit {}", get_git_commit_hash());
    println!("Run todo --detail for more info");
}

fn display_detail() {
    println!("todo v{}", env!("CARGO_PKG_VERSION"));
    println!(
        "Description: {} ({})",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_NAME")
    );
    print!("BuildInfo: Commit {}", get_git_commit_hash());
    println!("AuthorInfo: {}", env!("CARGO_PKG_AUTHORS"));
    println!("RepositoryInfo: {}", env!("CARGO_PKG_REPOSITORY"));
}

fn get_git_commit_hash() -> String {
    use std::process::Command;
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    git_hash
}
