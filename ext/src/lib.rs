use libc::{c_int, c_void, c_char, uintptr_t};
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;
use tokio::io::AsyncWriteExt;
use std::os::unix::io::AsRawFd;
use std::os::unix::net::UnixStream;
use std::sync::atomic::{AtomicI32, Ordering};
use std::ffi::{CStr, CString};
use std::borrow::Cow;
use std::io::Write;
use std::ptr;
use std::time::Duration;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{Zval, ZendHashTable};
use ext_php_rs::convert::FromZval;
use ext_php_rs::binary::Binary;
use colored::*;

pub mod tgcalls;
pub mod helper;

static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    Runtime::new().expect("Failed to create Tokio runtime")
});

static LOG_MASK: AtomicI32 = AtomicI32::new(0);

#[php_const]
pub const PHP_TGCALLS_VERSION: &str = "0.0.1";

#[php_function]
pub fn tgcalls_get_protocol() -> PhpResult<Zval> {
    unsafe {
        let mut proto = tgcalls::ntg_protocol_struct {
            min_layer: 0,
            max_layer: 0,
            udp_p2p: false,
            udp_reflector: false,
            library_versions: std::ptr::null_mut(),
            library_versions_size: 0,
        };

        let res = tgcalls::ntg_get_protocol(&mut proto);

        if res != 0 {
            return Err(PhpException::default(format!("Failed to get protocol : {}", res)));
        }

        let mut arr = ZendHashTable::new();

        arr.insert("min_layer", proto.min_layer).ok();
        arr.insert("max_layer", proto.max_layer).ok();
        arr.insert("udp_p2p", proto.udp_p2p).ok();
        arr.insert("udp_reflector", proto.udp_reflector).ok();
        arr.insert("library_versions", proto.get_library_versions()).ok();

        let mut zval = Zval::new();

        zval.set_hashtable(arr);

        Ok(zval)
    }
}

#[php_function]
pub fn tgcalls_get_media_devices() -> PhpResult<Zval> {
    unsafe {
        let mut devices = tgcalls::ntg_media_devices_struct {
            microphone: std::ptr::null_mut(),
            size_microphone: 0,
            speaker: std::ptr::null_mut(),
            size_speaker: 0,
            camera: std::ptr::null_mut(),
            size_camera: 0,
            screen: std::ptr::null_mut(),
            size_screen: 0,
        };

        let res = tgcalls::ntg_get_media_devices(&mut devices);

        if res != 0 {
            return Err(PhpException::default(format!("Failed : {}", res)));
        }

        let convert_list = |ptr: *mut tgcalls::ntg_device_info_struct, size: i32| -> Zval {
            let mut list = ZendHashTable::new();

            if !ptr.is_null() && size > 0 {
                for i in 0..size as isize {
                    let device_struct = (*ptr.offset(i)).clone();

                    let mut props = ZendHashTable::new();

                    props.insert("name", device_struct.get_name()).ok();
                    props.insert("metadata", device_struct.get_metadata()).ok();

                    let mut device = Zval::new();

                    device.set_hashtable(props);

                    list.push(device).ok();
                }
            }
            let mut devices = Zval::new();

            devices.set_hashtable(list);

            devices
        };

        let mut arr = ZendHashTable::new();

        arr.insert("microphone", convert_list(devices.microphone, devices.size_microphone)).ok();
        arr.insert("speaker", convert_list(devices.speaker, devices.size_speaker)).ok();
        arr.insert("camera", convert_list(devices.camera, devices.size_camera)).ok();
        arr.insert("screen", convert_list(devices.screen, devices.size_screen)).ok();

        let mut zval = Zval::new();

        zval.set_hashtable(arr);

        Ok(zval)
    }
}

#[php_function]
pub fn tgcalls_get_version() -> PhpResult<String> {
    unsafe {
        let mut buffer: *mut c_char = std::ptr::null_mut();

        let res = tgcalls::ntg_get_version(&mut buffer);

        if res != 0 || buffer.is_null() {
            return Err(PhpException::default("Failed to get version string".to_string()));
        }

        Ok(CStr::from_ptr(buffer).to_string_lossy().into_owned())
    }
}

#[php_function]
pub fn tgcalls_enable_glib_loop(enable: bool) -> bool {
    unsafe {
        tgcalls::ntg_enable_g_lib_loop(enable) == 0
    }
}

unsafe extern "C" fn log_handler(log_message: tgcalls::ntg_log_message_struct) {

    let mask = LOG_MASK.load(Ordering::Relaxed);

    if mask == 0 || (log_message.level as i32 & mask) == 0 {
        return;
    }

    let message = unsafe {
        if !log_message.message.is_null() {
            CStr::from_ptr(log_message.message).to_string_lossy()
        } else {
            Cow::Borrowed("No message")
        }
    };

    let file = unsafe {
        if !log_message.file.is_null() {
            CStr::from_ptr(log_message.file).to_string_lossy()
        } else {
            Cow::Borrowed("unknown")
        }
    };
    let level = match log_message.level {
        tgcalls::ntg_log_level_enum::NTG_LOG_ERROR => "ERROR".red().bold(),
        tgcalls::ntg_log_level_enum::NTG_LOG_WARNING => "WARN".yellow().bold(),
        tgcalls::ntg_log_level_enum::NTG_LOG_INFO => "INFO".green(),
        tgcalls::ntg_log_level_enum::NTG_LOG_DEBUG => "DEBUG".blue(),
        _ => "UNKNOWN".purple(),
    };

    let log_line = format!(
        "[TgCalls {}] {}:{} | {}\n",
        level,
        file,
        log_message.line,
        message
    );

    #[allow(unused_must_use)] std::io::stderr().write_all(log_line.as_bytes());
}

