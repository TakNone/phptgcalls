<?php

declare(strict_types = 1);

namespace Tak\Tgcalls;

enum Event : string {
	case Stream = 'stream';
	case Upgrade = 'upgrade';
	case Connection = 'connection';
	case Signaling = 'signaling';
	case Frame = 'frame';
	case RemoteSource = 'remote_source';
	case BroadcastTimestamp = 'broadcast_timestamp';
	case BroadcastPart = 'broadcast_part';
}

?>