<?php

declare(strict_types = 1);

namespace Tak\Tgcalls;

use Tak\Liveproto\Utils\Logging;

use Tak\Liveproto\Errors\RpcError;

use Tak\Liveproto\Filters\Filter\Update;

use TgCalls\MediaSegmentQuality;

use TgCalls\MediaSegmentStatus;

use TgCalls\ConnectionState;

use function Tak\Tgcalls\parse_connections;

final class HandleUpdates {
	public array $calls = array();

	public function __construct(public private(set) Driver $driver){
		$driver->client->addHandler($this->handlePhoneCall(...),strval('phoneCall'.spl_object_hash($driver)),new Update('updatePhoneCall'));
		$driver->client->addHandler($this->handleSignalingData(...),strval('signalingData'.spl_object_hash($driver)),new Update('updatePhoneCallSignalingData'));
		$driver->on(Event::Connection,$this->eventConnection(...));
		$driver->on(Event::Signaling,$this->eventSignalingData(...));
		$driver->on(Event::BroadcastTimestamp,$this->eventBroadcastTimestamp(...));
		$driver->on(Event::BroadcastPart,$this->eventBroadcastPart(...));
	}
	public function handlePhoneCall(object $update) : void {
		switch($update->phone_call->getClass()):
			case 'phoneCallWaiting':
				$user_id = $update->phone_call->participant_id;
				if($user_id === $this->driver->chat_id):
					$inputPhoneCall = $this->driver->client->inputPhoneCall(id : $update->phone_call->id,access_hash : $update->phone_call->access_hash);
					$this->calls[$user_id] = $inputPhoneCall;
				endif;
				break;
			case 'phoneCallRequested':
				$user_id = $update->phone_call->admin_id;
				if($user_id === $this->driver->chat_id):
					$inputPhoneCall = $this->driver->client->inputPhoneCall(id : $update->phone_call->id,access_hash : $update->phone_call->access_hash);
					$g_b = $this->driver->init_exchange(g_a_hash : $update->phone_call->g_a_hash);
					$this->driver->client->phone->acceptCall(peer : $inputPhoneCall,g_b : $g_b,protocol : $this->driver->get_protocol());
					$this->calls[$user_id] = $inputPhoneCall;
				endif;
				break;
			case 'phoneCallAccepted':
				$user_id = $update->phone_call->participant_id;
				if($user_id === $this->driver->chat_id):
					$inputPhoneCall = $this->driver->client->inputPhoneCall(id : $update->phone_call->id,access_hash : $update->phone_call->access_hash);
					$auth_params = $this->driver->tgcalls->exchange_keys(g_a_or_b : $update->phone_call->g_b,fingerprint : null);
					$phone = $this->driver->client->phone->confirmCall(peer : $inputPhoneCall,g_a : $auth_params->g_a_or_b,key_fingerprint : $auth_params->key_fingerprint,protocol : $this->driver->get_protocol());
					$servers = parse_connections($phone->phone_call);
					$this->driver->tgcalls->connect_peer(servers : $servers,protocol_versions : $phone->phone_call->protocol->library_versions,p2p_allowed : $phone->phone_call->p2p_allowed);
					$this->calls[$user_id] = $inputPhoneCall;
				endif;
				break;
			case 'phoneCall':
				$user_id = $update->phone_call->admin_id;
				if($user_id === $this->driver->chat_id):
					$inputPhoneCall = $this->driver->client->inputPhoneCall(id : $update->phone_call->id,access_hash : $update->phone_call->access_hash);
					$this->driver->tgcalls->exchange_keys(g_a_or_b : $update->phone_call->g_a_or_b,fingerprint : $update->phone_call->key_fingerprint);
					$servers = parse_connections($update->phone_call);
					$this->driver->tgcalls->connect_peer(servers : $servers,protocol_versions : $update->phone_call->protocol->library_versions,p2p_allowed : $update->phone_call->p2p_allowed);
					$this->calls[$user_id] = $inputPhoneCall;
				endif;
				break;
			case 'phoneCallDiscarded':
				$user_id = $this->getInfo($update->phone_call->id);
				if($user_id === $this->driver->chat_id):
					$this->driver->tgcalls->stop();
				endif;
				unset($this->calls[$user_id]);
				break;
			default:
				break;
		endswitch;
	}
	private function handleSignalingData(object $update) : void {
		$user_id = $this->getInfo($update->phone_call_id);
		if($user_id === $this->driver->chat_id):
			$this->driver->tgcalls->send_signaling_data($update->data);
		endif;
	}
	private function eventConnection(object $event) : void {
		if($event->chat_id === $this->driver->chat_id and array_key_exists($this->driver->chat_id,$this->calls)):
			switch($event->state):
				case ConnectionState::CONNECTING:
				case ConnectionState::CONNECTED:
					Logging::log('Handle Updates','Connection CONNECTING / CONNECTED',E_NOTICE);
					break;
				case ConnectionState::TIMEOUT:
					Logging::log('Handle Updates','Connection TIMEOUT',E_WARNING);
					break;
				case ConnectionState::FAILED:
				case ConnectionState::CLOSED:
					Logging::log('Handle Updates','Connection FAILED / CLOSED',E_ERROR);
					unset($this->calls[$event->chat_id]);
					$this->driver->tgcalls->stop();
					break;
			endswitch;
		endif;
	}
	private function eventSignalingData(object $event) : void {
		if($event->chat_id === $this->driver->chat_id and array_key_exists($this->driver->chat_id,$this->calls)):
			$this->driver->client->phone->sendSignalingData(peer : $this->calls[$event->chat_id],data : $event->data);
		endif;
	}
	private function eventBroadcastTimestamp(object $event) : void {
		if($event->chat_id === $this->driver->chat_id and array_key_exists($this->driver->chat_id,$this->calls)):
			$groupCallStream = $this->driver->client->phone->getGroupCallStreamChannels(call : $this->calls[$event->chat_id]);
			$last_timestamp = reset($groupCallStream->channels)->last_timestamp_ms;
			$this->driver->tgcalls->send_broadcast_timestamp($last_timestamp);
		endif;
	}
	private function eventBroadcastPart(object $event) : void {
		if($event->chat_id === $this->driver->chat_id and array_key_exists($this->driver->chat_id,$this->calls)):
			try {
				$frame = strval(null);
				$location = $this->driver->client->inputGroupCallStream(
					call : $this->calls[$event->chat_id],
					time_ms : $event->timestamp,
					video_channel : $event->channel_id,
					video_quality : match($event->quality){
						MediaSegmentQuality::THUMBNAIL => 0,
						MediaSegmentQuality::MEDIUM => 1,
						MediaSegmentQuality::FULL => 2,
						default => 0
					},
					scale : 0
				);
				$status = MediaSegmentStatus::SUCCESS;
				$file = $this->driver->client->upload->getFile(location : $location,offset : 0,limit : $event->limit,timeout : 60);
				$frame = $file->bytes;
			} catch(RpcError $error){
				if($error->getCode() == 420):
					if($error->getValue() === 0):
						$status = MediaSegmentStatus::RESYNC_NEEDED;
					else:
						$status = MediaSegmentStatus::NOT_READY;
					endif;
				endif;
			}
			$this->driver->tgcalls->send_broadcast_part($event->segment_id,$event->part_id,$status,$event->quality_update,$frame);
		endif;
	}
	public function getInfo(int $phone_call_id) : int {
		foreach($this->calls as $user_id => $phoneCall):
			if($phone_call_id === $phoneCall->id):
				return $user_id;
			endif;
		endforeach;
		throw new \Error('Phone call id '.$phone_call_id.' not found');
	}
	public function __destruct(){
		$this->driver->client->removeHandler($this->handlePhoneCall(...),strval('phoneCall'.spl_object_hash($this->driver)));
		$this->driver->client->removeHandler($this->handleSignalingData(...),strval('signalingData'.spl_object_hash($this->driver)));
	}
}

?>