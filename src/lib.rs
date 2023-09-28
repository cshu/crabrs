mod gitcmd;
mod hash;
mod lockfile;
mod treefile;

pub use crate::gitcmd::*;
pub use crate::hash::*;
pub use crate::lockfile::*;
pub use crate::treefile::*;

use log::*;

pub fn encode_uri_component(com: &str) -> String {
    use url::form_urlencoded;
    let encoded: String = form_urlencoded::Serializer::new(String::new())
        .append_key_only(com)
        .finish();
    encoded
}

pub fn now_in_millis() -> i64 {
    systemtime2millis(std::time::SystemTime::now())
}

pub fn systemtime2millis(st: std::time::SystemTime) -> i64 {
    match st.duration_since(std::time::SystemTime::UNIX_EPOCH) {
        Ok(dur) => dur.as_millis() as i64,
        Err(sterr) => -(sterr.duration().as_millis() as i64),
    }
}
pub fn exists_without_following_sym(pat: &std::path::Path) -> std::io::Result<bool> {
    //note race condition is possible
    if pat.is_symlink() {
        return Ok(true);
    }
    pat.try_exists()
}
pub fn real_reg_file_without_symlink(pat: &std::path::Path) -> bool {
    //note race condition is possible
    !pat.is_symlink() && pat.is_file()
}

pub fn real_dir_without_symlink(pat: &std::path::Path) -> bool {
    //note race condition is possible
    !pat.is_symlink() && pat.is_dir()
}

pub fn possible_to_create_new_file(pat: &std::path::Path) -> std::io::Result<bool> {
    if exists_without_following_sym(pat)? {
        return Ok(false);
    }
    let mut ancestors = pat.ancestors();
    if ancestors.next().is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unexpected path with zero component.",
        ));
    }
    for anc in ancestors {
        if exists_without_following_sym(anc)? {
            return Ok(real_dir_without_symlink(anc));
        }
    }
    return Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to find any existing ancestor.",
    ));
}

pub fn read_to_string_with_path_empty_chk(path: &str) -> std::io::Result<String> {
    if path.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Path is empty.",
        ));
    }
    std::fs::read_to_string(path)
}

#[macro_export]
macro_rules! eq_exclam {
    ($arg: expr) => {
        $arg = !$arg;
    };
}

#[macro_export]
macro_rules! debug_assert_after_eval {
    ($arg: expr) => {{
        let _ok = $arg;
        debug_assert!(_ok);
    }};
}

#[macro_export]
macro_rules! assert_always {
    ($arg: expr) => {{
        if !($arg) {
            panic!("ERROR assert_always failed");
        }
    }};
}

#[macro_export]
macro_rules! cout_n_flush_input {
    ($msg: expr, $lns: expr, $none_ret: expr) => {{
        cout_n_flush!($msg);
        match $lns.next() {
            None => {
                println!("{}", "Input ended.");
                return Ok($none_ret);
            }
            Some(Err(err)) => {
                return Err(err.into());
            }
            Some(Ok(linestr)) => linestr,
        }
    }};
}

#[macro_export]
macro_rules! cout_n_flush_input_none_ret_false {
    ($msg: expr, $lns: expr) => {
        cout_n_flush_input!($msg, $lns, false)
    };
}

pub fn read_to_buf<T: std::io::Read>(rd: &mut T, buf: &mut [u8]) -> CustRes<usize> {
    let mut off = 0;
    Ok(loop {
        let rlen = rd.read(&mut buf[off..])?;
        if rlen == 0 {
            break off;
        }
        off += rlen;
        if off == buf.len() {
            break off;
        }
    })
}

pub fn read_n_find<T: std::io::Read>(
    rd: &mut T,
    buf: &mut [u8],
    needle: &[u8],
) -> CustRes<Option<usize>> {
    use memchr::memmem;
    debug_assert!(!needle.is_empty());
    debug_assert!(buf.len() > needle.len());
    let mut off = 0;
    let mut rlen = read_to_buf(rd, buf)?;
    loop {
        if let Some(idx) = memmem::find(&buf[0..rlen], needle) {
            break Ok(Some(off + idx));
        }
        if rlen < buf.len() {
            break Ok(None);
        }
        let memmovelen = needle.len() - 1;
        let discardlen = buf.len() - memmovelen;
        buf.copy_within(discardlen.., 0);
        off += discardlen;
        rlen = memmovelen + read_to_buf(rd, &mut buf[memmovelen..])?;
    }
}

