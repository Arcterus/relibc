/* automatically generated by rust-bindgen */
#[no_mangle]
pub extern "C" fn btowc(arg1: libc::c_int) -> wint_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn fwprintf(arg1: *mut FILE, arg2: *const wchar_t, ...)
     -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn fwscanf(arg1: *mut FILE, arg2: *const wchar_t, ...)
     -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn fgetwc(arg1: *mut FILE) -> wint_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn fgetws(arg1: *mut wchar_t, arg2: libc::c_int,
                  arg3: *mut FILE) -> *mut wchar_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn fputwc(arg1: wchar_t, arg2: *mut FILE) -> wint_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn fputws(arg1: *const wchar_t, arg2: *mut FILE)
     -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn fwide(arg1: *mut FILE, arg2: libc::c_int)
     -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn getwc(arg1: *mut FILE) -> wint_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn getwchar() -> wint_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn mbsinit(arg1: *const mbstate_t) -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn mbrlen(arg1: *const libc::c_char, arg2: usize,
                  arg3: *mut mbstate_t) -> usize {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn mbrtowc(arg1: *mut wchar_t, arg2: *const libc::c_char,
                   arg3: usize, arg4: *mut mbstate_t) -> usize {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn mbsrtowcs(arg1: *mut wchar_t,
                     arg2: *mut *const libc::c_char, arg3: usize,
                     arg4: *mut mbstate_t) -> usize {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn putwc(arg1: wchar_t, arg2: *mut FILE) -> wint_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn putwchar(arg1: wchar_t) -> wint_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn swprintf(arg1: *mut wchar_t, arg2: usize,
                    arg3: *const wchar_t, ...) -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn swscanf(arg1: *const wchar_t, arg2: *const wchar_t, ...)
     -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn ungetwc(arg1: wint_t, arg2: *mut FILE) -> wint_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn vfwprintf(arg1: *mut FILE, arg2: *const wchar_t,
                     arg3: *mut __va_list_tag) -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn vwprintf(arg1: *const wchar_t, arg2: *mut __va_list_tag)
     -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn vswprintf(arg1: *mut wchar_t, arg2: usize, arg3: *const wchar_t,
                     arg4: *mut __va_list_tag) -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcrtomb(arg1: *mut libc::c_char, arg2: wchar_t,
                   arg3: *mut mbstate_t) -> usize {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcscat(arg1: *mut wchar_t, arg2: *const wchar_t) -> *mut wchar_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcschr(arg1: *const wchar_t, arg2: wchar_t)
     -> *mut libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcscmp(arg1: *const wchar_t, arg2: *const wchar_t)
     -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcscoll(arg1: *const wchar_t, arg2: *const wchar_t)
     -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcscpy(arg1: *mut wchar_t, arg2: *const wchar_t) -> *mut wchar_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcscspn(arg1: *const wchar_t, arg2: *const wchar_t) -> usize {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcsftime(arg1: *mut wchar_t, arg2: usize, arg3: *const wchar_t,
                    arg4: *mut tm) -> usize {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcslen(arg1: *const wchar_t) -> libc::c_ulong {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcsncat(arg1: *mut wchar_t, arg2: *const wchar_t, arg3: usize)
     -> *mut wchar_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcsncmp(arg1: *const wchar_t, arg2: *const wchar_t, arg3: usize)
     -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcsncpy(arg1: *mut wchar_t, arg2: *const wchar_t, arg3: usize)
     -> *mut wchar_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcspbrk(arg1: *const wchar_t, arg2: *const wchar_t)
     -> *mut wchar_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcsrchr(arg1: *const wchar_t, arg2: wchar_t) -> *mut wchar_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcsrtombs(arg1: *mut libc::c_char,
                     arg2: *mut *const wchar_t, arg3: usize,
                     arg4: *mut mbstate_t) -> usize {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcsspn(arg1: *const wchar_t, arg2: *const wchar_t) -> usize {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcsstr(arg1: *const wchar_t, arg2: *const wchar_t) -> *mut wchar_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcstod(arg1: *const wchar_t, arg2: *mut *mut wchar_t) -> f64 {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcstok(arg1: *mut wchar_t, arg2: *const wchar_t,
                  arg3: *mut *mut wchar_t) -> *mut wchar_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcstol(arg1: *const wchar_t, arg2: *mut *mut wchar_t,
                  arg3: libc::c_int) -> libc::c_long {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcstoul(arg1: *const wchar_t, arg2: *mut *mut wchar_t,
                   arg3: libc::c_int) -> libc::c_ulong {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcswcs(arg1: *const wchar_t, arg2: *const wchar_t) -> *mut wchar_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcswidth(arg1: *const wchar_t, arg2: usize)
     -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcsxfrm(arg1: *mut wchar_t, arg2: *const wchar_t, arg3: usize)
     -> usize {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wctob(arg1: wint_t) -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcwidth(arg1: wchar_t) -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wmemchr(arg1: *const wchar_t, arg2: wchar_t, arg3: usize)
     -> *mut libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wmemcmp(arg1: *const wchar_t, arg2: *const wchar_t, arg3: usize)
     -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wmemcpy(arg1: *mut wchar_t, arg2: *const wchar_t, arg3: usize)
     -> *mut wchar_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wmemmove(arg1: *mut wchar_t, arg2: *const wchar_t, arg3: usize)
     -> *mut wchar_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wmemset(arg1: *mut wchar_t, arg2: wchar_t, arg3: usize)
     -> *mut wchar_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wprintf(arg1: *const wchar_t, ...) -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wscanf(arg1: *const wchar_t, ...) -> libc::c_int {
    unimplemented!();
}