#[php_function]
pub fn tgcalls_set_log_level(mask: i32) {
    LOG_MASK.store(mask, Ordering::Relaxed);
    if mask != 0 {
        unsafe {
            tgcalls::ntg_register_logger(Some(log_handler));
        }
    }
}

pub trait AsyncCommandHandler {
    fn get_handle(&self) -> uintptr_t;

    fn execute_async(
        &self,
        chat_id: i64,
        func: unsafe extern "C" fn(uintptr_t, i64, tgcalls::ntg_async_struct) -> i32,
        timeout: u64
    ) -> PhpResult<bool> {
        unsafe {
            let mut f = helper::NtgFuture::new();

            let res = func(self.get_handle(), chat_id, f.as_async_struct());

            if res != 0 {
                return Err(PhpException::default(format!("C Command failed : {}", res)));
            }

            match f.wait_timeout(std::time::Duration::from_secs(timeout)) {
                Ok(0) => Ok(true),
                Ok(code) => Err(PhpException::default(format!("Async error : {}", code))),
                Err(e) => Err(PhpException::default(format!("Timeout : {}", e))),
            }
        }
    }
}

impl AsyncCommandHandler for TgcallsClient {
    fn get_handle(&self) -> uintptr_t {
        self.client_handle
    }
}

#[derive(Clone)]
#[php_class]
pub struct TgcallsClient {
    client_handle: uintptr_t,
    peer_id: i64,
}

#[php_impl]
impl TgcallsClient {

    pub fn __construct(peer_id: i64) -> Self {
        unsafe {
            let handle = tgcalls::ntg_init();
            TgcallsClient {
                client_handle: handle,
                peer_id: peer_id,
            }
        }
    }

    pub fn __destruct(&self) {
        unsafe {
            if self.client_handle != 0 {
                tgcalls::ntg_destroy(self.client_handle);
            }
        }
    }

    #[php(defaults(p2p = false,timeout = 10u64))]
    pub fn create(&self, p2p: bool, timeout: u64) -> PhpResult<String> {
        unsafe {
            let mut f = helper::NtgFuture::new();

            let mut out_buffer: *mut c_char = ptr::null_mut();

            let create_res = if p2p {
                tgcalls::ntg_create_p2p(
                    self.client_handle,
                    self.peer_id,
                    f.as_async_struct()
                )
            } else {
                tgcalls::ntg_create(
                    self.client_handle,
                    self.peer_id,
                    &mut out_buffer,
                    f.as_async_struct()
                )
            };

            if create_res != 0 {
                return Err(PhpException::default(format!("ntg_create failed with code : {}", create_res)));
            }

            match f.wait_timeout(Duration::from_secs(timeout)) {
                Ok(0) => {
                    if !out_buffer.is_null() {
                        Ok(CStr::from_ptr(out_buffer).to_string_lossy().into_owned())
                    } else {
                        Ok(String::new())
                    }
                },
                Ok(code) => return Err(PhpException::default(format!("Async create failed with error code : {}", code))),
                Err(e) => return Err(PhpException::default(format!("Timeout in create : {}", e))),
            }
        }
    }

    #[php(change_case = "snake_case")]
    #[php(defaults(timeout = 10u64))]
    pub fn set_stream(&self, stream_mode: tgcalls::ntg_stream_mode_enum, desc: &tgcalls::ntg_media_description_struct, timeout: u64) -> PhpResult<bool> {
        unsafe {
            let mut f_source = helper::NtgFuture::new();

            let source_res = tgcalls::ntg_set_stream_sources(
                self.client_handle,
                self.peer_id,
                stream_mode,
                *desc,
                f_source.as_async_struct()
            );

            if source_res != 0 {
                return Err(PhpException::default(format!("ntg_set_stream_sources failed : {}", source_res)));
            }

            match f_source.wait_timeout(Duration::from_secs(timeout)) {
                Ok(0) => Ok(true),
                Ok(code) => return Err(PhpException::default(format!("Async set_stream failed : {}", code))),
                Err(e) => return Err(PhpException::default(format!("Timeout in set_stream : {}", e))),
            }
        }
    }

    #[php(defaults(presentation = false,timeout = 10u64))]
    pub fn connect(&self, params_json: String, presentation: bool, timeout: u64) -> PhpResult<bool> {
        unsafe {
            let mut f = helper::NtgFuture::new();

            let c_connect_data = CString::new(params_json).map_err(|e| PhpException::default(e.to_string()))?;

            let connect_res = tgcalls::ntg_connect(
                self.client_handle,
                self.peer_id,
                c_connect_data.as_ptr() as *mut c_char,
                presentation,
                f.as_async_struct()
            );

            if connect_res != 0 {
                return Err(PhpException::default(format!("ntg_connect immediate fail : {}", connect_res)));
            }

            match f.wait_timeout(Duration::from_secs(timeout)) {
                Ok(0) => Ok(true),
                Ok(code) => Err(PhpException::default(format!("Async connect failed with code : {}", code))),
                Err(e) => Err(PhpException::default(format!("Timeout in connect : {}", e))),
            }
        }
    }

