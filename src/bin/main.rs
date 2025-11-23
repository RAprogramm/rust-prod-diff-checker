// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::{
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
    process,
};

use clap::Parser;
use rust_diff_analyzer::{
    analysis::map_changes,
    classifier::rules::calculate_weight,
    config::{Config, OutputFormat},
    error::AppError,
    git::parse_diff,
    output::format_output,
    types::{AnalysisResult, SemanticUnitKind, Summary},
};

/// Semantic analyzer for Rust PR diffs
#[derive(Parser, Debug)]
#[command(name = "rust-diff-analyzer")]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to diff file (reads from stdin if not provided)
    #[arg(short, long)]
    diff_file: Option<PathBuf>,

    /// Path to configuration file
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Output format
    #[arg(short, long, value_enum)]
    format: Option<OutputFormatArg>,

    /// Maximum production units allowed
    #[arg(long)]
    max_units: Option<usize>,

    /// Maximum weighted score allowed
    #[arg(long)]
    max_score: Option<usize>,

    /// Base directory for resolving file paths
    #[arg(short, long, default_value = ".")]
    base_dir: PathBuf,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum OutputFormatArg {
    Github,
    Json,
    Human,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let args = Args::parse();

    let mut config = if let Some(config_path) = &args.config {
        Config::from_file(config_path)?
    } else {
        let default_path = Path::new(".rust-diff-analyzer.toml");
        if default_path.exists() {
            Config::from_file(default_path)?
        } else {
            Config::default()
        }
    };

    if let Some(format) = args.format {
        config.output.format = match format {
            OutputFormatArg::Github => OutputFormat::Github,
            OutputFormatArg::Json => OutputFormat::Json,
            OutputFormatArg::Human => OutputFormat::Human,
        };
    }

    if let Some(max_units) = args.max_units {
        config.limits.max_prod_units = max_units;
    }

    if let Some(max_score) = args.max_score {
        config.limits.max_weighted_score = max_score;
    }

    config.validate()?;

    let diff_content = read_diff(&args.diff_file)?;

    let file_diffs = parse_diff(&diff_content)?;

    let base_dir = args.base_dir.clone();
    let changes = map_changes(&file_diffs, &config, |path| {
        let full_path = base_dir.join(path);
        fs::read_to_string(full_path)
    })?;

    let mut summary = Summary::default();

    for change in &changes {
        if change.classification.is_production() {
            match change.unit.kind {
                SemanticUnitKind::Function => summary.prod_functions += 1,
                SemanticUnitKind::Struct | SemanticUnitKind::Enum => summary.prod_structs += 1,
                _ => summary.prod_other += 1,
            }
            summary.prod_lines_added += change.lines_added;
            summary.prod_lines_removed += change.lines_removed;
            summary.weighted_score += calculate_weight(&change.unit, &config);
        } else {
            summary.test_units += 1;
            summary.test_lines_added += change.lines_added;
            summary.test_lines_removed += change.lines_removed;
        }
    }

    summary.exceeds_limit = summary.total_prod_units() > config.limits.max_prod_units
        || summary.weighted_score > config.limits.max_weighted_score;

    let result = AnalysisResult::new(changes, summary);

    let output = format_output(&result, &config)?;
    print!("{}", output);

    if result.summary.exceeds_limit && config.limits.fail_on_exceed {
        process::exit(1);
    }

    Ok(())
}

fn read_diff(path: &Option<PathBuf>) -> Result<String, AppError> {
    match path {
        Some(p) => fs::read_to_string(p).map_err(|e| AppError::FileRead {
            path: p.clone(),
            source: e,
        }),
        None => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .map_err(AppError::Io)?;
            Ok(buffer)
        }
    }
}
