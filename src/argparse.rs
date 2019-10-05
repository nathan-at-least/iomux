use std::convert::AsRef;
use std::ffi::OsStr;
use std::process::Command;

pub fn build_commands<T, I>(args: T) -> Vec<Command>
where
    T: IntoIterator<Item = I>,
    I: AsRef<OsStr>,
{
    let mut it = args.into_iter();
    it.next(); // Skip self command name.

    let mut cmds = vec![];
    while let Some(cmd) = it.next() {
        let mut cmd = Command::new(cmd);
        while let Some(argish) = it.next() {
            let arg = argish.as_ref();
            if arg == "--" {
                break;
            } else if let Some(s) = arg.to_str() {
                if s.len() > 2 && s.chars().all(|c| c == '-') {
                    cmd.arg(&s[1..]);
                } else {
                    cmd.arg(s);
                }
            } else {
                cmd.arg(arg);
            }
        }
        cmds.push(cmd);
    }

    cmds
}

#[cfg(test)]
mod tests {
    use super::build_commands;

    macro_rules! debug_case {
        ( $name:ident, $expected:expr, [ $( $arg:expr ),* ] ) => {
            #[test]
            fn $name() {
                let args: Vec<&str> = vec![ $( $arg ),* ];
                let actual = build_commands(args);
                assert_eq!(
                    $expected,
                    format!("{:?}", actual)
                );
            }
        }
    }

    debug_case!(empty, "[]", []);
    debug_case!(no_args, "[]", ["selfprog"]);
    debug_case!(
        one_command,
        "[\"echo\" \"hello\" \"world\"]",
        ["selfprog", "echo", "hello", "world"]
    );
    debug_case!(
        two_commands,
        "[\"echo\" \"hello\" \"world\", \"date\"]",
        ["selfprog", "echo", "hello", "world", "--", "date"]
    );
    debug_case!(
        escaped_separator,
        "[\"sh\" \"--\" \"foo\", \"date\"]",
        ["selfprog", "sh", "---", "foo", "--", "date"]
    );
    debug_case!(
        escaped_escape,
        "[\"sh\" \"---\" \"foo\", \"date\"]",
        ["selfprog", "sh", "----", "foo", "--", "date"]
    );
    debug_case!(
        tricky_nonescape,
        "[\"weird\" \"---x\" \"foo\", \"date\"]",
        ["selfprog", "weird", "---x", "foo", "--", "date"]
    );
}
