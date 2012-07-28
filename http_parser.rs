/* automatically generated by rust-bindgen */

import libc::*;

type __u_char = c_uchar;

type __u_short = c_ushort;

type __u_int = c_uint;

type __u_long = c_ulong;

type __int8_t = c_char;

type __uint8_t = c_uchar;

type __int16_t = c_short;

type __uint16_t = c_ushort;

type __int32_t = c_int;

type __uint32_t = c_uint;

type __int64_t = c_long;

type __uint64_t = c_ulong;

type __quad_t = c_long;

type __u_quad_t = c_ulong;

type __dev_t = c_ulong;

type __uid_t = c_uint;

type __gid_t = c_uint;

type __ino_t = c_ulong;

type __ino64_t = c_ulong;

type __mode_t = c_uint;

type __nlink_t = c_ulong;

type __off_t = c_long;

type __off64_t = c_long;

type __pid_t = c_int;

type __fsid_t = {
    __val: (c_int,c_int),
};

type __clock_t = c_long;

type __rlim_t = c_ulong;

type __rlim64_t = c_ulong;

type __id_t = c_uint;

type __time_t = c_long;

type __useconds_t = c_uint;

type __suseconds_t = c_long;

type __daddr_t = c_int;

type __swblk_t = c_long;

type __key_t = c_int;

type __clockid_t = c_int;

type __timer_t = *c_void;

type __blksize_t = c_long;

type __blkcnt_t = c_long;

type __blkcnt64_t = c_long;

type __fsblkcnt_t = c_ulong;

type __fsblkcnt64_t = c_ulong;

type __fsfilcnt_t = c_ulong;

type __fsfilcnt64_t = c_ulong;

type __ssize_t = c_long;

type __loff_t = __off64_t;

type __qaddr_t = *__quad_t;

type __caddr_t = *c_char;

type __intptr_t = c_long;

type __socklen_t = c_uint;

type u_char = __u_char;

type u_short = __u_short;

type u_int = __u_int;

type u_long = __u_long;

type quad_t = __quad_t;

type u_quad_t = __u_quad_t;

type fsid_t = __fsid_t;

type loff_t = __loff_t;

type ino_t = __ino_t;

type dev_t = __dev_t;

type gid_t = __gid_t;

type mode_t = __mode_t;

type nlink_t = __nlink_t;

type uid_t = __uid_t;

type off_t = __off_t;

type pid_t = __pid_t;

type id_t = __id_t;

type ssize_t = __ssize_t;

type daddr_t = __daddr_t;

type caddr_t = __caddr_t;

type key_t = __key_t;

type clock_t = __clock_t;

type time_t = __time_t;

type clockid_t = __clockid_t;

type timer_t = __timer_t;

type size_t = c_ulong;

type ulong = c_ulong;

type ushort = c_ushort;

type uint = c_uint;

type int8_t = c_char;

type int16_t = c_short;

type int32_t = c_int;

type int64_t = c_long;

type u_int8_t = c_uchar;

type u_int16_t = c_ushort;

type u_int32_t = c_uint;

type u_int64_t = c_ulong;

type register_t = c_long;

type __sig_atomic_t = c_int;

type __sigset_t = {
    __val: (c_ulong,c_ulong,c_ulong,c_ulong,c_ulong,c_ulong,c_ulong,c_ulong,c_ulong,c_ulong,c_ulong,c_ulong,c_ulong,c_ulong,c_ulong,c_ulong),
};

type sigset_t = __sigset_t;

type struct_timespec = {
    tv_sec: __time_t,
    tv_nsec: c_long,
};

type struct_timeval = {
    tv_sec: __time_t,
    tv_usec: __suseconds_t,
};

type suseconds_t = __suseconds_t;

type __fd_mask = c_long;

type fd_set = {
    __fds_bits: (__fd_mask,__fd_mask,__fd_mask,__fd_mask,__fd_mask,__fd_mask,__fd_mask,__fd_mask,__fd_mask,__fd_mask,__fd_mask,__fd_mask,__fd_mask,__fd_mask,__fd_mask,__fd_mask),
};

type fd_mask = __fd_mask;

type blksize_t = __blksize_t;

type blkcnt_t = __blkcnt_t;

type fsblkcnt_t = __fsblkcnt_t;

type fsfilcnt_t = __fsfilcnt_t;

type pthread_t = c_ulong;

type pthread_attr_t = c_void /* FIXME: union type */;

type struct___pthread_internal_list = {
    __prev: *c_void /* struct___pthread_internal_list */,
    __next: *c_void /* struct___pthread_internal_list */,
};

type __pthread_list_t = struct___pthread_internal_list;

type struct___pthread_mutex_s = {
    __lock: c_int,
    __count: c_uint,
    __owner: c_int,
    __nusers: c_uint,
    __kind: c_int,
    __spins: c_int,
    __list: __pthread_list_t,
};

