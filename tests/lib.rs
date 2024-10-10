use validate_npm_package_name::{validate, ValidateResult};

#[test]
fn it_should_work() {
    assert_eq!(
        validate(&String::from("some-package")),
        ValidateResult {
            valid_for_new_packages: true,
            valid_for_old_packages: true,
            warnings: None,
            errors: None
        }
    );

    assert_eq!(
        validate(&String::from("example.com")),
        ValidateResult {
            valid_for_new_packages: true,
            valid_for_old_packages: true,
            warnings: None,
            errors: None
        }
    );

    assert_eq!(
        validate(&String::from("under_score")),
        ValidateResult {
            valid_for_new_packages: true,
            valid_for_old_packages: true,
            warnings: None,
            errors: None
        }
    );

    assert_eq!(
        validate(&String::from("period.js")),
        ValidateResult {
            valid_for_new_packages: true,
            valid_for_old_packages: true,
            warnings: None,
            errors: None
        }
    );

    assert_eq!(
        validate(&String::from("123numeric")),
        ValidateResult {
            valid_for_new_packages: true,
            valid_for_old_packages: true,
            warnings: None,
            errors: None
        }
    );

    assert_eq!(
        validate(&String::from("crazy!")),
        ValidateResult {
            valid_for_new_packages: false,
            valid_for_old_packages: true,
            warnings: Some(vec![String::from(
                "name can no longer contain special characters (\"~\'!()*\")"
            )]),
            errors: None
        }
    );
}

#[test]
fn it_should_work_with_scope() {
    assert_eq!(
        validate(&String::from("@npm/thingy")),
        ValidateResult {
            valid_for_new_packages: true,
            valid_for_old_packages: true,
            warnings: None,
            errors: None
        }
    );

    assert_eq!(
        validate(&String::from("@npm-zors/money!time.js")),
        ValidateResult {
            valid_for_new_packages: false,
            valid_for_old_packages: true,
            warnings: Some(vec![String::from(
                "name can no longer contain special characters (\"~\'!()*\")"
            )]),
            errors: None
        }
    );
}

#[test]
fn it_should_work_with_invalid_name() {
    assert_eq!(
        validate(&String::from("")),
        ValidateResult {
            valid_for_new_packages: false,
            valid_for_old_packages: false,
            warnings: None,
            errors: Some(vec![String::from("name length must be greater than zero")])
        }
    );

    assert_eq!(
        validate(&String::from(".start-with-period")),
        ValidateResult {
            valid_for_new_packages: false,
            valid_for_old_packages: false,
            warnings: None,
            errors: Some(vec![String::from("name cannot start with a period")])
        }
    );

    assert_eq!(
        validate(&String::from("_start-with-underscore")),
        ValidateResult {
            valid_for_new_packages: false,
            valid_for_old_packages: false,
            warnings: None,
            errors: Some(vec![String::from("name cannot start with an underscore")])
        }
    );

    assert_eq!(
        validate(&String::from("contain:colons")),
        ValidateResult {
            valid_for_new_packages: false,
            valid_for_old_packages: false,
            warnings: None,
            errors: Some(vec![String::from(
                "name can only contain URL-friendly characters"
            )])
        }
    );

    assert_eq!(
        validate(&String::from(" leading-space")),
        ValidateResult {
            valid_for_new_packages: false,
            valid_for_old_packages: false,
            warnings: None,
            errors: Some(vec![
                String::from("name cannot contain leading or trailing spaces"),
                String::from("name can only contain URL-friendly characters")
            ])
        }
    );

    assert_eq!(
        validate(&String::from("trailing-space ")),
        ValidateResult {
            valid_for_new_packages: false,
            valid_for_old_packages: false,
            warnings: None,
            errors: Some(vec![
                String::from("name cannot contain leading or trailing spaces"),
                String::from("name can only contain URL-friendly characters")
            ])
        }
    );

    assert_eq!(
        validate(&String::from("s/l/a/s/h/e/s")),
        ValidateResult {
            valid_for_new_packages: false,
            valid_for_old_packages: false,
            warnings: None,
            errors: Some(vec![String::from(
                "name can only contain URL-friendly characters"
            )])
        }
    );

    assert_eq!(
        validate(&String::from("node_modules")),
        ValidateResult {
            valid_for_new_packages: false,
            valid_for_old_packages: false,
            warnings: None,
            errors: Some(vec![String::from("node_modules is a blacklisted name")])
        }
    );

    assert_eq!(
        validate(&String::from("favicon.ico")),
        ValidateResult {
            valid_for_new_packages: false,
            valid_for_old_packages: false,
            warnings: None,
            errors: Some(vec![String::from("favicon.ico is a blacklisted name")])
        }
    );
}

#[test]
fn it_should_work_with_nodejs_module() {
    assert_eq!(
        validate(&String::from("http")),
        ValidateResult {
            valid_for_new_packages: false,
            valid_for_old_packages: true,
            warnings: Some(vec![String::from("http is a core module name")]),
            errors: None
        }
    );

    assert_eq!(
        validate(&String::from("process")),
        ValidateResult {
            valid_for_new_packages: false,
            valid_for_old_packages: true,
            warnings: Some(vec![String::from("process is a core module name")]),
            errors: None
        }
    );
}

#[test]
fn it_should_work_with_long_name() {
    assert_eq!(
        validate(&String::from("ifyouwanttogetthesumoftwonumberswherethosetwonumbersarechosenbyfindingthelargestoftwooutofthreenumbersandsquaringthemwhichismultiplyingthembyitselfthenyoushouldinputthreenumbersintothisfunctionanditwilldothatforyou-")),
        ValidateResult {
            valid_for_new_packages: false,
            valid_for_old_packages: true,
            warnings: Some(vec![String::from("name can no longer contain more than 214 characters")]),
            errors: None
        }
    );

    assert_eq!(
        validate(&String::from("ifyouwanttogetthesumoftwonumberswherethosetwonumbersarechosenbyfindingthelargestoftwooutofthreenumbersandsquaringthemwhichismultiplyingthembyitselfthenyoushouldinputthreenumbersintothisfunctionanditwilldothatforyou")),
        ValidateResult {
            valid_for_new_packages: true,
            valid_for_old_packages: true,
            warnings: None,
            errors: None
        }
    );
}

#[test]
fn it_should_work_with_legacy_mixed_case() {
    assert_eq!(
        validate(&String::from("CAPITAL-LETTERS")),
        ValidateResult {
            valid_for_new_packages: false,
            valid_for_old_packages: true,
            warnings: Some(vec![String::from(
                "name can no longer contain capital letters"
            )]),
            errors: None
        }
    );
}
