use workdir::Workdir;

fn data(headers: bool) -> Vec<Vec<String>> {
    let mut rows = vec![
        svec!["foobar", "barfoo"],
        svec!["a", "b"],
        svec!["barfoo", "foobar"],
    ];
    if headers { rows.insert(0, svec!["h1", "h2"]); }
    rows
}

#[test]
fn search() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn search_no_headers() {
    let wrk = Workdir::new("search_no_headers");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn search_select() {
    let wrk = Workdir::new("search_select");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");
    cmd.arg("--select").arg("h2");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["barfoo", "foobar"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn search_select_no_headers() {
    let wrk = Workdir::new("search_select_no_headers");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");
    cmd.arg("--select").arg("2");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["barfoo", "foobar"],
    ];
    assert_eq!(got, expected);
}
