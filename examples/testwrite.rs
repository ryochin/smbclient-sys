extern crate smbclient_sys as smbc;
extern crate libc;

use std::mem;
use std::ffi::{CString};
use libc::{c_char, c_int, c_void, size_t, strncpy, O_WRONLY, O_CREAT, O_TRUNC};

extern "C" fn auth_data(_srv: *const c_char,
            _shr: *const c_char,
            _wg: *mut c_char,
            _glen: c_int,
            un: *mut c_char,
            _unlen: c_int,
            pw: *mut c_char,
            _pwlen: c_int) {
                unsafe {
        strncpy(un, CString::new("vertexclique").unwrap().as_ptr(), 12);
        strncpy(pw, CString::new("1234").unwrap().as_ptr(), 4);
    }
}

pub static mut AUTH_CALLBACK: smbc::smbc_get_auth_data_fn = Some(auth_data);

fn main() {
    println!("Launch...");
    unsafe {
        let fname = CString::new("smb://air5650-nas/rechner/cyberpunk.txt").unwrap();

        smbc::smbc_init(AUTH_CALLBACK, 0);
        let retval: i32 = smbc::smbc_open(fname.as_ptr(), O_CREAT | O_TRUNC | O_WRONLY, 0);
        if retval < 0 {
            println!("Couldn't accessed to a SMB file");
        } else {
            println!("Accessed to specified SMB file");

            // Prepare a data to be written
            let the_cyberpunk_anime = b"Ghost in the Shell";
            let casted_ptr: *mut c_void = mem::transmute(the_cyberpunk_anime);

            // Write it through
            let writeval: isize = smbc::smbc_write(retval, casted_ptr, the_cyberpunk_anime.len() as size_t);
            if writeval > 0 {
                println!("Write successful...");
            } else {
                panic!("Write failed...");
            }

            smbc::smbc_close(writeval as i32);
        }
    }
}
