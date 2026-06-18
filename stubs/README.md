## 📖 API Reference

### Global Functions

| Function | Return | Description |
|----------|--------|-------------|
| `tgcalls_get_protocol()` | `array` | Get protocol information |
| `tgcalls_get_media_devices()` | `array` | List available media devices |
| `tgcalls_get_version()` | `string` | Extension version string |
| `tgcalls_enable_glib_loop(bool)` | `bool` | Toggle GLib event loop |
| `tgcalls_set_log_level(int)` | `void` | Set log verbosity mask |

---

### TgcallsClient

| Method | Return | Description |
|--------|--------|-------------|
| `__construct(int $peer_id)` | — | Initialize client for a Telegram peer |
| `create(bool $p2p, int $timeout)` | `string` | Generate call parameters JSON |
| `connect(string $params_json, bool $presentation, int $timeout)` | `bool` | Connect to call with params |
| `set_stream(int $mode, MediaDescription $desc, int $timeout)` | `bool` | Start streaming media |
| `init_exchange(DhConfig $dh, ?string $g_a_hash, int $timeout)` | `string` | Init DH key exchange |
| `exchange_keys(string $g_a_or_b, ?int $fingerprint, int $timeout)` | `AuthParams` | Complete key exchange |
| `skip_exchange(string $key, bool $outgoing, int $timeout)` | `bool` | Skip exchange with known key |
| `connect_peer(array $servers, array $versions, bool $p2p, int $timeout)` | `bool` | Connect to peer servers |
| `send_signaling_data(string $data, int $timeout)` | `bool` | Send signaling data |
| `init_presentation(int $timeout)` | `array` | Initialize screen sharing |
| `add_incoming_video(string $endpoint, array $ssrc, int $timeout)` | `int` | Add incoming video stream |
| `remove_incoming_video(string $endpoint, int $timeout)` | `bool` | Remove incoming video |
| `send_external_frame(int $device, string $frame, FrameData $data, int $timeout)` | `bool` | Send raw video frame |
| `send_broadcast_timestamp(int $ts, int $timeout)` | `bool` | Send broadcast timestamp |
| `send_broadcast_part(int $seg, int $part, int $status, bool $quality, string $frame, int $timeout)` | `bool` | Send broadcast segment |
| `stop(bool $presentation, int $timeout)` | `bool` | Stop the call |
| `pause(int $timeout)` | `bool` | Pause media stream |
| `resume(int $timeout)` | `bool` | Resume media stream |
| `mute(int $timeout)` | `bool` | Mute microphone |
| `unmute(int $timeout)` | `bool` | Unmute microphone |
| `cpu(int $timeout)` | `float` | Get CPU usage |
| `connection_mode(int $timeout)` | `int` | Get current connection mode |
| `calls(int $timeout)` | `array` | List active calls |
| `time(int $stream_mode, int $timeout)` | `int` | Get stream time position |
| `state(int $timeout)` | `MediaState` | Get current media state |

---

### TgcallsEvents

| Method | Return | Description |
|--------|--------|-------------|
| `__construct(TgcallsClient $client)` | — | Bind events to a client |
| `enable_stream_updates()` | `int` | Enable stream state events |
| `enable_upgrade_updates()` | `int` | Enable connection upgrade events |
| `enable_connection_updates()` | `int` | Enable connection state events |
| `enable_signaling_updates()` | `int` | Enable signaling data events |
| `enable_frame_updates()` | `int` | Enable video frame events |
| `enable_remote_source_updates()` | `int` | Enable remote source events |
| `enable_broadcast_timestamp_updates()` | `int` | Enable broadcast timestamp events |
| `enable_broadcast_part_updates()` | `int` | Enable broadcast part events |
| `get_fd()` | `int` | Get event file descriptor (for async I/O) |

---

### Core Data Classes

