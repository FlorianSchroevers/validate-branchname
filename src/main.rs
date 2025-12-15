
use clap::{Arg, Command};
use regex::Regex;
use git2::{Repository, Error};

fn current_branch_name(repo_path: &str) -> Result<String, Error> {
    // Walk up from repo_path to discover the .git directory
    let repo = Repository::discover(repo_path)?;
    let head = repo.head()?; // could be detached

    if head.is_branch() {
        let shorthand = head
            .shorthand()
            .unwrap_or("unknown");
        Ok(shorthand.to_string())
    } else {
        // Detached HEAD: return short commit id (or your policy)
        let oid = head
            .target()
            .ok_or_else(|| Error::from_str("No target OID"))?;
        Ok(format!("DETACHED@{}", &oid.to_string()[..7]))
    }
}

fn validate_branchname(pattern: &str) -> Result<bool, Error> {
    // Parse the regex pattern (return a git2::Error for uniformity)
    let re = Regex::new(pattern)
        .map_err(|e| Error::from_str(&format!("Invalid regex pattern: {e}")))?;

    match current_branch_name(".") {
        Ok(branchname) => {
            // `captures` returns Some(_) if it matches; None otherwise.
            Ok(re.captures(&branchname).is_some())
        }
        Err(err) => Err(Error::from_str(&format!("Error getting branch name: {err}"))),
    }
}

fn main() {
    let matches = Command::new("validate_branchname")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Match current branch name with given regex pattern")
        .override_usage("validate_branchname --pattern <regex_pattern>")
        .arg(
            Arg::new("pattern")
                .long("pattern")
                .short('p')
                .help("Regex pattern to match against the current branch name")
                .required(true)
                .value_name("REGEX")
                .num_args(1)
        )
        .get_matches();

    let pattern = matches
        .get_one::<String>("pattern")
        .expect("pattern is required");

    let exit_code = match validate_branchname(pattern) {
        Ok(true) => 0,   // match
        Ok(false) => 1,  // no match
        Err(err) => {
            eprintln!("{err}");
            1
        }
    };

    std::process::exit(exit_code);
}
