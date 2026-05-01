use clap::{ArgAction, Parser};

const GIT_SOURCES: &[(&str, &str)] = &[
    ("github", "https://github.com/"),
    ("gitlab", "https://gitlab.com/"),
    ("bitbucket", "https://bitbucket.org/"),
    ("codeberg", "https://codeberg.org/"),
];

fn is_owner_repo_shorthand(input: &str) -> bool {
    let value = input.trim();
    if value.is_empty() || value.contains("://") || value.starts_with("git@") {
        return false;
    }

    let mut parts = value.split('/');
    match (parts.next(), parts.next(), parts.next()) {
        (Some(owner), Some(repo), None) => !owner.is_empty() && !repo.is_empty(),
        _ => false,
    }
}

pub fn parse_owner_repo(input: &str) -> Result<String, String> {
    let value = input.trim();
    if is_owner_repo_shorthand(value) {
        Ok(value.to_string())
    } else {
        Err(format!(
            "invalid repository '{}': expected OWNER/REPO",
            input
        ))
    }
}

fn source_base_url(source: &str) -> Option<&'static str> {
    let source = source.trim().to_lowercase();
    GIT_SOURCES
        .iter()
        .find(|(name, _)| *name == source)
        .map(|(_, url)| *url)
}

// Create a git URL from a source and repository name
pub fn build_git_url(source: &str, repo: &str) -> String {
    let repo = repo.trim();
    if let Some(base) = source_base_url(source) {
        return format!("{}{}.git", base, repo);
    }
    repo.to_string()
}

#[derive(Parser)]
#[command(author = "Alberson da Silva Miranda", version, about)]

pub struct Cli {
    #[arg(required = false)]
    pub packages: Vec<String>,

    #[arg(short = 'c', long)]
    pub country: Option<String>,

    #[arg(short, long)]
    pub library: Option<String>,

    #[arg(
        long,
        action = ArgAction::Append,
        value_name = "OWNER/REPO",
        value_parser = parse_owner_repo
    )]
    pub github: Vec<String>,

    #[arg(
        long,
        action = ArgAction::Append,
        value_name = "OWNER/REPO",
        value_parser = parse_owner_repo
    )]
    pub gitlab: Vec<String>,

    #[arg(
        long,
        action = ArgAction::Append,
        value_name = "OWNER/REPO",
        value_parser = parse_owner_repo
    )]
    pub bitbucket: Vec<String>,

    #[arg(
        long,
        action = ArgAction::Append,
        value_name = "OWNER/REPO",
        value_parser = parse_owner_repo
    )]
    pub codeberg: Vec<String>,

    #[arg(
      long,
      action = ArgAction::Append,
      help = "Additional custom repository URL (takes precedence over country selection)"
    )]
    pub url: Option<String>,

    #[arg(long, help = "Update all installed packages before installing")]
    pub update: bool,
}
