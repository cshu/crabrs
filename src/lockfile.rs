use crate::*;
use std::*;

pub fn file_lock(lock_p: &path::Path, content_if_created: &[u8]) -> CustRes<fs::File> {
    use fs2::*;
    if !lock_p.try_exists()? {
        fs::write(lock_p, content_if_created)?;
    }
    //both windows and unix allow 2 processes File::open simultaneously
    let fobj = fs::File::open(lock_p)?;
    fobj.lock_exclusive()?; //both windows and unix block here
    Ok(fobj)
}
pub fn file_unlock(fobj: fs::File) -> bool {
    use fs2::*;
    if let Err(err) = fobj.unlock() {
        error!("{}", "FAILED TO UNLOCK FILE");
        error!("{}", err);
        return false;
    }
    true
}
