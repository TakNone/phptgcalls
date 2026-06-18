<?php

declare(strict_types = 1);

namespace Tak\Tgcalls\Parsers;

use Tak\Tgcalls\Event;

use Tak\Liveproto\Utils\Binary;

use TgCalls\ConnectionKind;

use TgCalls\ConnectionState;

class Connection {
	public int $chat_id;
	public ConnectionKind $kind;
	public ConnectionState $state;

	public const Event TYPE = Event::Connection;

	public function __construct(Binary $reader){
		$this->chat_id = $reader->readLong();
		$this->kind = ConnectionKind::cases()[$reader->readInt()];
		$this->state = ConnectionState::cases()[$reader->readInt()];
	}
}

?>