| Class | Description | Key Arguments/Properties |
| :--- | :---: | ---: |
| `TgCalls\NetworkInfo` | Contains status information about the current P2P/RTC connection. | `kind` (ConnectionKind), `state` (ConnectionState) |
| `TgCalls\AudioDescription` | Configures audio streams (microphone/speaker). | `media_source`, `input` (path/dev), `sample_rate`, `channel_count`, `keep_open` |
| `TgCalls\VideoDescription` | Configures video streams (camera/screen). | `media_source`, `input`, `width`, `height`, `fps`, `keep_open` |
| `TgCalls\MediaDescription` | Container holding all active audio/video stream configurations. | `microphone`, `speaker`, `camera`, `screen` (all optional objects) |
| `TgCalls\CallInfo` | Provides high-level status for a specific group call. | `chat_id`, `capture` (status), `playback` (status) |
| `TgCalls\MediaState` | Represents the current muting and pause state of a session. | `muted`, `video_paused`, `video_stopped`, `presentation_paused` |
| `TgCalls\RtcServer` | Configuration for STUN/TURN servers used in connection establishment. | `id`, `ipv4`, `ipv6`, `port`, `turn`, `stun`, `tcp`, `username`, `password`, `peer_tag` |
| `TgCalls\Protocol` | Details about the supported protocol versions and UDP capabilities. | `min_layer`, `max_layer`, `udp_p2p`, `udp_reflector`, `library_versions` |
| `TgCalls\DhConfig` | Diffie-Hellman key exchange parameters for secure communication. | `g`, `p` (binary), `random` (binary) |
| `TgCalls\FrameData` | Metadata associated with a video or audio frame. | `absolute_capture_timestamp_ms`, `width`, `height`, `rotation` |
| `TgCalls\RemoteSource` | Describes a source received from a remote participant. | `ssrc`, `state`, `device` |
| `TgCalls\SsrcGroup` | Groups of Synchronization Source identifiers for media tracks. | `semantics`, `ssrcs` (array of IDs) |
| `TgCalls\DeviceInfo` | Metadata for identified hardware media devices. | `name`, `metadata` |
| `TgCalls\MediaDevices` | A collection of all detected hardware devices. | `microphone`, `speaker`, `camera`, `screen` (arrays of `DeviceInfo`) |
| `TgCalls\Frame` | Represents a raw data frame in the stream. | `ssrc`, `data` (binary), `frame_data` (`FrameData` object) |
| `TgCalls\SegmentPartRequest` | Requests a specific segment/part of a broadcast. | `segment_id`, `part_id`, `limit`, `timestamp`, `quality_update`, `channel_id`, `quality` |
| `TgCalls\AuthParams` | Authentication parameters for key exchange. | `g_a_or_b` (binary), `key_fingerprint` |
| `TgCalls\LogMessage` | Structured log output from the C++ core. | `level`, `source`, `file`, `line`, `message` |

---

### Enumerations (`TgCalls`)

These enums are used throughout the classes above to control behavior  :

* **`ErrorCode`** : Covers `NTG_ERROR_...` codes (Connection, Crypto, FFMPEG, MediaDevice, WebRTC, etc.)
* **`MediaSource`** : Flags for stream origins : `FILE`, `SHELL`, `FFMPEG`, `DEVICE`, `DESKTOP`, `EXTERNAL`
* **`StreamDevice`** : Specifies `MICROPHONE`, `SPEAKER`, `CAMERA`, or `SCREEN`
* **`StreamMode`** : Either `CAPTURE` (sending) or `PLAYBACK` (receiving)
* **`StreamType`** : `AUDIO` or `VIDEO`
* **`StreamStatus`** : `ACTIVE`, `PAUSED`, or `IDLING`
* **`ConnectionState`** : `CONNECTING`, `CONNECTED`, `TIMEOUT`, `FAILED`, `CLOSED`
* **`ConnectionKind`** : `NORMAL` or `PRESENTATION`
* **`MediaSegmentQuality`** : `NONE`, `THUMBNAIL`, `MEDIUM`, or `FULL`
* **`MediaSegmentStatus`** : `NOT_READY`, `RESYNC_NEEDED`, or `SUCCESS`
* **`ConnectionMode`** : `NONE`, `RTC`, `STREAM`, or `RTMP`
* **`LogLevel`** : `DEBUG`, `INFO`, `WARNING`, `ERROR`, `UNKNOWN`
* **`LogSource`** : `WEBRTC_LOG` or `SELF_LOG`

---

> [!TIP]
> The Rust bridge uses `ext-php-rs` to map these structs to native PHP objects. Note that for fields involving pointers (like `*mut c_char` or binary data), the PHP implementation provides `getter` methods that safely convert the C-pointers back into native PHP `String` or `Binary` types, and `Drop` implementations ensure that the memory allocated by `CString` in Rust is correctly freed when the PHP object is garbage collected