fn bytes2hex(bytes: &[u8]) -> String {
    let mut retval = String::with_capacity(bytes.len() * 2);
    for octet in bytes {
        use std::fmt::Write;
        write!(&mut retval, "{:02x}", octet).unwrap();
    }
    retval
}

pub struct StdinWrapper {
    pub lines: std::io::Lines<std::io::StdinLock<'static>>,
}
impl Default for StdinWrapper {
    fn default() -> Self {
        use log::*;
        info!("{}", "READING STDIN");
        let stdin = std::io::stdin();
        use std::io::prelude::*;
        Self {
            lines: stdin.lock().lines(),
        }
    }
}

#[macro_export]
macro_rules! subsli_rear {
    ($sli: expr, $rlen: expr) => {
        &$sli[$sli.len() - $rlen..]
    };
}

#[macro_export]
macro_rules! subsli_cut_rear {
    ($sli: expr, $rlen: expr) => {
        &$sli[..$sli.len() - $rlen]
    };
}

#[macro_export]
macro_rules! coutln {
    ($arg: expr) => {
        println!("{}", $arg);
    };
    ($arg0: expr, $arg1: expr) => {
        println!("{}{}", $arg0, $arg1);
    };
    ($arg0: expr, $arg1: expr, $arg2: expr) => {
        println!("{}{}{}", $arg0, $arg1, $arg2);
    };
    ($arg0: expr, $arg1: expr, $arg2: expr, $arg3: expr) => {
        println!("{}{}{}{}", $arg0, $arg1, $arg2, $arg3);
    };
    ($arg0: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr) => {
        println!("{}{}{}{}{}", $arg0, $arg1, $arg2, $arg3, $arg4);
    };
    ($arg0: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr, $arg5: expr) => {
        println!("{}{}{}{}{}{}", $arg0, $arg1, $arg2, $arg3, $arg4, $arg5);
    };
}

#[macro_export]
macro_rules! cerrln {
    ($arg: expr) => {
        eprintln!("{}", $arg);
    };
}

#[macro_export]
macro_rules! cout_n_flush {
    ($arg: expr) => {
        print!("{}", $arg);
        use std::io::Write;
        io::stdout().flush()?;
    };
}

pub fn dummy_err<T1, T2: std::fmt::Display>(msg: T2) -> Result<T1, CustomErr> {
    use log::*;
    error!("{}", msg);
    Err(CustomErr {})
}

pub type CustRes<T> = Result<T, CustomErr>;

pub struct CustomSimpleErr {}
impl<E: std::fmt::Display> From<E> for CustomSimpleErr {
    fn from(inner: E) -> Self {
        use log::*;
        error!("{}", inner);
        Self {}
    }
}

//#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomErr {
    //inner: Error
}
impl From<CustomSimpleErr> for CustomErr {
    fn from(_inner: CustomSimpleErr) -> Self {
        Self {}
    }
}

//note E: std::fmt::Debug instead of Display works in most cases but it makes #[derive(Debug)] impossible thus you cannot even `unwrap` with CustRes. One solution is to use `.unwrap_or_else(|_|->usize{panic!();})` instead of `unwrap`
impl<E: std::fmt::Debug> From<E> for CustomErr {
    #[track_caller]
    fn from(inner: E) -> Self {
        use log::*;
        use std::backtrace::*;
        //note sometimes some line numbers are not captured and even some fn names are not captured (optimized out). The fix is to change profile debug=1
        error!(
            "{:?}\n{:?}\n{}",
            inner,
            std::panic::Location::caller(),
            Backtrace::force_capture()
        );
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_uri_component() {
        eprintln!("{}", encode_uri_component(r##"'wwwwwwwww='"##));
        //assert_eq!(result, 4);
    }
}
