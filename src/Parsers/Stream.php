<?php

declare(strict_types = 1);

namespace Tak\Tgcalls\Parsers;

use Tak\Tgcalls\Event;

use Tak\Liveproto\Utils\Binary;

use TgCalls\StreamType;

use TgCalls\StreamDevice;

class Stream {
	public int $chat_id;
	public StreamType $type;
	public StreamDevice $device;

	public const Event TYPE = Event::Stream;

	public function __construct(Binary $reader){
		$this->chat_id = $reader->readLong();
		$this->type = StreamType::cases()[$reader->readInt()];
		$this->device = StreamDevice::cases()[$reader->readInt()];
	}
}

?>