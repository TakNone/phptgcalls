use libc::{c_int, c_void, c_char, uintptr_t};
use std::ptr;
use std::ffi::{CStr, CString};
use ext_php_rs::prelude::*;
use ext_php_rs::binary::Binary;

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_enum]
#[php(name = "TgCalls\\ErrorCode")]
#[php(allow_native_discriminants)]
pub enum ntg_error_code_enum {
    // NTgCalls //
    #[php(name = "CONNECTION_NOT_FOUND", value = -101)]
    NTG_ERROR_CONNECTION_NOT_FOUND = -101,
    #[php(name = "CRYPTO", value = -102)]
    NTG_ERROR_CRYPTO = -102,
    #[php(name = "SIGNALING", value = -104)]
    NTG_ERROR_SIGNALING = -104,
    #[php(name = "SIGNALING_UNSUPPORTED", value = -105)]
    NTG_ERROR_SIGNALING_UNSUPPORTED = -105,
    #[php(name = "INVALID_PARAMS", value = -106)]
    NTG_ERROR_INVALID_PARAMS = -106,

    // STREAM //
    #[php(name = "FILE", value = -200)]
    NTG_ERROR_FILE = -200,
    #[php(name = "FFMPEG", value = -202)]
    NTG_ERROR_FFMPEG = -202,
    #[php(name = "SHELL", value = -203)]
    NTG_ERROR_SHELL = -203,
    #[php(name = "MEDIA_DEVICE", value = -204)]
    NTG_ERROR_MEDIA_DEVICE = -204,

    // WebRTC //
    #[php(name = "RTMP_STREAMING_UNSUPPORTED", value = -300)]
    NTG_ERROR_RTMP_STREAMING_UNSUPPORTED = -300,
    #[php(name = "PARSE_TRANSPORT", value = -301)]
    NTG_ERROR_PARSE_TRANSPORT = -301,
    #[php(name = "CONNECTION", value = -302)]
    NTG_ERROR_CONNECTION = -302,
    #[php(name = "TELEGRAM_SERVER", value = -303)]
    NTG_ERROR_TELEGRAM_SERVER = -303,
    #[php(name = "WEBRTC", value = -304)]
    NTG_ERROR_WEBRTC = -304,
    #[php(name = "PARSE_SDP", value = -305)]
    NTG_ERROR_PARSE_SDP = -305,
    #[php(name = "RTC_CONNECTION_NEEDED", value = -306)]
    NTG_ERROR_RTC_CONNECTION_NEEDED = -306,

    // Others //
    #[php(name = "UNKNOWN", value = -1)]
    NTG_ERROR_UNKNOWN = -1,
    #[php(name = "NULL_POINTER", value = -2)]
    NTG_ERROR_NULL_POINTER = -2,
    #[php(name = "TOO_SMALL", value = -3)]
    NTG_ERROR_TOO_SMALL = -3,
    #[php(name = "ASYNC_NOT_READY", value = -4)]
    NTG_ERROR_ASYNC_NOT_READY = -4,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_enum]
#[php(name = "TgCalls\\MediaSource")]
#[php(allow_native_discriminants)]
pub enum ntg_media_source_enum {
    #[php(name = "FILE", value = 1)]
    NTG_FILE = 1 << 0,
    #[php(name = "SHELL", value = 2)]
    NTG_SHELL = 1 << 1,
    #[php(name = "FFMPEG", value = 4)]
    NTG_FFMPEG = 1 << 2,
    #[php(name = "DEVICE", value = 8)]
    NTG_DEVICE = 1 << 3,
    #[php(name = "DESKTOP", value = 16)]
    NTG_DESKTOP = 1 << 4,
    #[php(name = "EXTERNAL", value = 32)]
    NTG_EXTERNAL = 1 << 5,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_enum]
#[php(name = "TgCalls\\StreamDevice")]
#[php(allow_native_discriminants)]
pub enum ntg_stream_device_enum {
    #[php(name = "MICROPHONE")]
    NTG_STREAM_MICROPHONE,
    #[php(name = "SPEAKER")]
    NTG_STREAM_SPEAKER,
    #[php(name = "CAMERA")]
    NTG_STREAM_CAMERA,
    #[php(name = "SCREEN")]
    NTG_STREAM_SCREEN,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_enum]
#[php(name = "TgCalls\\StreamMode")]
#[php(allow_native_discriminants)]
pub enum ntg_stream_mode_enum {
    #[php(name = "CAPTURE")]
    NTG_STREAM_CAPTURE,
    #[php(name = "PLAYBACK")]
    NTG_STREAM_PLAYBACK,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_enum]
#[php(name = "TgCalls\\StreamType")]
#[php(allow_native_discriminants)]
pub enum ntg_stream_type_enum {
    #[php(name = "AUDIO")]
    NTG_STREAM_AUDIO,
    #[php(name = "VIDEO")]
    NTG_STREAM_VIDEO,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_enum]
#[php(name = "TgCalls\\StreamStatus")]
#[php(allow_native_discriminants)]
pub enum ntg_stream_status_enum {
    #[php(name = "ACTIVE")]
    NTG_ACTIVE,
    #[php(name = "PAUSED")]
    NTG_PAUSED,
    #[php(name = "IDLING")]
    NTG_IDLING,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_enum]
