/* automatically generated by rust-bindgen */

pub type wchar_t = libc::c_int;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct div_t {
    pub quot: libc::c_int,
    pub rem: libc::c_int,
}

impl Clone for div_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ldiv_t {
    pub quot: libc::c_long,
    pub rem: libc::c_long,
}
impl Clone for ldiv_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[no_mangle]
pub extern "C" fn abort() {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn abs(arg1: libc::c_int) -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn atexit(arg1: ::std::option::Option<unsafe extern "C" fn()>) -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn atof(arg1: *const libc::c_char) -> f64 {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn atoi(arg1: *const libc::c_char) -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn atol(arg1: *const libc::c_char) -> libc::c_long {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn bsearch(
    arg1: *const libc::c_void,
    arg2: *const libc::c_void,
    arg3: usize,
    arg4: usize,
    arg5: ::std::option::Option<
        unsafe extern "C" fn(arg1: *const libc::c_void,
                             arg2: *const libc::c_void)
                             -> libc::c_int,
    >,
) -> *mut libc::c_void {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn calloc(arg1: usize, arg2: usize) -> *mut libc::c_void {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn div(arg1: libc::c_int, arg2: libc::c_int) -> div_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn drand48() -> f64 {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn ecvt(
    arg1: f64,
    arg2: libc::c_int,
    arg3: *mut libc::c_int,
    arg4: *mut libc::c_int,
) -> *mut libc::c_char {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn erand48(arg1: *mut libc::c_ushort) -> f64 {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn exit(arg1: libc::c_int) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn fcvt(
    arg1: f64,
    arg2: libc::c_int,
    arg3: *mut libc::c_int,
    arg4: *mut libc::c_int,
) -> *mut libc::c_char {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn free(arg1: *mut libc::c_void) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn gcvt(arg1: f64, arg2: libc::c_int, arg3: *mut libc::c_char) -> *mut libc::c_char {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn getenv(arg1: *const libc::c_char) -> *mut libc::c_char {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn getsubopt(
    arg1: *mut *mut libc::c_char,
    arg2: *const *const libc::c_char,
    arg3: *mut *mut libc::c_char,
) -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn grantpt(arg1: libc::c_int) -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn initstate(
    arg1: libc::c_uint,
    arg2: *mut libc::c_char,
    arg3: usize,
) -> *mut libc::c_char {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn jrand48(arg1: *mut libc::c_ushort) -> libc::c_long {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn l64a(arg1: libc::c_long) -> *mut libc::c_char {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn labs(arg1: libc::c_long) -> libc::c_long {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn lcong48(arg1: *mut libc::c_ushort) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn ldiv(arg1: libc::c_long, arg2: libc::c_long) -> ldiv_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn lrand48() -> libc::c_long {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn malloc(arg1: usize) -> *mut libc::c_void {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn mblen(arg1: *const libc::c_char, arg2: usize) -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn mbstowcs(arg1: *mut wchar_t, arg2: *const libc::c_char, arg3: usize) -> usize {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn mbtowc(
    arg1: *mut wchar_t,
    arg2: *const libc::c_char,
    arg3: usize,
) -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn mktemp(arg1: *mut libc::c_char) -> *mut libc::c_char {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn mkstemp(arg1: *mut libc::c_char) -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn mrand48() -> libc::c_long {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn nrand48(arg1: *mut libc::c_ushort) -> libc::c_long {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn ptsname(arg1: libc::c_int) -> *mut libc::c_char {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn putenv(arg1: *mut libc::c_char) -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn qsort(
    arg1: *mut libc::c_void,
    arg2: usize,
    arg3: usize,
    arg4: ::std::option::Option<
        unsafe extern "C" fn(arg1: *const libc::c_void,
                             arg2: *const libc::c_void)
                             -> libc::c_int,
    >,
) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn rand() -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn rand_r(arg1: *mut libc::c_uint) -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn random() -> libc::c_long {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn realloc(arg1: *mut libc::c_void, arg2: usize) -> *mut libc::c_void {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn realpath(
    arg1: *const libc::c_char,
    arg2: *mut libc::c_char,
) -> *mut libc::c_char {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn seed48(arg1: *mut libc::c_ushort) -> libc::c_ushort {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn setkey(arg1: *const libc::c_char) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn setstate(arg1: *const libc::c_char) -> *mut libc::c_char {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn srand(arg1: libc::c_uint) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn srand48(arg1: libc::c_long) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn srandom(arg1: libc::c_uint) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn strtod(arg1: *const libc::c_char, arg2: *mut *mut libc::c_char) -> f64 {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn strtol(
    arg1: *const libc::c_char,
    arg2: *mut *mut libc::c_char,
    arg3: libc::c_int,
) -> libc::c_long {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn strtoul(
    arg1: *const libc::c_char,
    arg2: *mut *mut libc::c_char,
    arg3: libc::c_int,
) -> libc::c_ulong {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn system(arg1: *const libc::c_char) -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn ttyslot() -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn unlockpt(arg1: libc::c_int) -> libc::c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn valloc(arg1: usize) -> *mut libc::c_void {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wcstombs(arg1: *mut libc::c_char, arg2: *const wchar_t, arg3: usize) -> usize {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wctomb(arg1: *mut libc::c_char, arg2: wchar_t) -> libc::c_int {
    unimplemented!();
}
