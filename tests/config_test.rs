// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#[cfg(test)]
mod config_tests {
    use rust_diff_analyzer::config::Config;

    #[test]
    fn test_default_ignored_authors_is_empty() {
        let config = Config::default();
        assert!(config.classification.ignored_authors.is_empty());
    }

    #[test]
    fn test_should_ignore_author_exact_match() {
        let mut config = Config::default();
        config
            .classification
            .ignored_authors
            .push("dependabot[bot]".to_string());
        assert!(config.should_ignore_author("dependabot[bot]"));
    }

    #[test]
    fn test_should_ignore_author_partial_match() {
        let mut config = Config::default();
        config
            .classification
            .ignored_authors
            .push("dependabot".to_string());
        assert!(config.should_ignore_author("dependabot[bot]"));
        assert!(config.should_ignore_author("dependabot[bot]@users.noreply.github.com"));
    }

    #[test]
    fn test_should_ignore_author_no_match() {
        let mut config = Config::default();
        config
            .classification
            .ignored_authors
            .push("dependabot[bot]".to_string());
        assert!(!config.should_ignore_author("developer"));
        assert!(!config.should_ignore_author("maintainer"));
    }

    #[test]
    fn test_should_ignore_author_empty_list() {
        let config = Config::default();
        assert!(!config.should_ignore_author("any_author"));
    }

    #[test]
    fn test_should_ignore_commit_alias() {
        let mut config = Config::default();
        config
            .classification
            .ignored_authors
            .push("github-actions".to_string());
        assert!(config.should_ignore_commit("github-actions[bot]"));
        assert!(config.should_ignore_commit("github-actions[bot]@users.noreply.github.com"));
    }

    #[test]
    fn test_ignored_authors_multiple() {
        let mut config = Config::default();
        config.classification.ignored_authors = vec![
            "dependabot[bot]".to_string(),
            "github-actions[bot]".to_string(),
            "renovate[bot]".to_string(),
        ];

        assert!(config.should_ignore_author("dependabot[bot]"));
        assert!(config.should_ignore_author("github-actions[bot]"));
        assert!(config.should_ignore_author("renovate[bot]"));
        assert!(!config.should_ignore_author("developer"));
    }

    #[test]
    fn test_validate_rejects_empty_author() {
        let mut config = Config::default();
        config.classification.ignored_authors.push("".to_string());
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_rejects_duplicate_author() {
        let mut config = Config::default();
        config
            .classification
            .ignored_authors
            .push("dependabot".to_string());
        config
            .classification
            .ignored_authors
            .push("dependabot".to_string());
        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_accepts_valid_config() {
        let mut config = Config::default();
        config.classification.ignored_authors = vec![
            "dependabot[bot]".to_string(),
            "github-actions[bot]".to_string(),
        ];
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_case_sensitivity() {
        let mut config = Config::default();
        config
            .classification
            .ignored_authors
            .push("Dependabot".to_string());
        // Exact case match required
        assert!(!config.should_ignore_author("dependabot[bot]"));
        assert!(config.should_ignore_author("Dependabot"));
    }

    #[test]
    fn test_config_builder_ignored_authors() {
        use rust_diff_analyzer::config::ConfigBuilder;

        let config = ConfigBuilder::new()
            .add_ignored_author("dependabot[bot]")
            .build();

        assert!(config.should_ignore_author("dependabot[bot]"));
    }
}
