<?php

declare(strict_types = 1);

namespace Tak\Tgcalls\Parsers;

use Tak\Tgcalls\Event;

use Tak\Liveproto\Utils\Binary;

use TgCalls\MediaSegmentQuality;

class BroadcastPart {
	public int $chat_id;
	public int $segment_id;
	public int $part_id;
	public int $limit;
	public int $timestamp;
	public bool $quality_update;
	public int $channel_id;
	public MediaSegmentQuality $quality;

	public const Event TYPE = Event::BroadcastPartEvent;

	public function __construct(Binary $reader){
		$this->chat_id = $reader->readLong();
		$this->segment_id = $reader->readLong();
		$this->part_id = $reader->readInt();
		$this->limit = $reader->readInt();
		$this->timestamp = $reader->readLong();
		$this->quality_update = boolval($reader->readByte());
		$this->channel_id = $reader->readInt();
		$this->quality = MediaSegmentQuality::cases()[$reader->readInt()];
	}
}

?>