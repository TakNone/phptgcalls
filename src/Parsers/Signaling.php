<?php

declare(strict_types = 1);

namespace Tak\Tgcalls\Parsers;

use Tak\Tgcalls\Event;

use Tak\Liveproto\Utils\Binary;

class Signaling {
	public int $chat_id;
	public string $data;

	public const Event TYPE = Event::Signaling;

	public function __construct(Binary $reader){
		$this->chat_id = $reader->readLong();
		$length = $reader->readInt();
		$this->data = $reader->read($length);
	}
}

?>