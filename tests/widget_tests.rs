#[test]
fn test_single_empty_child() -> std::io::Result<()> {
    run_single_child(
        "\
         Ok(Outline(\"1, 2, 3\\n\"))\n\
         Ok(Status(ExitStatus(ExitStatus(0))))\n\
         ",
    )
}

fn run_single_child<S: AsRef<str>>(expected: S) -> std::io::Result<()> {
    use std::collections::HashSet;
    use std::os::unix::process::ExitStatusExt;

    fn lineset_of(text: &str) -> HashSet<&str> {
        let mut set = HashSet::new();

        for line in text.split('\n') {
            set.insert(line);
        }

        set
    }

    let expectedset = lineset_of(expected.as_ref());

    let output = std::process::Command::new("target/debug/iomux")
        .arg("target/debug/iomux-testwidget")
        .arg("3")
        .output()?;

    let actualset = lineset_of(std::str::from_utf8(&output.stdout[..]).expect("utf8 output"));

    assert_eq!(std::process::ExitStatus::from_raw(0), output.status);
    assert_eq!(expectedset, actualset);
    assert_eq!(Vec::<u8>::new(), output.stderr);

    Ok(())
}
