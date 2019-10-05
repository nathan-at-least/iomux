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
        while let Some(arg) = it.next() {
            if arg.as_ref() == "--" {
                break;
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
    use crate::todebug::ToDebug;

    macro_rules! debug_case {
        ( $name:ident, $expected:expr, [ $( $arg:expr ),* ] ) => {
            #[test]
            fn $name() {
                let args: Vec<&str> = vec![ $( $arg ),* ];
                let actual = build_commands(args);
                assert_eq!($expected, actual.to_debug());
            }
        }
    }

    debug_case!(empty, "[]", []);
    debug_case!(no_args, "[]", ["selfprog"]);
}
