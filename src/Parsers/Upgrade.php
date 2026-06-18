<?php

declare(strict_types = 1);

namespace Tak\Tgcalls\Parsers;

use Tak\Tgcalls\Event;

use Tak\Liveproto\Utils\Binary;

class Upgrade {
	public int $chat_id;
	public bool $muted;
	public bool $video_paused;
	public bool $video_stopped;
	public bool $presentation_paused;

	public const Event TYPE = Event::Upgrade;

	public function __construct(Binary $reader){
		$this->chat_id = $reader->readLong();
		$this->muted = boolval($reader->readByte());
		$this->video_paused = boolval($reader->readByte());
		$this->video_stopped = boolval($reader->readByte());
		$this->presentation_paused = boolval($reader->readByte());
	}
}

?>