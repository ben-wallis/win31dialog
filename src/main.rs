use std::ptr::null_mut;
use std::mem::size_of;

use winapi::{
    shared::{
        minwindef::{HINSTANCE, LPVOID},
        windef::HWND
    },
    um::commdlg::{GetOpenFileNameW, OPENFILENAMEW, OFN_ALLOWMULTISELECT}
};

fn main() {
    let filter: Vec<u16> = "Text Files (*.txt)\0*.txt\0\0".encode_utf16().collect();
    let title: Vec<u16> = "Selecting files like it's 1992\0".encode_utf16().collect();
    let mut file: Vec<u16> = "\0".encode_utf16().collect();
    file.resize(260, 0);

    let initial_dir: Vec<u16> = "\0".encode_utf16().collect();

    let mut openfilenamew = OPENFILENAMEW {
        lStructSize: size_of::<OPENFILENAMEW>() as u32,
        hwndOwner: 0 as HWND, // TODO: owner
        hInstance: 0 as HINSTANCE,
        lpstrFilter: filter.as_ptr(),
        lpstrCustomFilter: null_mut(),
        nMaxCustFilter: 0,
        nFilterIndex: 1,
        lpstrFile: file.as_mut_ptr(),
        nMaxFile: file.len() as u32,
        lpstrFileTitle: null_mut(),
        nMaxFileTitle: 0,
        lpstrInitialDir: initial_dir.as_ptr(),
        lpstrTitle: title.as_ptr(),
        Flags: OFN_ALLOWMULTISELECT,
        nFileOffset: 0,
        nFileExtension: 0,
        lpstrDefExt: null_mut(),
        lCustData: 0,
        lpfnHook: None,
        lpTemplateName: null_mut(),
        pvReserved: 0 as LPVOID,
        dwReserved: 0,
        FlagsEx: 0,
    };

    unsafe { GetOpenFileNameW(&mut openfilenamew); }
}
