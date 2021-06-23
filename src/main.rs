mod util;

use crate::util::get_special_folder_path;
use crate::util::get_user_name;
use crate::util::ConstantSpecialItemIdList;
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let desktop_folder_path = get_special_folder_path(ConstantSpecialItemIdList::Desktop, false)
        .or_else(|| {
            let user_profile = std::env::var_os("USERPROFILE").or_else(|| {
                let user = get_user_name().ok()?;
                Some(OsString::from(format!(
                    "C:/Users/{}/Desktop",
                    user.to_string_lossy()
                )))
            })?;
            Some(PathBuf::from(user_profile).join("Desktop"))
        })
        .expect("failed to locate desktop_folder_path");

    let nul = desktop_folder_path.canonicalize()?.join("NUL"); 
    let mut file = File::create(nul)?;
    writeln!(file, "pranked")?;
    Ok(())
}