type pthread_mutex_t = c_void /* FIXME: union type */;

type pthread_mutexattr_t = c_void /* FIXME: union type */;

type pthread_cond_t = c_void /* FIXME: union type */;

type pthread_condattr_t = c_void /* FIXME: union type */;

type pthread_key_t = c_uint;

type pthread_once_t = c_int;

type pthread_rwlock_t = c_void /* FIXME: union type */;

type pthread_rwlockattr_t = c_void /* FIXME: union type */;

type pthread_spinlock_t = c_int;

type pthread_barrier_t = c_void /* FIXME: union type */;

type pthread_barrierattr_t = c_void /* FIXME: union type */;

type uint8_t = c_uchar;

type uint16_t = c_ushort;

type uint32_t = c_uint;

type uint64_t = c_ulong;

type int_least8_t = c_char;

type int_least16_t = c_short;

type int_least32_t = c_int;

type int_least64_t = c_long;

type uint_least8_t = c_uchar;

type uint_least16_t = c_ushort;

type uint_least32_t = c_uint;

type uint_least64_t = c_ulong;

type int_fast8_t = c_char;

type int_fast16_t = c_long;

type int_fast32_t = c_long;

type int_fast64_t = c_long;

type uint_fast8_t = c_uchar;

type uint_fast16_t = c_ulong;

type uint_fast32_t = c_ulong;

type uint_fast64_t = c_ulong;

type intptr_t = c_long;

type uintptr_t = c_ulong;

type intmax_t = c_long;

type uintmax_t = c_ulong;

type http_parser = struct_http_parser;

type http_parser_settings = struct_http_parser_settings;

type http_data_cb = *u8;

type http_cb = *u8;

type enum_http_method = c_uint;
const HTTP_DELETE: u32 = 0_u32;
const HTTP_GET: u32 = 1_u32;
const HTTP_HEAD: u32 = 2_u32;
const HTTP_POST: u32 = 3_u32;
const HTTP_PUT: u32 = 4_u32;
const HTTP_CONNECT: u32 = 5_u32;
const HTTP_OPTIONS: u32 = 6_u32;
const HTTP_TRACE: u32 = 7_u32;
const HTTP_COPY: u32 = 8_u32;
const HTTP_LOCK: u32 = 9_u32;
const HTTP_MKCOL: u32 = 10_u32;
const HTTP_MOVE: u32 = 11_u32;
const HTTP_PROPFIND: u32 = 12_u32;
const HTTP_PROPPATCH: u32 = 13_u32;
const HTTP_SEARCH: u32 = 14_u32;
const HTTP_UNLOCK: u32 = 15_u32;
const HTTP_REPORT: u32 = 16_u32;
const HTTP_MKACTIVITY: u32 = 17_u32;
const HTTP_CHECKOUT: u32 = 18_u32;
const HTTP_MERGE: u32 = 19_u32;
const HTTP_MSEARCH: u32 = 20_u32;
const HTTP_NOTIFY: u32 = 21_u32;
const HTTP_SUBSCRIBE: u32 = 22_u32;
const HTTP_UNSUBSCRIBE: u32 = 23_u32;
const HTTP_PATCH: u32 = 24_u32;
const HTTP_PURGE: u32 = 25_u32;

type enum_http_parser_type = c_uint;
const HTTP_REQUEST: u32 = 0_u32;
const HTTP_RESPONSE: u32 = 1_u32;
const HTTP_BOTH: u32 = 2_u32;

type enum_flags = c_uint;
const F_CHUNKED: u32 = 1_u32;
const F_CONNECTION_KEEP_ALIVE: u32 = 2_u32;
const F_CONNECTION_CLOSE: u32 = 4_u32;
const F_TRAILING: u32 = 8_u32;
const F_UPGRADE: u32 = 16_u32;
const F_SKIPBODY: u32 = 32_u32;

