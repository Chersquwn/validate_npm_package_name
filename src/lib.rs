use std::sync::LazyLock;

use builtin_modules::BUILTIN_MODULES;
use regex::Regex;
use uri_encode::encode_uri_component;

#[derive(PartialEq, Debug)]
pub struct ValidateResult {
    pub valid_for_new_packages: bool,
    pub valid_for_old_packages: bool,
    pub warnings: Option<Vec<String>>,
    pub errors: Option<Vec<String>>,
}

static SCOPE_PACKAGE_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:@([^/]+?)[/])?([^/]+?)$").unwrap());

static BLACKLIST: LazyLock<[String; 2]> =
    LazyLock::new(|| [String::from("node_modules"), String::from("favicon.ico")]);

/// validate npm package name
///
/// # Example
/// ```
/// assert_eq!(
///     validate_npm_package_name::validate(&String::from("some-package")),
///     validate_npm_package_name::ValidateResult {
///         valid_for_new_packages: true,
///         valid_for_old_packages: true,
///         warnings: None,
///         errors: None
///     }
/// );
///
/// assert_eq!(
///     validate_npm_package_name::validate(&String::from("_start-with-underscore")),
///     validate_npm_package_name::ValidateResult {
///         valid_for_new_packages: false,
///         valid_for_old_packages: false,
///         warnings: None,
///         errors: Some(vec![String::from("name cannot start with an underscore")])
///     }  
/// );
/// ```
pub fn validate(name: &String) -> ValidateResult {
    let mut warnings: Vec<String> = vec![];
    let mut errors: Vec<String> = vec![];

    if name.is_empty() {
        errors.push(String::from("name length must be greater than zero"));
    }

    if Regex::new(r"^\.").unwrap().is_match(name) {
        errors.push(String::from("name cannot start with a period"));
    }

    if Regex::new(r"^_").unwrap().is_match(name) {
        errors.push(String::from("name cannot start with an underscore"));
    }

    if name.trim() != name {
        errors.push(String::from(
            "name cannot contain leading or trailing spaces",
        ));
    }

    BLACKLIST.iter().for_each(|blacklisted_name| {
        if name.to_lowercase() == *blacklisted_name {
            errors.push(format!("{blacklisted_name} is a blacklisted name"));
        }
    });

    if BUILTIN_MODULES.contains(&name.to_lowercase()) {
        warnings.push(format!("{name} is a core module name"));
    }

    if name.len() > 214 {
        warnings.push(String::from(
            "name can no longer contain more than 214 characters",
        ));
    }

    if name.to_lowercase() != *name {
        warnings.push(String::from("name can no longer contain capital letters"));
    }

    if Regex::new(r"[~'!()*]")
        .unwrap()
        .is_match(name.split("/").last().unwrap())
    {
        warnings.push(String::from(
            "name can no longer contain special characters (\"~\'!()*\")",
        ));
    }

    if encode_uri_component(name) != *name {
        let name_matches = SCOPE_PACKAGE_PATTERN.captures(name);

        if let Some(matches) = name_matches {
            let scope = match matches.get(1) {
                None => String::new(),
                Some(x) => x.as_str().to_string(),
            };
            let pkg = match matches.get(2) {
                None => String::new(),
                Some(x) => x.as_str().to_string(),
            };

            if encode_uri_component(&scope) == scope && encode_uri_component(&pkg) == pkg {
                return done(warnings, errors);
            }
        }

        errors.push(String::from(
            "name can only contain URL-friendly characters",
        ));
    }

    done(warnings, errors)
}

fn done(warnings: Vec<String>, errors: Vec<String>) -> ValidateResult {
    ValidateResult {
        valid_for_new_packages: errors.is_empty() && warnings.is_empty(),
        valid_for_old_packages: errors.is_empty(),
        warnings: if warnings.is_empty() {
            None
        } else {
            Some(warnings)
        },
        errors: if errors.is_empty() {
            None
        } else {
            Some(errors)
        },
    }
}
