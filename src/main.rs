mod cli;
mod cran;

use crate::cli::*;
use crate::cran::*;

use clap::Parser;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let cran_repo = resolve_cran_repo(&args)?;
    let Cli {
        packages,
        library,
        github,
        gitlab,
        bitbucket,
        codeberg,
        url,
        update,
        ..
    } = args;

    let mut expr_parts = Vec::new();

    if library.is_none() {
        expr_parts.push(
            r#"local({
    lib <- .libPaths()[1]
    if (file.access(lib, 2) != 0) {
        user_lib <- Sys.getenv("R_LIBS_USER")
        if (!nzchar(user_lib)) stop("R_LIBS_USER is not set and default library is not writable")
        user_lib <- normalizePath(user_lib, mustWork = FALSE)
        if (!dir.exists(user_lib)) {
            message(sprintf("Creating personal library: %s", user_lib))
            dir.create(user_lib, recursive = TRUE)
        }
        .libPaths(c(user_lib, .libPaths()))
    }
})"#
            .to_string(),
        );
    }

    if update {
        let mut urls: Vec<String> = Vec::new();
        if let Some(additional_url) = &url {
            urls.push(format!("\"{}\"", escape_r_string(additional_url)));
        }
        urls.push(format!("\"{}\"", escape_r_string(&cran_repo)));

        let repos_expr = urls.join(", ");

        let mut update_expr = format!("update.packages(ask = FALSE, repos = c({}))", repos_expr);

        if let Some(lib) = &library {
            update_expr = format!(
                "update.packages(ask = FALSE, lib.loc = \"{}\", repos = c({}))",
                escape_r_string(lib),
                repos_expr
            );
        }

        expr_parts.push(update_expr);
    }

    if !packages.is_empty() {
        let pkg_list = packages
            .iter()
            .map(|p| format!("\"{}\"", escape_r_string(p)))
            .collect::<Vec<_>>()
            .join(", ");

        let mut urls: Vec<String> = Vec::new();
        if let Some(additional_url) = &url {
            urls.push(format!("\"{}\"", escape_r_string(additional_url)));
        }

        urls.push(format!("\"{}\"", escape_r_string(&cran_repo)));

        let repos_expr = urls.join(", ");

        let mut cran_expr = format!(
            "install.packages(c({}), repos = c({})",
            pkg_list, repos_expr
        );

        if let Some(lib) = &library {
            cran_expr.push_str(&format!(", lib = \"{}\"", escape_r_string(lib)));
        }
        cran_expr.push(')');
        expr_parts.push(cran_expr);
    }

    let mut git_repos: Vec<(&str, &String)> = Vec::new();
    for repo in &github {
        git_repos.push(("github", repo));
    }
    for repo in &gitlab {
        git_repos.push(("gitlab", repo));
    }
    for repo in &bitbucket {
        git_repos.push(("bitbucket", repo));
    }
    for repo in &codeberg {
        git_repos.push(("codeberg", repo));
    }

    if !git_repos.is_empty() {
        let mut remotes_bootstrap = format!(
            "if (!requireNamespace(\"remotes\", quietly = TRUE)) install.packages(\"remotes\", repos = \"{}\"",
            escape_r_string(&cran_repo)
        );
        if let Some(lib) = &library {
            remotes_bootstrap.push_str(&format!(", lib = \"{}\"", escape_r_string(lib)));
        }
        remotes_bootstrap.push(')');
        expr_parts.push(remotes_bootstrap);

        for (source, repo) in git_repos {
            let git_url = build_git_url(source, repo);
            let mut install_expr =
                format!("remotes::install_git(\"{}\"", escape_r_string(&git_url));
            if let Some(lib) = &library {
                install_expr.push_str(&format!(", lib = \"{}\"", escape_r_string(lib)));
            }
            install_expr.push_str(", upgrade = \"never\")");
            expr_parts.push(install_expr);
        }
    }

    if expr_parts.is_empty() {
        return Err("no packages or git sources provided".into());
    }

    let expr = expr_parts.join("; ");

    let status = Command::new("Rscript").arg("-e").arg(&expr).status()?;
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}
