use libc::{c_int, c_void, c_char, uintptr_t};
use std::ffi::CStr;
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;
use colored::*;

use crate::tgcalls::*;

pub struct NtgFuture {
    pair: Arc<(Mutex<bool>, Condvar)>,
    pub error_code_buf: Box<c_int>, 
    pub error_msg_buf: Box<*mut c_char>,
}

impl NtgFuture {
    pub fn new() -> Self {
        Self {
            pair: Arc::new((Mutex::new(false), Condvar::new())),
            error_code_buf: Box::new(0),
            error_msg_buf: Box::new(std::ptr::null_mut()),
        }
    }

    pub fn as_async_struct(&mut self) -> ntg_async_struct {
        let user_data = Arc::into_raw(self.pair.clone()) as *mut c_void;

        ntg_async_struct {
            user_data,
            error_code: &mut *self.error_code_buf,
            error_message: &mut *self.error_msg_buf,
            promise: Some(Self::c_callback),
        }
    }

    unsafe extern "C" fn c_callback(user_data: *mut c_void) {
        if user_data.is_null() {
            return;
        }
        let pair = unsafe {
            Arc::from_raw(user_data as *const (Mutex<bool>, Condvar))
        };
        let (lock, cvar) = &*pair;
        let mut completed = lock.lock().unwrap();
        *completed = true;
        cvar.notify_one();
    }

    pub fn wait_timeout(&self, duration: Duration) -> Result<i32, String> {
        let (lock, cvar) = &*self.pair;
        let mut completed = lock.lock().unwrap();
        while !*completed {
            let result = cvar.wait_timeout(completed, duration).unwrap();
            completed = result.0;
            if result.1.timed_out() {
                return Err("Timeout waiting for callback".to_string());
            }
        }
        let error_code: i32 = *self.error_code_buf;
        let error_message: String = if (*self.error_msg_buf).is_null() {
            "<NONE>".to_string()
        } else {
            unsafe {
                CStr::from_ptr(*self.error_msg_buf).to_str().unwrap_or("<INVALID UTF-8>").to_string()
            }
        };
        if error_code != 0 {
            eprintln!("[NtgCalls {}] {} : {}\n", "ERROR".red().bold(), error_message, error_code);
        }

        Ok(error_code)
    }
}

pub enum NtgEvent {
    Stream(StreamEvent),
    Upgrade(UpgradeEvent),
    Connection(ConnectionEvent),
    Signaling(SignalingEvent),
    Frame(FrameEvent),
    RemoteSource(RemoteSourceEvent),
    BroadcastTimestamp(BroadcastTimestampEvent),
    BroadcastPart(BroadcastPartEvent),
}

pub struct StreamEvent {
    pub chat_id: i64,
    pub stream_type: ntg_stream_type_enum,
    pub device: ntg_stream_device_enum,
}

pub struct UpgradeEvent {
    pub chat_id: i64,
    pub media_state: ntg_media_state_struct,
}

pub struct ConnectionEvent {
    pub chat_id: i64,
    pub network_info: ntg_network_info_struct,
}

pub struct SignalingEvent {
    pub chat_id: i64,
    pub data: Vec<u8>,
}

pub struct FrameEvent {
    pub chat_id: i64,
    pub stream_mode: ntg_stream_mode_enum,
    pub device: ntg_stream_device_enum,
    pub frame: ntg_frame_struct,
    pub frame_count: u64,
}

unsafe impl Send for FrameEvent {}

pub struct RemoteSourceEvent {
    pub chat_id: i64,
    pub remote_source: ntg_remote_source_struct,
}

pub struct BroadcastTimestampEvent {
    pub chat_id: i64,
}

pub struct BroadcastPartEvent {
    pub chat_id: i64,
    pub request: ntg_segment_part_request_struct,
}

pub unsafe extern "C" fn proxy_on_stream(
    _ptr: uintptr_t,
    chat_id: i64,
    stream_type: ntg_stream_type_enum,
    device: ntg_stream_device_enum,
    user_data: *mut c_void,
) {
    unsafe {
        if user_data.is_null() {
            return;
        }
        let tx = &*(user_data as *mut tokio::sync::mpsc::UnboundedSender<NtgEvent>);
        #[allow(unused_must_use)] tx.send(NtgEvent::Stream(StreamEvent { chat_id, stream_type, device }));
    }
}