    #[php(change_case = "snake_case")]
    #[php(defaults(timeout = 10u64))]
    pub fn init_exchange(
        &self,
        dh_config: &tgcalls::ntg_dh_config_struct,
        g_a_hash: Option<Binary<u8>>,
        timeout: u64
    ) -> PhpResult<Binary<u8>> {
        unsafe {
            let mut f = helper::NtgFuture::new();

            let mut out_buffer: *mut u8 = std::ptr::null_mut();
            let mut out_size: i32 = 0;

            let (g_a_ptr, g_a_len) = if let Some(ref bin) = g_a_hash {
                (bin.as_ptr(), bin.len() as c_int)
            } else {
                (ptr::null(), 0)
            };

            let exchange_res = tgcalls::ntg_init_exchange(
                self.client_handle,
                self.peer_id,
                dh_config as *const _ as *mut _,
                g_a_ptr,
                g_a_len,
                &mut out_buffer,
                &mut out_size,
                f.as_async_struct()
            );

            if exchange_res != 0 {
                return Err(PhpException::default(format!("ntg_init_exchange failed with code : {}", exchange_res)));
            }

            match f.wait_timeout(Duration::from_secs(timeout)) {
                Ok(0) => {
                    if !out_buffer.is_null() && out_size > 0 {
                        let slice = std::slice::from_raw_parts(out_buffer, out_size as usize);
                        Ok(Binary::from(slice.to_vec()))
                    } else {
                        Ok(Binary::from(Vec::new()))
                    }
                },
                Ok(code) => return Err(PhpException::default(format!("Async init_exchange failed with error code : {}", code))),
                Err(e) => return Err(PhpException::default(format!("Timeout in init_exchange : {}", e))),
            }
        }
    }

    #[php(change_case = "snake_case")]
    #[php(defaults(timeout = 10u64))]
    pub fn exchange_keys(
        &self,
        g_a_or_b: Binary<u8>,
        fingerprint: Option<i64>,
        timeout: u64
    ) -> PhpResult<tgcalls::ntg_auth_params_struct> {
        unsafe {
            let mut f = helper::NtgFuture::new();

            let mut auth_params = tgcalls::ntg_auth_params_struct {
                g_a_or_b: std::ptr::null_mut(),
                size_g_a_b: 0,
                key_fingerprint: 0,
            };

            let fp: i64 = fingerprint.unwrap_or(0);

            let exchange_res = tgcalls::ntg_exchange_keys(
                self.client_handle,
                self.peer_id,
                g_a_or_b.as_ptr(),
                g_a_or_b.len() as i32,
                fp,
                &mut auth_params,
                f.as_async_struct()
            );

            if exchange_res != 0 {
                return Err(PhpException::default(format!("ntg_exchange_keys failed with code : {}", exchange_res)));
            }

            match f.wait_timeout(Duration::from_secs(timeout)) {
                Ok(0) => Ok(auth_params),
                Ok(code) => return Err(PhpException::default(format!("Async exchange_keys failed with error code : {}", code))),
                Err(e) => return Err(PhpException::default(format!("Timeout in exchange_keys : {}", e))),
            }
        }
    }

    #[php(change_case = "snake_case")]
    #[php(defaults(timeout = 10u64))]
    pub fn skip_exchange(
        &self,
        encryption_key: Binary<u8>,
        is_outgoing: bool,
        timeout: u64
    ) -> PhpResult<bool> {
        unsafe {
            let mut f = helper::NtgFuture::new();

            let exchange_res = tgcalls::ntg_skip_exchange(
                self.client_handle,
                self.peer_id,
                encryption_key.as_ptr(),
                encryption_key.len() as i32,
                is_outgoing,
                f.as_async_struct()
            );

            if exchange_res != 0 {
                return Err(PhpException::default(format!("ntg_skip_exchange failed with code : {}", exchange_res)));
            }

            match f.wait_timeout(Duration::from_secs(timeout)) {
                Ok(0) => Ok(true),
                Ok(code) => return Err(PhpException::default(format!("Async skip_exchange failed with error code : {}", code))),
                Err(e) => return Err(PhpException::default(format!("Timeout in skip_exchange : {}", e))),
            }
        }
    }

    #[php(change_case = "snake_case")]
    #[php(defaults(timeout = 10u64))]
    pub fn connect_peer(
        &self,
        servers: &Zval,
        protocol_versions: Vec<String>,
        p2p_allowed: bool,
        timeout: u64
    ) -> PhpResult<bool> {
        unsafe {
            let mut f = helper::NtgFuture::new();

            let mut c_servers: Vec<tgcalls::ntg_rtc_server_struct> = Vec::new();

            if let Some(hashtable) = servers.array() {
                for (_idx, val) in hashtable.iter() {
                    if let Some(server_ref) = <&tgcalls::ntg_rtc_server_struct>::from_zval(val) {
                        c_servers.push(*server_ref);
                    }
                }
            }

            let c_strings: Vec<*mut c_char> = protocol_versions.into_iter().map(|s| CString::new(s).unwrap().into_raw()).collect();
            let mut c_versions = c_strings.into_boxed_slice();

            let connect_res = tgcalls::ntg_connect_p2p(
                self.client_handle,
                self.peer_id,
                c_servers.as_mut_ptr(),
                c_servers.len() as i32,
                c_versions.as_mut_ptr(),
                c_versions.len() as i32,
                p2p_allowed,
                f.as_async_struct()
            );

            if connect_res != 0 {
                return Err(PhpException::default(format!("ntg_connect_p2p failed with code : {}", connect_res)));
            }

            match f.wait_timeout(Duration::from_secs(timeout)) {
                Ok(0) => Ok(true),
                Ok(code) => return Err(PhpException::default(format!("Async connect_p2p failed with error code : {}", code))),
                Err(e) => return Err(PhpException::default(format!("Timeout in connect_p2p : {}", e))),
            }
        }
    }

