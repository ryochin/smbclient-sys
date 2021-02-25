extern crate pkg_config;

fn main ()
{
    match pkg_config::probe_library("smbclient") {
        Ok(_) => {
            if cfg!(target_os = "macos") {
                println!("cargo:rustc-flags=-L /opt/samba/lib -l smbclient");
            } else {
                println!("cargo:rustc-flags=-l smbclient");
            }
        },
        Err(e) => {
            println!("error: SMB Client library not found! Probably libsmbclient is not installed.");
            panic!("pkg_config probe failed: {}", e);
        }
    };
}