#[php(name = "TgCalls\\ConnectionState")]
#[php(allow_native_discriminants)]
pub enum ntg_connection_state_enum {
    #[php(name = "CONNECTING")]
    NTG_STATE_CONNECTING,
    #[php(name = "CONNECTED")]
    NTG_STATE_CONNECTED,
    #[php(name = "TIMEOUT")]
    NTG_STATE_TIMEOUT,
    #[php(name = "FAILED")]
    NTG_STATE_FAILED,
    #[php(name = "CLOSED")]
    NTG_STATE_CLOSED,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_enum]
#[php(name = "TgCalls\\ConnectionKind")]
#[php(allow_native_discriminants)]
pub enum ntg_connection_kind_enum {
    #[php(name = "NORMAL")]
    NTG_KIND_NORMAL,
    #[php(name = "PRESENTATION")]
    NTG_KIND_PRESENTATION,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_enum]
#[php(name = "TgCalls\\MediaSegmentQuality")]
#[php(allow_native_discriminants)]
pub enum ntg_media_segment_quality_enum {
    #[php(name = "NONE")]
    NTG_MEDIA_SEGMENT_QUALITY_NONE,
    #[php(name = "THUMBNAIL")]
    NTG_MEDIA_SEGMENT_QUALITY_THUMBNAIL,
    #[php(name = "MEDIUM")]
    NTG_MEDIA_SEGMENT_QUALITY_MEDIUM,
    #[php(name = "FULL")]
    NTG_MEDIA_SEGMENT_QUALITY_FULL,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_enum]
#[php(name = "TgCalls\\MediaSegmentStatus")]
#[php(allow_native_discriminants)]
pub enum ntg_media_segment_status_enum {
    #[php(name = "NOT_READY")]
    NTG_MEDIA_SEGMENT_NOT_READY,
    #[php(name = "RESYNC_NEEDED")]
    NTG_MEDIA_SEGMENT_RESYNC_NEEDED,
    #[php(name = "SUCCESS")]
    NTG_MEDIA_SEGMENT_SUCCESS,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_enum]
#[php(name = "TgCalls\\ConnectionMode")]
#[php(allow_native_discriminants)]
pub enum ntg_connection_mode_enum {
    #[php(name = "NONE")]
    NTG_CONNECTION_MODE_NONE,
    #[php(name = "RTC")]
    NTG_CONNECTION_MODE_RTC,
    #[php(name = "STREAM")]
    NTG_CONNECTION_MODE_STREAM,
    #[php(name = "RTMP")]
    NTG_CONNECTION_MODE_RTMP,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_enum]
#[php(name = "TgCalls\\LogLevel")]
#[php(allow_native_discriminants)]
pub enum ntg_log_level_enum {
    #[php(name = "DEBUG", value = 1)]
    NTG_LOG_DEBUG = 1 << 0,
    #[php(name = "INFO", value = 2)]
    NTG_LOG_INFO = 1 << 1,
    #[php(name = "WARNING", value = 4)]
    NTG_LOG_WARNING = 1 << 2,
    #[php(name = "ERROR", value = 8)]
    NTG_LOG_ERROR = 1 << 3,
    #[php(name = "UNKNOWN", value = -1)]
    NTG_LOG_UNKNOWN = -1,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_enum]
#[php(name = "TgCalls\\LogSource")]
#[php(allow_native_discriminants)]
pub enum ntg_log_source_enum {
    #[php(name = "WEBRTC_LOG", value = 1)]
    NTG_LOG_WEBRTC = 1 << 0,
    #[php(name = "SELF_LOG", value = 2)]
    NTG_LOG_SELF = 1 << 1,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_class]
#[php(name = "TgCalls\\NetworkInfo")]
pub struct ntg_network_info_struct {
    #[php(prop, change_case = "snake_case")]
    pub kind: ntg_connection_kind_enum,
    #[php(prop, change_case = "snake_case")]
    pub state: ntg_connection_state_enum,
}

#[php_impl]
impl ntg_network_info_struct {
    pub fn __construct(
        kind: ntg_connection_kind_enum,
        state: ntg_connection_state_enum
    ) -> Self {
        Self { kind, state }
    }
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone)]
#[php_class]
#[php(name = "TgCalls\\AudioDescription")]
pub struct ntg_audio_description_struct {
    #[php(prop, change_case = "snake_case")]
    pub media_source: ntg_media_source_enum,
    pub input: *mut c_char,
    #[php(prop, change_case = "snake_case")]
    pub sample_rate: u32,
    #[php(prop, change_case = "snake_case")]
    pub channel_count: u8,
    #[php(prop, change_case = "snake_case")]
    pub keep_open: bool,
}

#[php_impl]
impl ntg_audio_description_struct {
    pub fn __construct(
        media_source: ntg_media_source_enum,
        input: String,
        sample_rate: u32,
        channel_count: u8,
        keep_open: bool
    ) -> Self {
        let input = CString::new(input).expect("CString::new failed").into_raw();
        Self { media_source, input, sample_rate, channel_count, keep_open }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_input(&self) -> String {
        unsafe {
            if !self.input.is_null() {
                CStr::from_ptr(self.input).to_string_lossy().into_owned()
            } else {
                String::new()
            }
        }
    }
}

impl Drop for ntg_audio_description_struct {
    fn drop(&mut self) {
        unsafe {
            if !self.input.is_null() {
                let _ = std::ffi::CString::from_raw(self.input);
            }
        }
    }
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone)]
#[php_class]
#[php(name = "TgCalls\\VideoDescription")]
pub struct ntg_video_description_struct {
    #[php(prop, change_case = "snake_case")]
    pub media_source: ntg_media_source_enum,
    pub input: *mut c_char,
    #[php(prop, change_case = "snake_case")]
    pub width: i16,
    #[php(prop, change_case = "snake_case")]
    pub height: i16,
    #[php(prop, change_case = "snake_case")]
    pub fps: u8,
    #[php(prop, change_case = "snake_case")]
    pub keep_open: bool,
}

