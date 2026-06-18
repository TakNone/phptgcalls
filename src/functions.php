<?php

declare(strict_types = 1);

namespace Tak\Tgcalls;

use Tak\Liveproto\Tl\Types\Other\PhoneCall;

use TgCalls\RtcServer;

function parse_connections(PhoneCall $phoneCall) : array {
	$servers = [];
	foreach($phoneCall->connections as $connection):
		if($connection->getClass() === 'phoneConnectionWebrtc'):
			$servers []= new RtcServer(
				id : $connection->id,
				ipv4 : $connection->ip,
				ipv6 : $connection->ipv6,
				username : $connection->username,
				password : $connection->password,
				port : $connection->port,
				turn : $connection->turn,
				stun : $connection->stun,
				tcp : false,
				peer_tag : strval(null)
			);
		elseif($connection->getClass() === 'phoneConnection'):
			 $servers []= new RtcServer(
				id : $connection->id,
				ipv4 : $connection->ip,
				ipv6 : $connection->ipv6,
				username : strval(null),
				password : strval(null),
				port : $connection->port,
				turn : true,
				stun : false,
				tcp : $connection->tcp,
				peer_tag : $connection->peer_tag
			);
		endif;
	endforeach;
	return $servers;
}

?>