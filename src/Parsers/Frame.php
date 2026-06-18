<?php

declare(strict_types = 1);

namespace Tak\Tgcalls\Parsers;

use Tak\Tgcalls\Event;

use Tak\Liveproto\Utils\Binary;

use TgCalls\StreamMode;

use TgCalls\StreamDevice;

class Frame {
	public int $chat_id;
	public StreamMode $mode;
	public StreamDevice $device;
	public int $ssrc;
	public string $frame_data;
	public int $timestamp;
	public int $width;
	public int $height;
	public int $rotation;
	public int $count;

	public const Event TYPE = Event::Frame;

	public function __construct(Binary $reader){
		$this->chat_id = $reader->readLong();
		$this->mode = StreamMode::cases()[$reader->readInt()];
		$this->device = StreamDevice::cases()[$reader->readInt()];
		$this->ssrc = $reader->readLong();
		$length = $reader->readInt();
		$this->frame_data = $reader->read($length);
		$struct = $reader->read(8 + 2 + 2 + 2);
		if($result = @unpack('Ptimestamp/vwidth/vheight/vrotation',$struct)):
			$this->timestamp = $result['timestamp'];
			$this->width = $result['width'];
			$this->height = $result['height'];
			$this->rotation = $result['rotation'];
		endif;
		$this->count = $reader->readLong();
	}
}

?>