type enum_http_errno = c_uint;
const HPE_OK: u32 = 0_u32;
const HPE_CB_message_begin: u32 = 1_u32;
const HPE_CB_url: u32 = 2_u32;
const HPE_CB_header_field: u32 = 3_u32;
const HPE_CB_header_value: u32 = 4_u32;
const HPE_CB_headers_complete: u32 = 5_u32;
const HPE_CB_body: u32 = 6_u32;
const HPE_CB_message_complete: u32 = 7_u32;
const HPE_INVALID_EOF_STATE: u32 = 8_u32;
const HPE_HEADER_OVERFLOW: u32 = 9_u32;
const HPE_CLOSED_CONNECTION: u32 = 10_u32;
const HPE_INVALID_VERSION: u32 = 11_u32;
const HPE_INVALID_STATUS: u32 = 12_u32;
const HPE_INVALID_METHOD: u32 = 13_u32;
const HPE_INVALID_URL: u32 = 14_u32;
const HPE_INVALID_HOST: u32 = 15_u32;
const HPE_INVALID_PORT: u32 = 16_u32;
const HPE_INVALID_PATH: u32 = 17_u32;
const HPE_INVALID_QUERY_STRING: u32 = 18_u32;
const HPE_INVALID_FRAGMENT: u32 = 19_u32;
const HPE_LF_EXPECTED: u32 = 20_u32;
const HPE_INVALID_HEADER_TOKEN: u32 = 21_u32;
const HPE_INVALID_CONTENT_LENGTH: u32 = 22_u32;
const HPE_INVALID_CHUNK_SIZE: u32 = 23_u32;
const HPE_INVALID_CONSTANT: u32 = 24_u32;
const HPE_INVALID_INTERNAL_STATE: u32 = 25_u32;
const HPE_STRICT: u32 = 26_u32;
const HPE_PAUSED: u32 = 27_u32;
const HPE_UNKNOWN: u32 = 28_u32;

type struct_http_parser = {
    _type_flags: c_uchar,
    state: c_uchar,
    header_state: c_uchar,
    index: c_uchar,
    nread: uint32_t,
    content_length: uint64_t,
    http_major: c_ushort,
    http_minor: c_ushort,
    status_code: c_ushort,
    method: c_uchar,
    http_errno_upgrade: c_uchar,
    data: *c_void,
};

type struct_http_parser_settings = {
    on_message_begin: http_cb,
    on_url: http_data_cb,
    on_header_field: http_data_cb,
    on_header_value: http_data_cb,
    on_headers_complete: http_cb,
    on_body: http_data_cb,
    on_message_complete: http_cb,
};

type enum_http_parser_url_fields = c_uint;
const UF_SCHEMA: u32 = 0_u32;
const UF_HOST: u32 = 1_u32;
const UF_PORT: u32 = 2_u32;
const UF_PATH: u32 = 3_u32;
const UF_QUERY: u32 = 4_u32;
const UF_FRAGMENT: u32 = 5_u32;
const UF_MAX: u32 = 6_u32;

type struct_http_parser_url = {
    field_set: uint16_t,
    port: uint16_t,
    field_data: (struct_unnamed1,struct_unnamed1,struct_unnamed1,struct_unnamed1,struct_unnamed1,struct_unnamed1),
};

type struct_unnamed2 = {
    __lock: c_int,
    __nr_readers: c_uint,
    __readers_wakeup: c_uint,
    __writer_wakeup: c_uint,
    __nr_readers_queued: c_uint,
    __nr_writers_queued: c_uint,
    __writer: c_int,
    __shared: c_int,
    __pad1: c_ulong,
    __pad2: c_ulong,
    __flags: c_uint,
};

type struct_unnamed3 = {
    __lock: c_int,
    __futex: c_uint,
    __total_seq: c_ulonglong,
    __wakeup_seq: c_ulonglong,
    __woken_seq: c_ulonglong,
    __mutex: *c_void,
    __nwaiters: c_uint,
    __broadcast_seq: c_uint,
};

type struct_unnamed1 = {
    off: uint16_t,
    len: uint16_t,
};

#[nolink]
#[link_args = "-L. -lhttp_parser"]
extern mod bindgen {

fn select(++__nfds: c_int, ++__readfds: *fd_set, ++__writefds: *fd_set, ++__exceptfds: *fd_set, ++__timeout: *struct_timeval) -> c_int;

fn pselect(++__nfds: c_int, ++__readfds: *fd_set, ++__writefds: *fd_set, ++__exceptfds: *fd_set, ++__timeout: *struct_timespec, ++__sigmask: *__sigset_t) -> c_int;

fn gnu_dev_major(++__dev: c_ulonglong) -> c_uint;

fn gnu_dev_minor(++__dev: c_ulonglong) -> c_uint;

fn gnu_dev_makedev(++__major: c_uint, ++__minor: c_uint) -> c_ulonglong;

fn http_parser_init(++parser: *http_parser, ++_type: enum_http_parser_type);

fn http_parser_execute(++parser: *http_parser, ++settings: *http_parser_settings, ++data: *c_char, ++len: size_t) -> size_t;

fn http_should_keep_alive(++parser: *http_parser) -> c_int;

fn http_method_str(++m: enum_http_method) -> *c_char;

fn http_errno_name(++err: enum_http_errno) -> *c_char;

fn http_errno_description(++err: enum_http_errno) -> *c_char;

fn http_parser_parse_url(++buf: *c_char, ++buflen: size_t, ++is_connect: c_int, ++u: *struct_http_parser_url) -> c_int;

fn http_parser_pause(++parser: *http_parser, ++paused: c_int);

}