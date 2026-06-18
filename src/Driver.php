<?php

declare(strict_types = 1);

namespace Tak\Tgcalls;

use Tak\Liveproto\Network\Client;

use Tak\Asyncio\Loop;

use TgCalls\DhConfig;

use TgCalls\StreamMode;

use TgCalls\AudioDescription;

use TgCalls\VideoDescription;

use TgCalls\MediaDescription;

use TgcallsClient;

use TgcallsEvents;

/**
 * @mixin TgcallsClient
 */
final class Driver extends Wrapper {
	private array $listeners = array();

	public function __construct(public readonly Client $client,public readonly int $chat_id,? array $enabledEvents = null){
		if($client->connected === false){
			throw new \InvalidArgumentException('Client is not connected');
		}
		$tgcalls = new TgcallsClient($chat_id);
		$events = new TgcallsEvents($tgcalls);
		parent::__construct($tgcalls,$events,$enabledEvents);
	}
	public function set_stream(StreamMode $stream_mode = StreamMode::CAPTURE,? AudioDescription $microphone = null,? AudioDescription $speaker = null,? VideoDescription $camera = null,? VideoDescription $screen = null) : bool {
		$desc = new MediaDescription(microphone : $microphone,speaker : $speaker,camera : $camera,screen : $screen);
		return $this->tgcalls->set_stream(stream_mode : $stream_mode,desc : $desc);
	}
	public function get_dh_config() : object {
		$dh = $this->client->messages->getDhConfig(version : 0,random_length : 256);
		return new DhConfig(g : $dh->g,p : $dh->p,random : $dh->random);
	}
	public function init_exchange(? string $g_a_hash = null) : string {
		return $this->tgcalls->init_exchange(dh_config : $this->get_dh_config(),g_a_hash : $g_a_hash);
	}
	public function get_protocol() : object {
		return $this->client->phoneCallProtocol(...tgcalls_get_protocol());
	}
	public function createUpdateHandler() : object {
		return new HandleUpdates(driver : $this);
	}
	public function on(Event $event,callable $callback) : void {
		$this->listeners[$event->value][] = $callback;
	}
	protected function emit(Event $event,? object $struct = null) : void {
		$callbacks = $this->listeners[$event->value] ?? array();
		foreach($callbacks as $callback):
			Loop::queue($callback(...),$struct);
		endforeach;
	}
	public function __call(string $name,array $arguments) : mixed {
		return call_user_func_array([$this->tgcalls,$name],$arguments);
	}
}

?>