    #[php(change_case = "snake_case")]
    #[php(defaults(timeout = 10u64))]
    pub fn send_signaling_data(
        &self,
        mut data: Binary<u8>,
        timeout: u64
    ) -> PhpResult<bool> {
        unsafe {
            let mut f = helper::NtgFuture::new();

            let signaling_res = tgcalls::ntg_send_signaling_data(
                self.client_handle,
                self.peer_id,
                data.as_mut_ptr(),
                data.len() as i32,
                f.as_async_struct()
            );

            if signaling_res != 0 {
                return Err(PhpException::default(format!("ntg_send_signaling_data failed with code : {}", signaling_res)));
            }

            match f.wait_timeout(Duration::from_secs(timeout)) {
                Ok(0) => Ok(true),
                Ok(code) => return Err(PhpException::default(format!("Async send_signaling_data failed with error code : {}", code))),
                Err(e) => return Err(PhpException::default(format!("Timeout in send_signaling_data : {}", e))),
            }
        }
    }

    #[php(change_case = "snake_case")]
    #[php(defaults(timeout = 10u64))]
    pub fn init_presentation(&self, timeout: u64) -> PhpResult<Vec<String>> {
        unsafe {
            let mut f = helper::NtgFuture::new();

            let mut out_buffer: *mut c_char = std::ptr::null_mut();

            let presentation_res = tgcalls::ntg_init_presentation(
                self.client_handle,
                self.peer_id,
                &mut out_buffer,
                f.as_async_struct()
            );

            if presentation_res != 0 {
                return Err(PhpException::default(format!("ntg_init_presentation failed with code : {}", presentation_res)));
            }

            match f.wait_timeout(Duration::from_secs(timeout)) {
                Ok(0) => {
                    if !out_buffer.is_null() {
                        let s = CStr::from_ptr(out_buffer).to_string_lossy().into_owned();
                        Ok(vec![s]) 
                    } else {
                        Ok(Vec::new())
                    }
                },
                Ok(code) => Err(PhpException::default(format!("Async init_presentation failed with error code : {}", code))),
                Err(e) => Err(PhpException::default(format!("Timeout in init_presentation : {}", e))),
            }
        }
    }

    #[php(change_case = "snake_case")]
    #[php(defaults(timeout = 10u64))]
    pub fn add_incoming_video(
        &self,
        endpoint: String,
        ssrc_groups: &Zval,
        timeout: u64
    ) -> PhpResult<u32> {
        unsafe {
            let mut f = helper::NtgFuture::new();

            let c_endpoint = CString::new(endpoint).unwrap().into_raw();

            let mut c_ssrc_groups: Vec<tgcalls::ntg_ssrc_group_struct> = Vec::new();

            if let Some(arr) = ssrc_groups.array() {
                for (_, val) in arr.iter() {
                    if let Some(g_ref) = <&tgcalls::ntg_ssrc_group_struct>::from_zval(val) {
                        c_ssrc_groups.push(g_ref.clone());
                    }
                }
            }

            let mut out_buffer: u32 = 0;

            let add_res = tgcalls::ntg_add_incoming_video(
                self.client_handle,
                self.peer_id,
                c_endpoint,
                c_ssrc_groups.as_mut_ptr(),
                c_ssrc_groups.len() as i32,
                &mut out_buffer,
                f.as_async_struct()
            );

            if add_res != 0 {
                return Err(PhpException::default(format!("ntg_add_incoming_video failed with code : {}", add_res)));
            }

            match f.wait_timeout(Duration::from_secs(timeout)) {
                Ok(0) => Ok(out_buffer),
                Ok(code) => Err(PhpException::default(format!("Async add_incoming_video failed with error code : {}", code))),
                Err(e) => Err(PhpException::default(format!("Timeout in add_incoming_video : {}", e))),
            }
        }
    }

    #[php(change_case = "snake_case")]
    #[php(defaults(timeout = 10u64))]
    pub fn remove_incoming_video(&self, endpoint: String, timeout: u64) -> PhpResult<bool> {
        unsafe {
            let mut f = helper::NtgFuture::new();

            let c_endpoint = CString::new(endpoint).unwrap().into_raw();

            let remove_res = tgcalls::ntg_remove_incoming_video(
                self.client_handle,
                self.peer_id,
                c_endpoint,
                f.as_async_struct()
            );

            if remove_res != 0 {
                return Err(PhpException::default(format!("ntg_remove_incoming_video failed with code : {}", remove_res)));
            }

            match f.wait_timeout(Duration::from_secs(timeout)) {
                Ok(0) => Ok(true),
                Ok(code) => Err(PhpException::default(format!("Async remove_incoming_video failed with error code : {}", code))),
                Err(e) => Err(PhpException::default(format!("Timeout in remove_incoming_video : {}", e))),
            }
        }
    }

