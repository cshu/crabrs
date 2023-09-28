use crate::*;
use std::ffi::*;

pub fn git_log_1_pretty_format<S1: AsRef<OsStr>, S2: AsRef<OsStr>>(
    cwd: &std::path::Path,
    fmtarg: S1,
    filenm: S2,
) -> CustRes<std::process::Output> {
    let out = std::process::Command::new("git")
        .current_dir(cwd)
        .arg("log")
        .arg("-1")
        .arg(fmtarg)
        .arg(filenm)
        .output()?;
    if !out.status.success() {
        return dummy_err("Exit status indicates failure.");
    }
    Ok(out)
}

pub fn git_log_1_pretty_format_iso<S: AsRef<OsStr>>(
    cwd: &std::path::Path,
    filenm: S,
) -> CustRes<String> {
    let out = git_log_1_pretty_format(cwd, r##"--pretty=format:%cI"##, filenm)?;
    Ok(String::from_utf8_lossy(&out.stdout).into())
}

pub fn git_log_1_pretty_format_isolike<S: AsRef<OsStr>>(
    cwd: &std::path::Path,
    filenm: S,
) -> CustRes<String> {
    let out = git_log_1_pretty_format(cwd, r##"--pretty=format:%ci"##, filenm)?;
    Ok(String::from_utf8_lossy(&out.stdout).into())
}

pub fn git_log_1_pretty_format_ct<S: AsRef<OsStr>>(
    cwd: &std::path::Path,
    filenm: S,
) -> CustRes<i64> {
    let out = git_log_1_pretty_format(cwd, r##"--pretty=format:%ct"##, filenm)?;
    Ok(String::from_utf8_lossy(&out.stdout).parse()?)
}

#[test]
fn test_git_log_1_pretty_format() {
    let file_under_git = match std::env::var("file_under_git") {
        Err(_) => {
            coutln!("No file provided for file_under_git");
            return;
        }
        Ok(inner) => inner,
    };
    let pb: std::path::PathBuf = (&file_under_git).into();
    let pdir = pb.parent().unwrap();
    dbg!(git_log_1_pretty_format_ct(&pdir, &file_under_git).unwrap());
    dbg!(git_log_1_pretty_format_isolike(&pdir, &file_under_git).unwrap());
    dbg!(git_log_1_pretty_format_iso(&pdir, &file_under_git).unwrap());
}

pub fn git_version() -> std::io::Result<std::process::Output> {
    std::process::Command::new("git").arg("--version").output()
}

pub fn chk_git_usable() -> bool {
    let gitout = match git_version() {
        Err(_) => {
            return false;
        }
        Ok(inner) => inner,
    };
    gitout.status.success()
}
