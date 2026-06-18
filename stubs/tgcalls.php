<?php

function tgcalls_get_protocol() : array {}

function tgcalls_get_media_devices() : array {}

function tgcalls_get_version() : string {}

function tgcalls_enable_glib_loop(bool $enable) : bool {}

function tgcalls_set_log_level(int $mask) : void {}

class TgcallsClient {
	public function __construct(int $peer_id){}

	public function create(bool $p2p = false,int $timeout = 10) : string {}

	public function set_stream(int $stream_mode,TgCalls\MediaDescription $desc,int $timeout = 10) : bool {}

	public function connect(string $params_json,bool $presentation = false,int $timeout = 10) : bool {}

	public function init_exchange(TgCalls\DhConfig $dh_config,? string $g_a_hash = null,int $timeout = 10) : string {}

	public function exchange_keys(string $g_a_or_b,? int $fingerprint = null,int $timeout = 10) : TgCalls\AuthParams {}

	public function skip_exchange(string $encryption_key,bool $is_outgoing,int $timeout = 10) : bool {}

	public function connect_peer(array $servers,array $protocol_versions,bool $p2p_allowed,int $timeout = 10) : bool {}

	public function send_signaling_data(string $data,int $timeout = 10) : bool {}

	public function init_presentation(int $timeout = 10) : array {}

	public function add_incoming_video(string $endpoint,array $ssrc_groups,int $timeout = 10) : int {}

	public function remove_incoming_video(string $endpoint,int $timeout = 10) : bool {}

	public function send_external_frame(int $device,string $frame,TgCalls\FrameData $frame_data,int $timeout = 10) : bool {}

	public function send_broadcast_timestamp(int $timestamp,int $timeout = 10) : bool {}

	public function send_broadcast_part(int $segment_id,int $part_id,int $status,bool $quality_update,string $frame,int $timeout = 10) : bool {}

	public function stop(bool $presentation = false,int $timeout = 10) : bool {}

	public function pause(int $timeout = 10) : bool {}

	public function resume(int $timeout = 10) : bool {}

	public function mute(int $timeout = 10) : bool {}

	public function unmute(int $timeout = 10) : bool {}

	public function cpu(int $timeout = 10) : float {}

	public function connection_mode(int $timeout = 10) : int {}

	public function calls(int $timeout = 10) : array {}

	public function time(int $stream_mode,int $timeout = 10) : int {}

	public function state(int $timeout = 10) : TgCalls\MediaState {}
}

class TgcallsEvents {
	public function __construct(TgcallsClient $client){}

	public function enable_stream_updates() : int {}

	public function enable_upgrade_updates() : int {}

	public function enable_connection_updates() : int {}

	public function enable_signaling_updates() : int {}

	public function enable_frame_updates() : int {}

	public function enable_remote_source_updates() : int {}

	public function enable_broadcast_timestamp_updates() : int {}

	public function enable_broadcast_part_updates() : int {}

	public function get_fd() : int {}
}

?>