    #[php(change_case = "snake_case")]
    #[php(defaults(timeout = 10u64))]
    pub fn send_external_frame(
        &self,
        device: tgcalls::ntg_stream_device_enum,
        mut frame: Binary<u8>,
        frame_data: &tgcalls::ntg_frame_data_struct,
        timeout: u64
    ) -> PhpResult<bool> {
        unsafe {
            let mut f = helper::NtgFuture::new();

            let send_res = tgcalls::ntg_send_external_frame(
                self.client_handle,
                self.peer_id,
                device,
                frame.as_mut_ptr(),
                frame.len() as i32,
                *frame_data,
                f.as_async_struct()
            );

            if send_res != 0 {
                return Err(PhpException::default(format!("ntg_send_external_frame failed with code : {}", send_res)));
            }

            match f.wait_timeout(Duration::from_secs(timeout)) {
                Ok(0) => Ok(true),
                Ok(code) => Err(PhpException::default(format!("Async send_external_frame failed with error code : {}", code))),
                Err(e) => Err(PhpException::default(format!("Timeout in send_external_frame : {}", e))),
            }
        }
    }

    #[php(change_case = "snake_case")]
    #[php(defaults(timeout = 10u64))]
    pub fn send_broadcast_timestamp(&self, timestamp: i64, timeout: u64) -> PhpResult<bool> {
        unsafe {
            let mut f = helper::NtgFuture::new();

            let send_res = tgcalls::ntg_send_broadcast_timestamp(
                self.client_handle,
                self.peer_id,
                timestamp,
                f.as_async_struct()
            );

            if send_res != 0 {
                return Err(PhpException::default(format!("ntg_send_broadcast_timestamp failed with code : {}", send_res)));
            }

            match f.wait_timeout(Duration::from_secs(timeout)) {
                Ok(0) => Ok(true),
                Ok(code) => Err(PhpException::default(format!("Async send_broadcast_timestamp failed with error code : {}", code))),
                Err(e) => Err(PhpException::default(format!("Timeout in send_broadcast_timestamp : {}", e))),
            }
        }
    }

    #[php(change_case = "snake_case")]
    #[php(defaults(timeout = 10u64))]
    pub fn send_broadcast_part(
        &self,
        segment_id: i64,
        part_id: i32,
        status: tgcalls::ntg_media_segment_status_enum,
        quality_update: bool,
        frame: Binary<u8>,
        timeout: u64
    ) -> PhpResult<bool> {
        unsafe {
            let mut f = helper::NtgFuture::new();

            let res = tgcalls::ntg_send_broadcast_part(
                self.client_handle,
                self.peer_id,
                segment_id,
                part_id,
                status,
                quality_update,
                frame.as_ptr(),
                frame.len() as i32,
                f.as_async_struct()
            );

            if res != 0 {
                return Err(PhpException::default(format!("ntg_send_broadcast_part failed with code : {}", res)));
            }

            match f.wait_timeout(Duration::from_secs(timeout)) {
                Ok(0) => Ok(true),
                Ok(code) => Err(PhpException::default(format!("Async send_broadcast_part failed with error code : {}", code))),
                Err(e) => Err(PhpException::default(format!("Timeout in send_broadcast_part : {}", e))),
            }
        }
    }

    #[php(defaults(presentation = false,timeout = 10u64))]
    pub fn stop(&self, presentation: bool, timeout: u64) -> PhpResult<bool> {
        if presentation {
            self.execute_async(self.peer_id, tgcalls::ntg_stop_presentation, timeout)
        } else {
            self.execute_async(self.peer_id, tgcalls::ntg_stop, timeout)
        }
    }

    #[php(defaults(timeout = 10u64))]
    pub fn pause(&self, timeout: u64) -> PhpResult<bool> {
        self.execute_async(self.peer_id, tgcalls::ntg_pause, timeout)
    }

    #[php(defaults(timeout = 10u64))]
    pub fn resume(&self, timeout: u64) -> PhpResult<bool> {
        self.execute_async(self.peer_id, tgcalls::ntg_resume, timeout)
    }

    #[php(defaults(timeout = 10u64))]
    pub fn mute(&self, timeout: u64) -> PhpResult<bool> {
        self.execute_async(self.peer_id, tgcalls::ntg_mute, timeout)
    }

    #[php(defaults(timeout = 10u64))]
    pub fn unmute(&self, timeout: u64) -> PhpResult<bool> {
        self.execute_async(self.peer_id, tgcalls::ntg_unmute, timeout)
    }

    #[php(defaults(timeout = 10u64))]
    pub fn cpu(&self, timeout: u64) -> PhpResult<f64> {
        unsafe {
            let mut f = helper::NtgFuture::new();

            let mut cpu_val: f64 = 0.0;

            let cpu_res = tgcalls::ntg_cpu_usage(
                self.client_handle,
                &mut cpu_val,
                f.as_async_struct()
            );

            if cpu_res != 0 {
                return Err(PhpException::default(format!("ntg_cpu_usage failed with code : {}", cpu_res)));
            }

            match f.wait_timeout(std::time::Duration::from_secs(timeout)) {
                Ok(0) => Ok(cpu_val),
                Ok(code) => Err(PhpException::default(format!("Async cpu_usage failed with error code : {}", code))),
                Err(e) => Err(PhpException::default(format!("Timeout in cpu_usage : {}", e))),
            }
        }
    }

