<?php

declare(strict_types = 1);

namespace Tak\Tgcalls;

use Tak\Liveproto\Network\Client;

use Tak\Liveproto\Utils\Binary;

use Tak\Asyncio\Loop;

use TgcallsClient;

use TgcallsEvents;

use Closure;

use Stringable;

abstract class Wrapper implements Stringable {
	private Closure $cancel;
	private int $fd;

	abstract protected function emit(Event $event,? object $struct = null) : void;

	public function __construct(public readonly TgcallsClient $tgcalls,private readonly TgcallsEvents $events,? array $enabledEvents){
		if(is_null($enabledEvents)):
			$enabledEvents = Event::cases();
		else:
			$enabledEvents = array_values(array_filter($enabledEvents,static fn(mixed $event) : bool => $event instanceof Event));
		endif;
		foreach($enabledEvents as $enabledEvent):
			call_user_func(array($events,'enable_'.$enabledEvent->value.'_updates'));
		endforeach;
		$this->fd = $this->events->get_fd();
		if($stream = @fopen('php://fd/'.$this->fd,'r+')):
			$readerId = $this->pipe($stream);
			$this->cancel = static function() use($stream,$readerId) : void {
				Loop::cancel($readerId);
				@fclose($stream);
			};
		else:
			throw new \RuntimeException('Failed to open stream');
		endif;
	}
	private function pipe(mixed $stream) : string {
		@stream_set_blocking($stream,false);
		return Loop::onReadable($stream,function(string $watcher,mixed $stream) : void {
			$bytes = stream_get_contents($stream);
			if($bytes and strlen($bytes) > 8):
				$reader = new Binary();
				$reader->write($bytes);
				do {
					$update = match($reader->readByte()){
						1 => new Parsers\Stream($reader),
						2 => new Parsers\Upgrade($reader),
						3 => new Parsers\Connection($reader),
						4 => new Parsers\Signaling($reader),
						5 => new Parsers\Frame($reader),
						6 => new Parsers\RemoteSource($reader),
						7 => new Parsers\BroadcastTimestamp($reader),
						8 => new Parsers\BroadcastPart($reader),
						default => Loop::cancel($watcher)
					};
					$this->emit($update::TYPE,$update);
				} while($reader->tellLength() - $reader->tellPosition() > 8);
			endif;
		});
	}
	public function __toString() : string {
		return strval($this->fd);
	}
	public function __destruct(){
		call_user_func($this->cancel);
	}
}

?>