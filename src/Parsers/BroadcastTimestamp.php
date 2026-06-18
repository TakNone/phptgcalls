<?php

declare(strict_types = 1);

namespace Tak\Tgcalls\Parsers;

use Tak\Tgcalls\Event;

use Tak\Liveproto\Utils\Binary;

class BroadcastTimestamp {
	public int $chat_id;

	public const Event TYPE = Event::BroadcastTimestamp;

	public function __construct(Binary $reader){
		$this->chat_id = $reader->readLong();
	}
}

?>