    #[php(change_case = "snake_case")]
    #[php(defaults(timeout = 10u64))]
    pub fn connection_mode(&self, timeout: u64) -> PhpResult<tgcalls::ntg_connection_mode_enum> {
        unsafe {
            let mut f = helper::NtgFuture::new();

            let mut out_mode = tgcalls::ntg_connection_mode_enum::NTG_CONNECTION_MODE_NONE;

            let mode_res = tgcalls::ntg_get_connection_mode(
                self.client_handle,
                self.peer_id,
                &mut out_mode,
                f.as_async_struct()
            );

            if mode_res != 0 {
                return Err(PhpException::default(format!("ntg_get_connection_mode failed with code : {}", mode_res)));
            }

            match f.wait_timeout(Duration::from_secs(timeout)) {
                Ok(0) => Ok(out_mode),
                Ok(code) => Err(PhpException::default(format!("Async get_connection_mode failed with error code : {}", code))),
                Err(e) => Err(PhpException::default(format!("Timeout in get_connection_mode : {}", e))),
            }
        }
    }

    #[php(defaults(timeout = 10u64))]
    pub fn calls(&self, timeout: u64) -> PhpResult<Vec<tgcalls::ntg_call_info_struct>> {
        unsafe {
            let mut f = helper::NtgFuture::new();

            let mut out_buffer: *mut tgcalls::ntg_call_info_struct = std::ptr::null_mut();

            let mut out_size: i32 = 0;

            let calls_res = tgcalls::ntg_calls(
                self.client_handle,
                &mut out_buffer,
                &mut out_size,
                f.as_async_struct()
            );

            if calls_res != 0 {
                return Err(PhpException::default(format!("ntg_calls failed with code : {}", calls_res)));
            }

            match f.wait_timeout(Duration::from_secs(timeout)) {
                Ok(0) => {
                    if !out_buffer.is_null() && out_size > 0 {
                        let slice = std::slice::from_raw_parts(out_buffer, out_size as usize);
                        Ok(slice.to_vec())
                    } else {
                        Ok(Vec::new())
                    }
                },
                Ok(code) => Err(PhpException::default(format!("Async get_calls failed with error code : {}", code))),
                Err(e) => Err(PhpException::default(format!("Timeout in get_calls : {}", e))),
            }
        }
    }

    #[php(defaults(timeout = 10u64))]
    pub fn time(&self, stream_mode: tgcalls::ntg_stream_mode_enum, timeout: u64) -> PhpResult<i64> {
        unsafe {
            let mut f = helper::NtgFuture::new();

            let mut out_time: i64 = 0;

            let time_res = tgcalls::ntg_time(
                self.client_handle,
                self.peer_id,
                stream_mode,
                &mut out_time,
                f.as_async_struct()
            );

            if time_res != 0 {
                return Err(PhpException::default(format!("ntg_time failed with code : {}", time_res)));
            }

            match f.wait_timeout(Duration::from_secs(timeout)) {
                Ok(0) => Ok(out_time),
                Ok(code) => Err(PhpException::default(format!("Async get_time failed with error code : {}", code))),
                Err(e) => Err(PhpException::default(format!("Timeout in get_time : {}", e))),
            }
        }
    }

    #[php(defaults(timeout = 10u64))]
    pub fn state(&self, timeout: u64) -> PhpResult<tgcalls::ntg_media_state_struct> {
        unsafe {
            let mut state = tgcalls::ntg_media_state_struct { 
                muted: false,
                video_paused: false,
                video_stopped: false,
                presentation_paused: false,
            };

            let mut f = helper::NtgFuture::new();

            let res = tgcalls::ntg_get_state(
                self.client_handle,
                self.peer_id,
                &mut state,
                f.as_async_struct()
            );

            if res != 0 {
                return Err(PhpException::default(format!("C call failed with code : {}", res)));
            }

            match f.wait_timeout(std::time::Duration::from_secs(timeout)) {
                Ok(0) => Ok(state),
                Ok(code) => return Err(PhpException::default(format!("Async error : {}", code))),
                Err(e) => return Err(PhpException::default(format!("Timeout : {}", e))),
            }
        }
    }
}

#[php_class]
pub struct TgcallsEvents {
    client: TgcallsClient,
    php_socket: UnixStream,
    rust_socket: tokio::sync::mpsc::UnboundedSender<helper::NtgEvent>,
}

#[php_impl]
impl TgcallsEvents {

    pub fn __construct(client: &TgcallsClient) -> Self {
        let _guard = RUNTIME.enter();

        let (rust_os_sock, php_os_sock) = UnixStream::pair().unwrap();

        rust_os_sock.set_nonblocking(true).unwrap();
        php_os_sock.set_nonblocking(true).unwrap();

        let rust_sock = tokio::net::UnixStream::from_std(rust_os_sock).unwrap();

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<helper::NtgEvent>();

        Self::spawn_event_worker(rust_sock, rx);

        Self {
            client: client.clone(),
            php_socket: php_os_sock,
            rust_socket: tx,
        }
    }

    #[php(change_case = "snake_case")]
    pub fn enable_stream_updates(&self) -> i32 {
        unsafe {
            tgcalls::ntg_on_stream_end(
                self.client.client_handle,
                Some(helper::proxy_on_stream),
                Box::into_raw(Box::new(self.rust_socket.clone())) as *mut c_void
            )
        }
    }

