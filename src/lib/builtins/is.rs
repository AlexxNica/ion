use builtins::man_pages::{check_help, MAN_IS};
use shell::Shell;

pub(crate) fn is(args: &[&str], shell: &mut Shell) -> Result<(), String> {
    match args.len() {
        4 => if args[1] != "not" {
            return Err(format!("Expected 'not' instead found '{}'\n", args[1]).to_string());
        } else if eval_arg(args[2], shell) == eval_arg(args[3], shell) {
            return Err("".to_string());
        },
        3 => if eval_arg(args[1], shell) != eval_arg(args[2], shell) {
            return Err("".to_string());
        },
        2 => if !check_help(args, MAN_IS) {
            return Err("is needs 3 or 4 arguments\n".to_string());
        },
        _ => return Err("is needs 3 or 4 arguments\n".to_string()),
    }

    Ok(())
}

fn eval_arg(arg: &str, shell: &mut Shell) -> String {
    let value = get_var_string(arg, shell);
    if value != "" {
        return value;
    }
    arg.to_string()
}

// On error returns an empty String.
fn get_var_string(name: &str, shell: &mut Shell) -> String {
    if name.chars().nth(0).unwrap() != '$' {
        return "".to_string();
    }

    let var = shell.variables.get_var(&name[1..]);
    let sh_var: &str = match var.as_ref() {
        Some(s) => s,
        None => "",
    };

    sh_var.to_string()
}

#[test]
fn test_is() {
    use shell::ShellBuilder;
    let mut shell = ShellBuilder::new().as_library();
    shell.set_var("x", "value");
    shell.set_var("y", "0");

    // Four arguments
    assert_eq!(
        is(&["is", " ", " ", " "], &mut shell),
        Err("Expected 'not' instead found ' '\n".to_string())
    );
    assert_eq!(
        is(&["is", "not", " ", " "], &mut shell),
        Err("".to_string())
    );
    assert_eq!(
        is(&["is", "not", "$x", "$x"], &mut shell),
        Err("".to_string())
    );
    assert_eq!(is(&["is", "not", "2", "1"], &mut shell), Ok(()));
    assert_eq!(is(&["is", "not", "$x", "$y"], &mut shell), Ok(()));

    // Three arguments
    assert_eq!(is(&["is", "1", "2"], &mut shell), Err("".to_string()));
    assert_eq!(is(&["is", "$x", "$y"], &mut shell), Err("".to_string()));
    assert_eq!(is(&["is", " ", " "], &mut shell), Ok(()));
    assert_eq!(is(&["is", "$x", "$x"], &mut shell), Ok(()));

    // Two arguments
    assert_eq!(
        is(&["is", " "], &mut shell),
        Err("is needs 3 or 4 arguments\n".to_string())
    );
    assert_eq!(is(&["is", "-h"], &mut shell), Ok(()));

    // One argument
    assert_eq!(
        is(&["is"], &mut shell),
        Err("is needs 3 or 4 arguments\n".to_string())
    );
}
