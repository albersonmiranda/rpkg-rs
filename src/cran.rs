use crate::cli::Cli;

use std::cmp::Ordering;
use std::io::{self, IsTerminal, Write};
use std::process::Command;
use strsim::jaro_winkler;

pub const DEFAULT_CRAN_REPO: &str = "https://cloud.r-project.org/";

#[derive(Debug, Clone)]
pub struct Mirror {
    name: String,
    country: String,
    city: String,
    url: String,
}

pub fn escape_r_string(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

fn fetch_cran_mirrors() -> Result<Vec<Mirror>, Box<dyn std::error::Error>> {
    let expr = r#"
mirrors <- getCRANmirrors(all = FALSE, local.only = FALSE)
url_col <- if ("URL" %in% names(mirrors)) "URL" else "url"
for (i in seq_len(nrow(mirrors))) {
    fields <- c(mirrors$Name[i], mirrors$Country[i], mirrors$City[i], mirrors[[url_col]][i])
    fields <- gsub("[\t\n\r]", " ", fields)
    cat(paste(fields, collapse = "\t"), "\n", sep = "")
}
"#;

    let output = Command::new("Rscript").arg("-e").arg(expr).output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("failed to retrieve CRAN mirrors: {}", stderr.trim()).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mirrors = stdout
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(4, '\t');
            let name = parts.next()?.trim().to_string();
            let country = parts.next()?.trim().to_string();
            let city = parts.next()?.trim().to_string();
            let url = parts.next()?.trim().to_string();
            if url.is_empty() {
                return None;
            }
            Some(Mirror {
                name,
                country,
                city,
                url,
            })
        })
        .collect::<Vec<_>>();

    if mirrors.is_empty() {
        return Err("no CRAN mirrors were returned by R".into());
    }

    Ok(mirrors)
}

fn ranked_mirrors<'a>(query: &str, mirrors: &'a [Mirror]) -> Vec<&'a Mirror> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return Vec::new();
    }

    let mut scored = mirrors
        .iter()
        .filter_map(|mirror| {
            let searchable = format!(
                "{} {} {}",
                mirror.name.to_lowercase(),
                mirror.country.to_lowercase(),
                mirror.city.to_lowercase()
            );

            let contains = searchable.contains(&q);
            let name_score = jaro_winkler(&mirror.name.to_lowercase(), &q);
            let country_score = jaro_winkler(&mirror.country.to_lowercase(), &q);
            let city_score = jaro_winkler(&mirror.city.to_lowercase(), &q);
            let mut score = name_score.max(country_score).max(city_score);
            if contains {
                score += 1.0;
            }

            if contains || score >= 0.83 {
                Some((mirror, score))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
    scored.into_iter().map(|(mirror, _)| mirror).collect()
}

fn select_mirror(country: &str, matches: &[&Mirror]) -> Result<String, Box<dyn std::error::Error>> {
    if matches.is_empty() {
        return Err(format!("no CRAN mirrors matched country query '{}'", country).into());
    }

    let max_options = matches.len().min(10);
    let shown = &matches[..max_options];

    println!("Matched CRAN mirrors for '{}':", country);
    for (idx, mirror) in shown.iter().enumerate() {
        println!(
            "{}. {} - {} ({}) -> {}",
            idx + 1,
            mirror.country,
            mirror.city,
            mirror.name,
            mirror.url
        );
    }

    if shown.len() == 1 {
        return Ok(shown[0].url.clone());
    }

    if !io::stdin().is_terminal() {
        println!(
            "Non-interactive mode: auto-selecting #1 -> {}",
            shown[0].url
        );
        return Ok(shown[0].url.clone());
    }

    loop {
        print!("Select repository number [1]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Ok(shown[0].url.clone());
        }

        match trimmed.parse::<usize>() {
            Ok(choice) if (1..=shown.len()).contains(&choice) => {
                return Ok(shown[choice - 1].url.clone())
            }
            _ => {
                println!(
                    "Invalid selection. Choose a number between 1 and {}.",
                    shown.len()
                );
            }
        }
    }
}

pub fn resolve_cran_repo(args: &Cli) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(country) = &args.country {
        let mirrors = fetch_cran_mirrors()?;
        let matches = ranked_mirrors(country, &mirrors);
        return select_mirror(country, &matches);
    }

    Ok(DEFAULT_CRAN_REPO.to_string())
}