    #[php(change_case = "snake_case")]
    pub fn enable_upgrade_updates(&self) -> i32 {
        unsafe {
            tgcalls::ntg_on_upgrade(
                self.client.client_handle,
                Some(helper::proxy_on_upgrade),
                Box::into_raw(Box::new(self.rust_socket.clone())) as *mut c_void
            )
        }
    }

    #[php(change_case = "snake_case")]
    pub fn enable_connection_updates(&self) -> i32 {
        unsafe {
            tgcalls::ntg_on_connection_change(
                self.client.client_handle,
                Some(helper::proxy_on_connection),
                Box::into_raw(Box::new(self.rust_socket.clone())) as *mut c_void
            )
        }
    }

    #[php(change_case = "snake_case")]
    pub fn enable_signaling_updates(&self) -> i32 {
        unsafe {
            tgcalls::ntg_on_signaling_data(
                self.client.client_handle,
                Some(helper::proxy_on_signaling_data),
                Box::into_raw(Box::new(self.rust_socket.clone())) as *mut c_void
            )
        }
    }

    #[php(change_case = "snake_case")]
    pub fn enable_frame_updates(&self) -> i32 {
        unsafe {
            tgcalls::ntg_on_frames(
                self.client.client_handle,
                Some(helper::proxy_on_frame),
                Box::into_raw(Box::new(self.rust_socket.clone())) as *mut c_void
            )
        }
    }

    #[php(change_case = "snake_case")]
    pub fn enable_remote_source_updates(&self) -> i32 {
        unsafe {
            tgcalls::ntg_on_remote_source_change(
                self.client.client_handle,
                Some(helper::proxy_on_remote_source),
                Box::into_raw(Box::new(self.rust_socket.clone())) as *mut c_void
            )
        }
    }

    #[php(change_case = "snake_case")]
    pub fn enable_broadcast_timestamp_updates(&self) -> i32 {
        unsafe {
            tgcalls::ntg_on_request_broadcast_timestamp(
                self.client.client_handle,
                Some(helper::proxy_on_broadcast_timestamp),
                Box::into_raw(Box::new(self.rust_socket.clone())) as *mut c_void
            )
        }
    }

    #[php(change_case = "snake_case")]
    pub fn enable_broadcast_part_updates(&self) -> i32 {
        unsafe {
            tgcalls::ntg_on_request_broadcast_part(
                self.client.client_handle,
                Some(helper::proxy_on_broadcast_part),
                Box::into_raw(Box::new(self.rust_socket.clone())) as *mut c_void
            )
        }
    }

    #[php(change_case = "snake_case")]
    pub fn get_fd(&self) -> i32 {
        self.php_socket.as_raw_fd()
    }
}

impl TgcallsEvents {
    fn spawn_event_worker(mut socket: tokio::net::UnixStream, mut rx: tokio::sync::mpsc::UnboundedReceiver<helper::NtgEvent>) {
        RUNTIME.spawn(async move {
            while let Some(event) = rx.recv().await {
                let packet = Self::encode_packet(event);
                if let Err(e) = socket.write_all(&packet).await {
                    eprintln!("Socket Error : {}", e);
                    break;
                }
            }
        });
    }

