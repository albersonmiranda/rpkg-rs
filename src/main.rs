mod cli;
mod cran;
mod expression;

use crate::cli::*;
use crate::cran::*;
use crate::expression::*;

use clap::Parser;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let cran_repo = resolve_cran_repo(&args)?;
    let expr = build_rscript_expr(&args, &cran_repo)?;

    if expr.is_empty() {
        return Err("no packages or git sources provided".into());
    }

    let status = Command::new("Rscript").arg("-e").arg(&expr).status()?;
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}
