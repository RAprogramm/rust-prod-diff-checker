// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::{
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
    process,
};

use clap::Parser;
use masterror::AppError;
use rust_diff_analyzer::{
    analysis::map_changes,
    classifier::rules::calculate_weight,
    config::{Config, OutputFormat},
    error::FileReadError,
    git::parse_diff,
    output::format_output,
    types::{AnalysisResult, Change, SemanticUnitKind, Summary},
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

    /// Maximum production lines added
    #[arg(long)]
    max_lines: Option<usize>,

    /// Base directory for resolving file paths
    #[arg(short, long, default_value = ".")]
    base_dir: PathBuf,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum OutputFormatArg {
    Github,
    Json,
    Human,
    Comment,
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
            OutputFormatArg::Comment => OutputFormat::Comment,
        };
    }

    if let Some(max_units) = args.max_units {
        config.limits.max_prod_units = max_units;
    }

    if let Some(max_score) = args.max_score {
        config.limits.max_weighted_score = max_score;
    }

    if let Some(max_lines) = args.max_lines {
        config.limits.max_prod_lines = Some(max_lines);
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
        || summary.weighted_score > config.limits.max_weighted_score
        || config
            .limits
            .max_prod_lines
            .map(|limit| summary.prod_lines_added > limit)
            .unwrap_or(false)
        || check_per_type_limits(&changes, &config);

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
        Some(p) => {
            fs::read_to_string(p).map_err(|e| AppError::from(FileReadError::new(p.clone(), e)))
        }
        None => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .map_err(|e| AppError::from(rust_diff_analyzer::error::IoError(e)))?;
            Ok(buffer)
        }
    }
}

fn check_per_type_limits(changes: &[Change], config: &Config) -> bool {
    let per_type = match &config.limits.per_type {
        Some(limits) => limits,
        None => return false,
    };

    let mut functions = 0;
    let mut structs = 0;
    let mut enums = 0;
    let mut traits = 0;
    let mut impl_blocks = 0;
    let mut consts = 0;
    let mut statics = 0;
    let mut type_aliases = 0;
    let mut macros = 0;
    let mut modules = 0;

    for change in changes {
        if !change.classification.is_production() {
            continue;
        }

        match change.unit.kind {
            SemanticUnitKind::Function => functions += 1,
            SemanticUnitKind::Struct => structs += 1,
            SemanticUnitKind::Enum => enums += 1,
            SemanticUnitKind::Trait => traits += 1,
            SemanticUnitKind::Impl => impl_blocks += 1,
            SemanticUnitKind::Const => consts += 1,
            SemanticUnitKind::Static => statics += 1,
            SemanticUnitKind::TypeAlias => type_aliases += 1,
            SemanticUnitKind::Macro => macros += 1,
            SemanticUnitKind::Module => modules += 1,
        }
    }

    per_type.functions.map(|l| functions > l).unwrap_or(false)
        || per_type.structs.map(|l| structs > l).unwrap_or(false)
        || per_type.enums.map(|l| enums > l).unwrap_or(false)
        || per_type.traits.map(|l| traits > l).unwrap_or(false)
        || per_type
            .impl_blocks
            .map(|l| impl_blocks > l)
            .unwrap_or(false)
        || per_type.consts.map(|l| consts > l).unwrap_or(false)
        || per_type.statics.map(|l| statics > l).unwrap_or(false)
        || per_type
            .type_aliases
            .map(|l| type_aliases > l)
            .unwrap_or(false)
        || per_type.macros.map(|l| macros > l).unwrap_or(false)
        || per_type.modules.map(|l| modules > l).unwrap_or(false)
}