pub unsafe extern "C" fn proxy_on_upgrade(
    _ptr: uintptr_t,
    chat_id: i64,
    media_state: ntg_media_state_struct,
    user_data: *mut c_void,
) {
    unsafe {
        if user_data.is_null() {
            return;
        }
        let tx = &*(user_data as *mut tokio::sync::mpsc::UnboundedSender<NtgEvent>);
        #[allow(unused_must_use)] tx.send(NtgEvent::Upgrade(UpgradeEvent { chat_id, media_state }));
    }
}

pub unsafe extern "C" fn proxy_on_connection(
    _ptr: uintptr_t,
    chat_id: i64,
    network_info: ntg_network_info_struct,
    user_data: *mut c_void,
) {
    unsafe {
        if user_data.is_null() {
            return;
        }
        let tx = &*(user_data as *mut tokio::sync::mpsc::UnboundedSender<NtgEvent>);
        #[allow(unused_must_use)] tx.send(NtgEvent::Connection(ConnectionEvent { chat_id, network_info }));
    }
}

pub unsafe extern "C" fn proxy_on_signaling_data(
    _ptr: uintptr_t,
    chat_id: i64,
    buffer: *mut u8,
    size: c_int,
    user_data: *mut c_void,
) {
    unsafe {
        if user_data.is_null() || buffer.is_null() {
            return;
        }
        let data = std::slice::from_raw_parts(buffer, size as usize).to_vec();
        let tx = &*(user_data as *mut tokio::sync::mpsc::UnboundedSender<NtgEvent>);
        #[allow(unused_must_use)] tx.send(NtgEvent::Signaling(SignalingEvent { chat_id, data }));
    }
}

pub unsafe extern "C" fn proxy_on_frame(
    _ptr: uintptr_t,
    chat_id: i64,
    stream_mode: ntg_stream_mode_enum,
    device: ntg_stream_device_enum,
    frame: *mut ntg_frame_struct,
    frame_count: u64,
    user_data: *mut c_void,
) {
    unsafe {
        if user_data.is_null() || frame.is_null() {
            return;
        }
        let frame = *frame;
        let tx = &*(user_data as *mut tokio::sync::mpsc::UnboundedSender<NtgEvent>);
        #[allow(unused_must_use)] tx.send(NtgEvent::Frame(FrameEvent { chat_id, stream_mode, device, frame, frame_count }));
    }
}

pub unsafe extern "C" fn proxy_on_remote_source(
    _ptr: uintptr_t,
    chat_id: i64,
    remote_source: ntg_remote_source_struct,
    user_data: *mut c_void,
) {
    unsafe {
        if user_data.is_null() {
            return;
        }
        let tx = &*(user_data as *mut tokio::sync::mpsc::UnboundedSender<NtgEvent>);
        #[allow(unused_must_use)] tx.send(NtgEvent::RemoteSource(RemoteSourceEvent { chat_id, remote_source }));
    }
}

pub unsafe extern "C" fn proxy_on_broadcast_timestamp(
    _ptr: uintptr_t,
    chat_id: i64,
    user_data: *mut c_void,
) {
    unsafe {
        if user_data.is_null() {
            return;
        }
        let tx = &*(user_data as *mut tokio::sync::mpsc::UnboundedSender<NtgEvent>);
        #[allow(unused_must_use)] tx.send(NtgEvent::BroadcastTimestamp(BroadcastTimestampEvent { chat_id }));
    }
}

pub unsafe extern "C" fn proxy_on_broadcast_part(
    _ptr: uintptr_t,
    chat_id: i64,
    request: ntg_segment_part_request_struct,
    user_data: *mut c_void,
) {
    unsafe {
        if user_data.is_null() {
            return;
        }
        let tx = &*(user_data as *mut tokio::sync::mpsc::UnboundedSender<NtgEvent>);
        #[allow(unused_must_use)] tx.send(NtgEvent::BroadcastPart(BroadcastPartEvent { chat_id, request }));
    }
}