<?php

declare(strict_types = 1);

namespace Tak\Tgcalls\Parsers;

use Tak\Tgcalls\Event;

use Tak\Liveproto\Utils\Binary;

use TgCalls\StreamStatus;

use TgCalls\StreamDevice;

class RemoteSource {
	public int $chat_id;
	public int $ssrc;
	public StreamStatus $state;
	public StreamDevice $device;

	public const Event TYPE = Event::RemoteSource;

	public function __construct(Binary $reader){
		$this->chat_id = $reader->readLong();
		$this->ssrc = $reader->readInt();
		$this->state = StreamStatus::cases()[$reader->readInt()];
		$this->device = StreamDevice::cases()[$reader->readInt()];
	}
}

?>