#[php_impl]
impl ntg_video_description_struct {
    pub fn __construct(
        media_source: ntg_media_source_enum,
        input: String,
        width: i16,
        height: i16,
        fps: u8,
        keep_open: bool
    ) -> Self {
        let input = CString::new(input).expect("CString::new failed").into_raw();
        Self { media_source, input, width, height, fps, keep_open }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_input(&self) -> String {
        unsafe {
            if !self.input.is_null() {
                CStr::from_ptr(self.input).to_string_lossy().into_owned()
            } else {
                String::new()
            }
        }
    }
}

impl Drop for ntg_video_description_struct {
    fn drop(&mut self) {
        unsafe {
            if !self.input.is_null() {
                let _ = std::ffi::CString::from_raw(self.input);
            }
        }
    }
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_class]
#[php(name = "TgCalls\\MediaDescription")]
pub struct ntg_media_description_struct {
    pub microphone: *mut ntg_audio_description_struct,
    pub speaker: *mut ntg_audio_description_struct,
    pub camera: *mut ntg_video_description_struct,
    pub screen: *mut ntg_video_description_struct,
}

#[php_impl]
impl ntg_media_description_struct {
    pub fn __construct(
        microphone: Option<&ntg_audio_description_struct>,
        speaker: Option<&ntg_audio_description_struct>,
        camera: Option<&ntg_video_description_struct>,
        screen: Option<&ntg_video_description_struct>,
    ) -> Self {
        let microphone = microphone.map_or(ptr::null_mut(), |r| r as *const _ as *mut _);
        let speaker = speaker.map_or(ptr::null_mut(), |r| r as *const _ as *mut _);
        let camera = camera.map_or(ptr::null_mut(), |r| r as *const _ as *mut _);
        let screen = screen.map_or(ptr::null_mut(), |r| r as *const _ as *mut _);

        Self { microphone, speaker, camera, screen }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_microphone(&self) -> Option<ntg_audio_description_struct> {
        unsafe {
            if self.microphone.is_null() {
                None
            } else {
                Some((*self.microphone).clone())
            }
        }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_speaker(&self) -> Option<ntg_audio_description_struct> {
        unsafe {
            if self.speaker.is_null() {
                None
            } else {
                Some((*self.speaker).clone())
            }
        }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_camera(&self) -> Option<ntg_video_description_struct> {
        unsafe {
            if self.camera.is_null() {
                None
            } else {
                Some((*self.camera).clone())
            }
        }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_screen(&self) -> Option<ntg_video_description_struct> {
        unsafe {
            if self.screen.is_null() {
                None
            } else {
                Some((*self.screen).clone())
            }
        }
    }
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_class]
#[php(name = "TgCalls\\CallInfo")]
pub struct ntg_call_info_struct {
    #[php(prop, change_case = "snake_case")]
    pub chat_id: i64,
    #[php(prop, change_case = "snake_case")]
    pub capture: ntg_stream_status_enum,
    #[php(prop, change_case = "snake_case")]
    pub playback: ntg_stream_status_enum,
}

#[php_impl]
impl ntg_call_info_struct {
    pub fn __construct(
        chat_id: i64,
        capture: ntg_stream_status_enum,
        playback: ntg_stream_status_enum
    ) -> Self {
        Self { chat_id, capture, playback }
    }
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_class]
#[php(name = "TgCalls\\MediaState")]
pub struct ntg_media_state_struct {
    #[php(prop, change_case = "snake_case")]
    pub muted: bool,
    #[php(prop, change_case = "snake_case")]
    pub video_paused: bool,
    #[php(prop, change_case = "snake_case")]
    pub video_stopped: bool,
    #[php(prop, change_case = "snake_case")]
    pub presentation_paused: bool,
}

#[php_impl]
impl ntg_media_state_struct {
    pub fn __construct(
        muted: bool,
        video_paused: bool,
        video_stopped: bool,
        presentation_paused: bool
    ) -> Self {
        Self { muted, video_paused, video_stopped, presentation_paused }
    }
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_class]
#[php(name = "TgCalls\\RtcServer")]
pub struct ntg_rtc_server_struct {
    #[php(prop, change_case = "snake_case")]
    pub id: u64,
    pub ipv4: *mut c_char,
    pub ipv6: *mut c_char,
    pub username: *mut c_char,
    pub password: *mut c_char,
    #[php(prop, change_case = "snake_case")]
    pub port: u16,
    #[php(prop, change_case = "snake_case")]
    pub turn: bool,
    #[php(prop, change_case = "snake_case")]
    pub stun: bool,
    #[php(prop, change_case = "snake_case")]
    pub tcp: bool,
    pub peer_tag: *mut u8,
    pub peer_tag_size: c_int,
}

#[php_impl]
impl ntg_rtc_server_struct {
    pub fn __construct(
        id: u64,
        ipv4: String,
        ipv6: String,
        port: u16,
        turn: bool,
        stun: bool,
        tcp: bool,
        username: String,
        password: String,
        mut peer_tag: Binary<u8>
    ) -> Self {
        let ipv4 = CString::new(ipv4).expect("CString::new failed").into_raw();
        let ipv6 = CString::new(ipv6).expect("CString::new failed").into_raw();

        let username = CString::new(username).expect("CString::new failed").into_raw();
        let password = CString::new(password).expect("CString::new failed").into_raw();

        let (peer_tag, peer_tag_size): (*mut u8, c_int) = (peer_tag.as_mut_ptr(), peer_tag.len() as c_int);

        Self {
            id,
            ipv4,
            ipv6,
            username,
            password,
            port,
            turn,
            stun,
            tcp,
            peer_tag,
            peer_tag_size
        }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_ipv4(&self) -> String {
        unsafe {
            if !self.ipv4.is_null() {
                CStr::from_ptr(self.ipv4).to_string_lossy().into_owned()
            } else {
                String::new()
            }
        }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_ipv6(&self) -> String {
        unsafe {
            if !self.ipv6.is_null() {
                CStr::from_ptr(self.ipv6).to_string_lossy().into_owned()
            } else {
                String::new()
            }
        }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_username(&self) -> String {
        unsafe {
            if !self.username.is_null() {
                CStr::from_ptr(self.username).to_string_lossy().into_owned()
            } else {
                String::new()
            }
        }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_password(&self) -> String {
        unsafe {
            if !self.password.is_null() {
                CStr::from_ptr(self.password).to_string_lossy().into_owned()
            } else {
                String::new()
            }
        }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_peer_tag(&self) -> Binary<u8> {
        unsafe {
            if !self.peer_tag.is_null() && self.peer_tag_size > 0 {
                let slice = std::slice::from_raw_parts(self.peer_tag, self.peer_tag_size as usize);
                Binary::from(slice.to_vec())
            } else {
                Binary::from(Vec::new())
            }
        }
    }
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_class]
#[php(name = "TgCalls\\Protocol")]
pub struct ntg_protocol_struct {
    #[php(prop, change_case = "snake_case")]
    pub min_layer: i32,
    #[php(prop, change_case = "snake_case")]
    pub max_layer: i32,
    #[php(prop, change_case = "snake_case")]
    pub udp_p2p: bool,
    #[php(prop, change_case = "snake_case")]
    pub udp_reflector: bool,
    pub library_versions: *mut *mut c_char,
    pub library_versions_size: c_int,
}

#[php_impl]
impl ntg_protocol_struct {
    pub fn __construct(
        min_layer: i32,
        max_layer: i32,
        udp_p2p: bool,
        udp_reflector: bool,
        library_versions: Vec<String>
    ) -> Self {
        let c_strings: Vec<*mut c_char> = library_versions.into_iter().map(|s| CString::new(s).unwrap().into_raw()).collect();
        let mut boxed_slice = c_strings.into_boxed_slice();
        let library_versions = boxed_slice.as_mut_ptr();
        let library_versions_size = boxed_slice.len() as c_int;
        std::mem::forget(boxed_slice);

        Self {
            min_layer,
            max_layer,
            udp_p2p,
            udp_reflector,
            library_versions,
            library_versions_size
        }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_library_versions(&self) -> Vec<String> {
        unsafe {
            let mut versions = Vec::new();

            if !self.library_versions.is_null() && self.library_versions_size > 0 {
                for i in 0..self.library_versions_size as isize {
                    let char_ptr = *self.library_versions.offset(i);
                    if !char_ptr.is_null() {
                        let version_str = CStr::from_ptr(char_ptr).to_string_lossy().into_owned();
                        versions.push(version_str);
                    }
                }
            }

            versions
        }
    }
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone)]
#[php_class]
#[php(name = "TgCalls\\DhConfig")]
pub struct ntg_dh_config_struct {
    pub g: i32,
    pub p: *const u8,
    pub size_p: c_int,
    pub random: *const u8,
    pub size_random: c_int,
}

#[php_impl]
impl ntg_dh_config_struct {
    pub fn __construct(
        g: i32,
        p: Binary<u8>,
        random: Binary<u8>
    ) -> Self {
        let mut p_vec = p.to_vec();
        p_vec.shrink_to_fit();
        let size_p = p_vec.len() as c_int;
        let p_ptr = p_vec.as_mut_ptr();
        std::mem::forget(p_vec);

        let mut random_vec = random.to_vec();
        random_vec.shrink_to_fit();
        let size_random = random_vec.len() as c_int;
        let random_ptr = random_vec.as_mut_ptr();
        std::mem::forget(random_vec);

        Self { g, p: p_ptr, size_p, random: random_ptr, size_random }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_p(&self) -> Binary<u8> {
        unsafe {
            if !self.p.is_null() && self.size_p > 0 {
                let slice = std::slice::from_raw_parts(self.p, self.size_p as usize);
                Binary::from(slice.to_vec())
            } else {
                Binary::from(Vec::new())
            }
        }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_random(&self) -> Binary<u8> {
        unsafe {
            if !self.random.is_null() && self.size_random > 0 {
                let slice = std::slice::from_raw_parts(self.random, self.size_random as usize);
                Binary::from(slice.to_vec())
            } else {
                Binary::from(Vec::new())
            }
        }
    }
}

impl Drop for ntg_dh_config_struct {
    fn drop(&mut self) {
        unsafe {
            if !self.p.is_null() && self.size_p > 0 {
                drop(Vec::from_raw_parts(self.p as *mut u8, self.size_p as usize, self.size_p as usize));
            }
            if !self.random.is_null() && self.size_random > 0 {
                drop(Vec::from_raw_parts(self.random as *mut u8, self.size_random as usize, self.size_random as usize));
            }
        }
    }
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_class]
#[php(name = "TgCalls\\FrameData")]
pub struct ntg_frame_data_struct {
    #[php(prop, change_case = "snake_case")]
    pub absolute_capture_timestamp_ms: i64,
    #[php(prop, change_case = "snake_case")]
    pub width: u16,
    #[php(prop, change_case = "snake_case")]
    pub height: u16,
    #[php(prop, change_case = "snake_case")]
    pub rotation: u16,
}

#[php_impl]
impl ntg_frame_data_struct {
    pub fn __construct(
        absolute_capture_timestamp_ms: i64,
        width: u16,
        height: u16,
        rotation: u16
    ) -> Self {
        Self { absolute_capture_timestamp_ms, width, height, rotation }
    }
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_class]
#[php(name = "TgCalls\\RemoteSource")]
pub struct ntg_remote_source_struct {
    #[php(prop, change_case = "snake_case")]
    pub ssrc: u32,
    #[php(prop, change_case = "snake_case")]
    pub state: ntg_stream_status_enum,
    #[php(prop, change_case = "snake_case")]
    pub device: ntg_stream_device_enum,
}

#[php_impl]
impl ntg_remote_source_struct {
    pub fn __construct(
        ssrc: u32,
        state: ntg_stream_status_enum,
        device: ntg_stream_device_enum
    ) -> Self {
        Self { ssrc, state, device }
    }
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone)]
#[php_class]
#[php(name = "TgCalls\\SsrcGroup")]
pub struct ntg_ssrc_group_struct {
    pub semantics: *mut c_char,
    pub ssrcs: *mut u32,
    pub size_ssrcs: c_int,
}

#[php_impl]
impl ntg_ssrc_group_struct {
    pub fn __construct(
        semantics: String,
        ssrcs: Vec<i64>
    ) -> Self {
        let semantics = CString::new(semantics).expect("CString::new failed").into_raw();
        let u32_ssrcs: Vec<u32> = ssrcs.iter().map(|&x| x as u32).collect();
        let mut boxed_ssrcs = u32_ssrcs.into_boxed_slice();
        let ssrcs = boxed_ssrcs.as_mut_ptr();
        let size_ssrcs = boxed_ssrcs.len() as c_int;
        std::mem::forget(boxed_ssrcs);

        Self { semantics, ssrcs, size_ssrcs }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_semantics(&self) -> String {
        unsafe {
            if !self.semantics.is_null() {
                CStr::from_ptr(self.semantics).to_string_lossy().into_owned()
            } else {
                String::new()
            }
        }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_ssrcs(&self) -> Vec<u32> {
        unsafe {
            if !self.ssrcs.is_null() && self.size_ssrcs > 0 {
                let slice = std::slice::from_raw_parts(self.ssrcs, self.size_ssrcs as usize);
                slice.to_vec()
            } else {
                Vec::new()
            }
        }
    }
}

impl Drop for ntg_ssrc_group_struct {
    fn drop(&mut self) {
        unsafe {
            if !self.semantics.is_null() {
                let _ = std::ffi::CString::from_raw(self.semantics);
            }
            if !self.ssrcs.is_null() && self.size_ssrcs > 0 {
                drop(Box::from_raw(std::slice::from_raw_parts_mut(self.ssrcs, self.size_ssrcs as usize)));
            }
        }
    }
}

#[allow(dead_code, non_camel_case_types)]
pub type ntg_async_callback = Option<unsafe extern "C" fn(user_data: *mut c_void)>;

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct ntg_async_struct {
    pub user_data: *mut c_void,
    pub error_code: *mut c_int,
    pub error_message: *mut *mut c_char,
    pub promise: ntg_async_callback,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone)]
#[php_class]
#[php(name = "TgCalls\\DeviceInfo")]
pub struct ntg_device_info_struct {
    pub name: *mut c_char,
    pub metadata: *mut c_char,
}

#[php_impl]
impl ntg_device_info_struct {
    pub fn __construct(
        name: String,
        metadata: String
    ) -> Self {
        let name = CString::new(name).expect("CString::new failed").into_raw();
        let metadata = CString::new(metadata).expect("CString::new failed").into_raw();
        Self { name, metadata }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_name(&self) -> String {
        unsafe {
            if !self.name.is_null() {
                CStr::from_ptr(self.name).to_string_lossy().into_owned()
            } else {
                String::new()
            }
        }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_metadata(&self) -> String {
        unsafe {
            if !self.metadata.is_null() {
                CStr::from_ptr(self.metadata).to_string_lossy().into_owned()
            } else {
                String::new()
            }
        }
    }
}

impl Drop for ntg_device_info_struct {
    fn drop(&mut self) {
        unsafe {
            if !self.name.is_null() {
                let _ = std::ffi::CString::from_raw(self.name);
            }
            if !self.metadata.is_null() {
                let _ = std::ffi::CString::from_raw(self.metadata);
            }
        }
    }
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_class]
#[php(name = "TgCalls\\MediaDevices")]
pub struct ntg_media_devices_struct {
    pub microphone: *mut ntg_device_info_struct,
    pub size_microphone: c_int,
    pub speaker: *mut ntg_device_info_struct,
    pub size_speaker: c_int,
    pub camera: *mut ntg_device_info_struct,
    pub size_camera: c_int,
    pub screen: *mut ntg_device_info_struct,
    pub size_screen: c_int,
}

#[php_impl]
impl ntg_media_devices_struct {
    pub fn __construct(
        microphone: Option<&mut ntg_device_info_struct>,
        speaker: Option<&mut ntg_device_info_struct>,
        camera: Option<&mut ntg_device_info_struct>,
        screen: Option<&mut ntg_device_info_struct>
    ) -> Self {
        let (microphone, size_microphone) = match microphone {
            Some(r) => (r as *const _ as *mut _, 1),
            None => (std::ptr::null_mut(), 0),
        };
        let (speaker, size_speaker) = match speaker {
            Some(r) => (r as *const _ as *mut _, 1),
            None => (std::ptr::null_mut(), 0),
        };
        let (camera, size_camera) = match camera {
            Some(r) => (r as *const _ as *mut _, 1),
            None => (std::ptr::null_mut(), 0),
        };
        let (screen, size_screen) = match screen {
            Some(r) => (r as *const _ as *mut _, 1),
            None => (std::ptr::null_mut(), 0),
        };
        Self {
            microphone,
            size_microphone,
            speaker,
            size_speaker,
            camera,
            size_camera,
            screen,
            size_screen
        }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_microphone(&self) -> Vec<ntg_device_info_struct> {
        unsafe {
            if !self.microphone.is_null() && self.size_microphone > 0 {
                let slice = std::slice::from_raw_parts(self.microphone, self.size_microphone as usize);
                slice.to_vec()
            } else {
                Vec::new()
            }
        }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_speaker(&self) -> Vec<ntg_device_info_struct> {
        unsafe {
            if !self.speaker.is_null() && self.size_speaker > 0 {
                let slice = std::slice::from_raw_parts(self.speaker, self.size_speaker as usize);
                slice.to_vec()
            } else {
                Vec::new()
            }
        }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_camera(&self) -> Vec<ntg_device_info_struct> {
        unsafe {
            if !self.camera.is_null() && self.size_camera > 0 {
                let slice = std::slice::from_raw_parts(self.camera, self.size_camera as usize);
                slice.to_vec()
            } else {
                Vec::new()
            }
        }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_screen(&self) -> Vec<ntg_device_info_struct> {
        unsafe {
            if !self.screen.is_null() && self.size_screen > 0 {
                let slice = std::slice::from_raw_parts(self.screen, self.size_screen as usize);
                slice.to_vec()
            } else {
                Vec::new()
            }
        }
    }
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_class]
#[php(name = "TgCalls\\Frame")]
pub struct ntg_frame_struct {
    #[php(prop, change_case = "snake_case")]
    pub ssrc: i64,
    pub data: *mut u8,
    pub size_data: c_int,
    pub frame_data: ntg_frame_data_struct,
}

#[php_impl]
impl ntg_frame_struct {
    pub fn __construct(
        ssrc: i64,
        mut data: Binary<u8>,
        frame_data: &ntg_frame_data_struct
    ) -> Self {
        let size_data = data.len() as c_int;
        let data = data.as_mut_ptr();

        let frame_data = *frame_data;
        Self { ssrc, data, size_data, frame_data }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_data(&self) -> Binary<u8> {
        unsafe {
            if !self.data.is_null() && self.size_data > 0 {
                let slice = std::slice::from_raw_parts(self.data, self.size_data as usize);
                Binary::from(slice.to_vec())
            } else {
                Binary::from(Vec::new())
            }
        }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_frame_data(&self) -> ntg_frame_data_struct {
        self.frame_data
    }
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_class]
#[php(name = "TgCalls\\SegmentPartRequest")]
pub struct ntg_segment_part_request_struct {
    #[php(prop, change_case = "snake_case")]
    pub segment_id: i64,
    #[php(prop, change_case = "snake_case")]
    pub part_id: i32,
    #[php(prop, change_case = "snake_case")]
    pub limit: i32,
    #[php(prop, change_case = "snake_case")]
    pub timestamp: i64,
    #[php(prop, change_case = "snake_case")]
    pub quality_update: bool,
    #[php(prop, change_case = "snake_case")]
    pub channel_id: i32,
    #[php(prop, change_case = "snake_case")]
    pub quality: ntg_media_segment_quality_enum,
}

#[php_impl]
impl ntg_segment_part_request_struct {
    pub fn __construct(
        segment_id: i64,
        part_id: i32,
        limit: i32,
        timestamp: i64,
        quality_update: bool,
        channel_id: i32,
        quality: ntg_media_segment_quality_enum
    ) -> Self {
        Self { segment_id, part_id, limit, timestamp, quality_update, channel_id, quality }
    }
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone)]
#[php_class]
#[php(name = "TgCalls\\AuthParams")]
pub struct ntg_auth_params_struct {
    pub g_a_or_b: *mut u8,
    pub size_g_a_b: c_int,
    #[php(prop, change_case = "snake_case")]
    pub key_fingerprint: i64,
}

#[php_impl]
impl ntg_auth_params_struct {
    pub fn __construct(
        g_a_or_b: Binary<u8>,
        key_fingerprint: i64
    ) -> Self {
        let mut g_vec = g_a_or_b.to_vec();
        g_vec.shrink_to_fit();
        let size_g_a_b = g_vec.len() as c_int;
        let g_ptr = g_vec.as_mut_ptr();
        std::mem::forget(g_vec);

        Self { g_a_or_b: g_ptr, size_g_a_b, key_fingerprint }
    }

    #[php(getter, change_case = "snake_case")]
    pub fn get_g_a_or_b(&self) -> Binary<u8> {
        unsafe {
            if !self.g_a_or_b.is_null() && self.size_g_a_b > 0 {
                let slice = std::slice::from_raw_parts(self.g_a_or_b, self.size_g_a_b as usize);
                Binary::from(slice.to_vec())
            } else {
                Binary::from(Vec::new())
            }
        }
    }
}

impl Drop for ntg_auth_params_struct {
    fn drop(&mut self) {
        unsafe {
            if !self.g_a_or_b.is_null() && self.size_g_a_b > 0 {
                drop(Vec::from_raw_parts(self.g_a_or_b, self.size_g_a_b as usize, self.size_g_a_b as usize));
            }
        }
    }
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[php_class]
#[php(name = "TgCalls\\LogMessage")]
pub struct ntg_log_message_struct {
    pub level: ntg_log_level_enum,
    pub source: ntg_log_source_enum,
    pub file: *mut c_char,
    pub line: u32,
    pub message: *mut c_char,
}

#[php_impl]
impl ntg_log_message_struct {
    pub fn __construct(
        level: ntg_log_level_enum,
        source: ntg_log_source_enum,
        file: String,
        line: u32,
        message: String
    ) -> Self {
        let file = CString::new(file).expect("CString::new failed").into_raw();
        let message = CString::new(message).expect("CString::new failed").into_raw();
        Self { level, source, file, line, message }
    }
}

#[allow(dead_code, non_camel_case_types)]
pub type ntg_stream_callback = Option<unsafe extern "C" fn(
    ptr: uintptr_t,
    chat_id: i64,
    stream_type: ntg_stream_type_enum,
    device: ntg_stream_device_enum,
    user_data: *mut c_void)>;

#[allow(dead_code, non_camel_case_types)]
pub type ntg_upgrade_callback = Option<unsafe extern "C" fn(
    ptr: uintptr_t,
    chat_id: i64,
    media_state: ntg_media_state_struct,
    user_data: *mut c_void)>;

#[allow(dead_code, non_camel_case_types)]
pub type ntg_connection_callback = Option<unsafe extern "C" fn(
    ptr: uintptr_t,
    chat_id: i64,
    network_info: ntg_network_info_struct,
    user_data: *mut c_void)>;

#[allow(dead_code, non_camel_case_types)]
pub type ntg_signaling_callback = Option<unsafe extern "C" fn(
    ptr: uintptr_t,
    chat_id: i64,
    buffer: *mut u8,
    size: c_int,
    user_data: *mut c_void)>;

#[allow(dead_code, non_camel_case_types)]
pub type ntg_frame_callback = Option<unsafe extern "C" fn(
    ptr: uintptr_t,
    chat_id: i64,
    stream_mode: ntg_stream_mode_enum,
    device: ntg_stream_device_enum,
    frame: *mut ntg_frame_struct,
    frame_count: u64,
    user_data: *mut c_void)>;

#[allow(dead_code, non_camel_case_types)]
pub type ntg_remote_source_callback = Option<unsafe extern "C" fn(
    ptr: uintptr_t,
    chat_id: i64,
    remote_source: ntg_remote_source_struct,
    user_data: *mut c_void)>;

#[allow(dead_code, non_camel_case_types)]
pub type ntg_broadcast_timestamp_callback = Option<unsafe extern "C" fn(
    ptr: uintptr_t,
    chat_id: i64,
    user_data: *mut c_void)>;

#[allow(dead_code, non_camel_case_types)]
pub type ntg_broadcast_part_callback = Option<unsafe extern "C" fn(
    ptr: uintptr_t,
    chat_id: i64,
    request: ntg_segment_part_request_struct,
    user_data: *mut c_void)>;

#[allow(dead_code, non_camel_case_types)]
pub type ntg_log_message_callback = Option<unsafe extern "C" fn(message: ntg_log_message_struct)>;

#[allow(dead_code)]
#[link(name = "ntgcalls")]
unsafe extern "C" {
    // --- Logger --- //
    pub fn ntg_register_logger(callback: ntg_log_message_callback);

    // --- Core Client --- //
    pub fn ntg_init() -> uintptr_t;
    pub fn ntg_destroy(ptr: uintptr_t) -> i32;
    pub fn ntg_get_protocol(buffer: *mut ntg_protocol_struct) -> i32;
    pub fn ntg_get_version(buffer: *mut *mut c_char) -> i32;
    pub fn ntg_cpu_usage(ptr: uintptr_t, buffer: *mut f64, future: ntg_async_struct) -> i32;
    pub fn ntg_enable_g_lib_loop(enable: bool) -> i32;

    // --- Media Devices --- //
    pub fn ntg_get_media_devices(buffer: *mut ntg_media_devices_struct) -> i32;

    // --- P2P Methods --- //
    pub fn ntg_create_p2p(ptr: uintptr_t, user_id: i64, future: ntg_async_struct) -> i32;
    pub fn ntg_init_exchange(
        ptr: uintptr_t,
        user_id: i64,
        dh_config: *mut ntg_dh_config_struct,
        g_a_hash: *const u8,
        size_g_a_hash: i32,
        buffer: *mut *mut u8,
        size: *mut i32,
        future: ntg_async_struct
    ) -> i32;
    pub fn ntg_exchange_keys(
        ptr: uintptr_t,
        user_id: i64,
        g_a_or_b: *const u8,
        size_g_a_b: i32,
        fingerprint: i64,
        buffer: *mut ntg_auth_params_struct,
        future: ntg_async_struct
    ) -> i32;
    pub fn ntg_skip_exchange(
        ptr: uintptr_t,
        user_id: i64,
        encryption_key: *const u8,
        size: i32,
        is_outgoing: bool,
        future: ntg_async_struct
    ) -> i32;
    pub fn ntg_connect_p2p(
        ptr: uintptr_t,
        user_id: i64,
        servers: *mut ntg_rtc_server_struct,
        servers_size: i32,
        versions: *mut *mut c_char,
        versions_size: i32,
        p2p_allowed: bool,
        future: ntg_async_struct
    ) -> i32;
    pub fn ntg_send_signaling_data(
        ptr: uintptr_t,
        user_id: i64,
        buffer: *mut u8,
        size: i32,
        future: ntg_async_struct
    ) -> i32;

    // --- Group Call Methods (RTC) --- //
    pub fn ntg_create(ptr: uintptr_t, chat_id: i64, buffer: *mut *mut c_char, future: ntg_async_struct) -> i32;
    pub fn ntg_connect(
        ptr: uintptr_t,
        chat_id: i64,
        params: *mut c_char,
        is_presentation: bool,
        future: ntg_async_struct
    ) -> i32;
    pub fn ntg_stop(ptr: uintptr_t, chat_id: i64, future: ntg_async_struct) -> i32;
    pub fn ntg_calls(ptr: uintptr_t, buffer: *mut *mut ntg_call_info_struct, size: *mut i32, future: ntg_async_struct) -> i32;

    // --- Presentation Methods --- //
    pub fn ntg_init_presentation(ptr: uintptr_t, chat_id: i64, buffer: *mut *mut c_char, future: ntg_async_struct) -> i32;
    pub fn ntg_stop_presentation(ptr: uintptr_t, chat_id: i64, future: ntg_async_struct) -> i32;
    pub fn ntg_add_incoming_video(
        ptr: uintptr_t,
        chat_id: i64,
        endpoint: *mut c_char,
        ssrc_groups: *mut ntg_ssrc_group_struct,
        size: i32,
        buffer: *mut u32,
        future: ntg_async_struct
    ) -> i32;
    pub fn ntg_remove_incoming_video(ptr: uintptr_t, chat_id: i64, endpoint: *mut c_char, future: ntg_async_struct) -> i32;

    // --- Stream Control Methods --- //
    pub fn ntg_set_stream_sources(
        ptr: uintptr_t,
        chat_id: i64,
        stream_mode: ntg_stream_mode_enum,
        desc: ntg_media_description_struct,
        future: ntg_async_struct
    ) -> i32;
    pub fn ntg_pause(ptr: uintptr_t, chat_id: i64, future: ntg_async_struct) -> i32;
    pub fn ntg_resume(ptr: uintptr_t, chat_id: i64, future: ntg_async_struct) -> i32;
    pub fn ntg_mute(ptr: uintptr_t, chat_id: i64, future: ntg_async_struct) -> i32;
    pub fn ntg_unmute(ptr: uintptr_t, chat_id: i64, future: ntg_async_struct) -> i32;

    // --- Get State/Info Methods --- //
    pub fn ntg_time(
        ptr: uintptr_t,
        chat_id: i64,
        stream_mode: ntg_stream_mode_enum,
        time: *mut i64,
        future: ntg_async_struct
    ) -> i32;
    pub fn ntg_get_state(
        ptr: uintptr_t,
        chat_id: i64,
        media_state: *mut ntg_media_state_struct,
        future: ntg_async_struct
    ) -> i32;
    pub fn ntg_get_connection_mode(ptr: uintptr_t, chat_id: i64, mode: *mut ntg_connection_mode_enum, future: ntg_async_struct) -> i32;

    // --- Data Sending Methods --- //
    pub fn ntg_send_external_frame(
        ptr: uintptr_t,
        chat_id: i64,
        device: ntg_stream_device_enum,
        frame: *mut u8,
        frame_size: i32,
        frame_data: ntg_frame_data_struct,
        future: ntg_async_struct
    ) -> i32;
    pub fn ntg_send_broadcast_timestamp(ptr: uintptr_t, chat_id: i64, timestamp: i64, future: ntg_async_struct) -> i32;
    pub fn ntg_send_broadcast_part(
        ptr: uintptr_t,
        chat_id: i64,
        segment_id: i64,
        part_id: i32,
        status: ntg_media_segment_status_enum,
        quality_update: bool,
        frame: *const u8,
        frame_size: i32,
        future: ntg_async_struct
    ) -> i32;

    // --- Callback Registration Methods --- //
    pub fn ntg_on_stream_end(ptr: uintptr_t, callback: ntg_stream_callback, user_data: *mut c_void) -> i32;
    pub fn ntg_on_upgrade(ptr: uintptr_t, callback: ntg_upgrade_callback, user_data: *mut c_void) -> i32;
    pub fn ntg_on_connection_change(ptr: uintptr_t, callback: ntg_connection_callback, user_data: *mut c_void) -> i32;
    pub fn ntg_on_signaling_data(ptr: uintptr_t, callback: ntg_signaling_callback, user_data: *mut c_void) -> i32;
    pub fn ntg_on_frames(ptr: uintptr_t, callback: ntg_frame_callback, user_data: *mut c_void) -> i32;
    pub fn ntg_on_remote_source_change(ptr: uintptr_t, callback: ntg_remote_source_callback, user_data: *mut c_void) -> i32;
    pub fn ntg_on_request_broadcast_timestamp(ptr: uintptr_t, callback: ntg_broadcast_timestamp_callback, user_data: *mut c_void) -> i32;
    pub fn ntg_on_request_broadcast_part(ptr: uintptr_t, callback: ntg_broadcast_part_callback, user_data: *mut c_void) -> i32;
}