    fn encode_packet(event: helper::NtgEvent) -> Vec<u8> {
        match event {
            // ID 1 : Stream //
            helper::NtgEvent::Stream(s) => {
                let mut p = Vec::with_capacity(1 + 8 + 4 + 4);
                p.push(1);
                p.extend_from_slice(&s.chat_id.to_le_bytes());
                p.extend_from_slice(&(s.stream_type as i32).to_le_bytes());
                p.extend_from_slice(&(s.device as i32).to_le_bytes());
                p
            }

            // ID 2 : Upgrade //
            helper::NtgEvent::Upgrade(u) => {
                let mut p = Vec::with_capacity(1 + 8 + 4);
                p.push(2);
                p.extend_from_slice(&u.chat_id.to_le_bytes());
                p.push(if u.media_state.muted { 1 } else { 0 });
                p.push(if u.media_state.video_paused { 1 } else { 0 });
                p.push(if u.media_state.video_stopped { 1 } else { 0 });
                p.push(if u.media_state.presentation_paused { 1 } else { 0 });
                p
            }

            // ID 3 : Connection //
            helper::NtgEvent::Connection(c) => {
                let mut p = Vec::with_capacity(1 + 8 + 8);
                p.push(3);
                p.extend_from_slice(&c.chat_id.to_le_bytes());
                p.extend_from_slice(&(c.network_info.kind as i32).to_le_bytes());
                p.extend_from_slice(&(c.network_info.state as i32).to_le_bytes());
                p
            }

            // ID 4 : Signaling //
            helper::NtgEvent::Signaling(sig) => {
                let mut p = Vec::with_capacity(1 + 8 + 4 + sig.data.len());
                p.push(4);
                p.extend_from_slice(&sig.chat_id.to_le_bytes());
                p.extend_from_slice(&(sig.data.len() as u32).to_le_bytes());
                p.extend_from_slice(&sig.data);
                p
            }

            // ID 5 : Frame //
            helper::NtgEvent::Frame(f) => {
                let frame_data = unsafe { std::slice::from_raw_parts(f.frame.data, f.frame.size_data as usize).to_vec() };
                let mut p = Vec::with_capacity(1 + 8 + 42 + frame_data.len());
                p.push(5);
                p.extend_from_slice(&f.chat_id.to_le_bytes());
                p.extend_from_slice(&(f.stream_mode as i32).to_le_bytes());
                p.extend_from_slice(&(f.device as i32).to_le_bytes());
                // Serializing ntg_frame_struct fields //
                p.extend_from_slice(&f.frame.ssrc.to_le_bytes());
                p.extend_from_slice(&(frame_data.len() as u32).to_le_bytes());
                p.extend_from_slice(&frame_data);
                // Serializing ntg_frame_data_struct fields //
                p.extend_from_slice(&f.frame.frame_data.absolute_capture_timestamp_ms.to_le_bytes());
                p.extend_from_slice(&f.frame.frame_data.width.to_le_bytes());
                p.extend_from_slice(&f.frame.frame_data.height.to_le_bytes());
                p.extend_from_slice(&f.frame.frame_data.rotation.to_le_bytes());
                p.extend_from_slice(&f.frame_count.to_le_bytes());
                p
            }

            // ID 6 : RemoteSource //
            helper::NtgEvent::RemoteSource(r) => {
                let mut p = Vec::with_capacity(1 + 8 + 4 + 4 + 4);
                p.push(6);
                p.extend_from_slice(&r.chat_id.to_le_bytes());
                p.extend_from_slice(&(r.remote_source.ssrc as u32).to_le_bytes());
                p.extend_from_slice(&(r.remote_source.state as i32).to_le_bytes());
                p.extend_from_slice(&(r.remote_source.device as i32).to_le_bytes());
                p
            }

            // ID 7 : BroadcastTimestamp //
            helper::NtgEvent::BroadcastTimestamp(bt) => {
                let mut p = Vec::with_capacity(1 + 8);
                p.push(7);
                p.extend_from_slice(&bt.chat_id.to_le_bytes());
                p
            }

            // ID 8 : BroadcastPart //
            helper::NtgEvent::BroadcastPart(bp) => {
                let mut p = Vec::with_capacity(1 + 8 + 33);
                p.push(8);
                p.extend_from_slice(&bp.chat_id.to_le_bytes());
                p.extend_from_slice(&bp.request.segment_id.to_le_bytes());
                p.extend_from_slice(&bp.request.part_id.to_le_bytes());
                p.extend_from_slice(&bp.request.limit.to_le_bytes());
                p.extend_from_slice(&bp.request.timestamp.to_le_bytes());
                p.push(if bp.request.quality_update { 1 } else { 0 });
                p.extend_from_slice(&bp.request.channel_id.to_le_bytes());
                p.extend_from_slice(&(bp.request.quality as i32).to_le_bytes());
                p
            }
        }
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .constant(wrap_constant!(PHP_TGCALLS_VERSION))
        .function(wrap_function!(tgcalls_get_protocol))
        .function(wrap_function!(tgcalls_get_version))
        .function(wrap_function!(tgcalls_get_media_devices))
        .function(wrap_function!(tgcalls_enable_glib_loop))
        .function(wrap_function!(tgcalls_set_log_level))
        .class::<TgcallsClient>()
        .class::<TgcallsEvents>()
        .class::<tgcalls::ntg_network_info_struct>()
        .class::<tgcalls::ntg_audio_description_struct>()
        .class::<tgcalls::ntg_video_description_struct>()
        .class::<tgcalls::ntg_media_description_struct>()
        .class::<tgcalls::ntg_call_info_struct>()
        .class::<tgcalls::ntg_media_state_struct>()
        .class::<tgcalls::ntg_rtc_server_struct>()
        .class::<tgcalls::ntg_protocol_struct>()
        .class::<tgcalls::ntg_dh_config_struct>()
        .class::<tgcalls::ntg_frame_data_struct>()
        .class::<tgcalls::ntg_remote_source_struct>()
        .class::<tgcalls::ntg_ssrc_group_struct>()
        .class::<tgcalls::ntg_device_info_struct>()
        .class::<tgcalls::ntg_media_devices_struct>()
        .class::<tgcalls::ntg_frame_struct>()
        .class::<tgcalls::ntg_segment_part_request_struct>()
        .class::<tgcalls::ntg_auth_params_struct>()
        .class::<tgcalls::ntg_log_message_struct>()
        .enumeration::<tgcalls::ntg_error_code_enum>()
        .enumeration::<tgcalls::ntg_media_source_enum>()
        .enumeration::<tgcalls::ntg_stream_device_enum>()
        .enumeration::<tgcalls::ntg_stream_mode_enum>()
        .enumeration::<tgcalls::ntg_stream_type_enum>()
        .enumeration::<tgcalls::ntg_stream_status_enum>()
        .enumeration::<tgcalls::ntg_connection_state_enum>()
        .enumeration::<tgcalls::ntg_connection_kind_enum>()
        .enumeration::<tgcalls::ntg_media_segment_quality_enum>()
        .enumeration::<tgcalls::ntg_media_segment_status_enum>()
        .enumeration::<tgcalls::ntg_connection_mode_enum>()
        .enumeration::<tgcalls::ntg_log_level_enum>()
        .enumeration::<tgcalls::ntg_log_source_enum>()
}