<?php

declare(strict_types = 1);

error_reporting(E_ALL);

if(file_exists('vendor/autoload.php')){
	require_once 'vendor/autoload.php';
} elseif(file_exists('liveproto.php') === false){
	copy('https://installer.liveproto.dev/liveproto.php','liveproto.php');
	require_once 'liveproto.php';
} else {
	require_once 'liveproto.phar';
}

use Tak\Liveproto\Network\Client;

use Tak\Liveproto\Utils\Settings;

use Tak\Liveproto\Filters\Filter;
use Tak\Liveproto\Filters\Filter\Update;
use Tak\Liveproto\Filters\Filter\Regex;
use Tak\Liveproto\Filters\Filter\Command;

use Tak\Liveproto\Filters\Events\NewMessage;

use Tak\Liveproto\Filters\Interfaces\Incoming;
use Tak\Liveproto\Filters\Interfaces\IsPrivate;

use Tak\Tgcalls\Driver;

use Tak\Asyncio\Loop;

use function Tak\Asyncio\delay;

$settings = new Settings();
$settings->setApiId(21724);
$settings->setApiHash('3e0cb5efcd52300aec5994fdfc5bdc16');
$settings->setHideLog(false);
$settings->setReceiveUpdates(false);

$calls = array();

#[Filter(new NewMessage(new Command('call')))]
function requestCall(Incoming & IsPrivate $update) : void {
	global $calls;
	$client = $update->getClient();
	$chat_id = $update->getPeerId();
	if(isset($calls[$chat_id]) === false){
		$tgcalls = new Driver(client : $client,chat_id : $chat_id);
		$tgcalls->create(p2p : true);
		$audio = new TgCalls\AudioDescription(
			media_source : TgCalls\MediaSource::FILE,
			input : realpath('./audio.wav'),
			sample_rate : 96000,
			channel_count : 2,
			keep_open: true
		);
		$tgcalls->set_stream(microphone : $audio);
		$handler = $tgcalls->createUpdateHandler();
		$g_a_hash = $tgcalls->init_exchange();
		$protocol = $tgcalls->get_protocol();
		try {
			$client->account->updateStatus(offline : false);
			$request = $client->phone->requestCall(user_id : $client->get_input_user($chat_id),random_id : random_int(0,PHP_INT_MAX),g_a_hash : $g_a_hash,protocol : $protocol);
			$calls[$chat_id] = $request->phone_call->id;
			$update->reply(message : '✅ Calling you...');
		} catch(Throwable){
			$update->reply(message : '❌ The process failed !');
		}
	} else {
		$update->reply(message : '⚠️ A call has already been made to you');
	}
}

#[Filter(new Update('updatePhoneCall'))]
function phoneCalls(object $update) : void {
	global $calls;
	$client = $update->getClient();
	$client->account->updateStatus(offline : false);
	switch($update->phone_call->getClass()){
		case 'phoneCallRequested':
			$chat_id = $update->phone_call->admin_id;
			if(isset($calls[$chat_id]) === false){
				$tgcalls = new Driver(client : $client,chat_id : $chat_id);
				$tgcalls->create(p2p : true);
				$audio = new TgCalls\AudioDescription(
					media_source : TgCalls\MediaSource::FILE,
					input : realpath('./audio.wav'),
					sample_rate : 96000,
					channel_count : 2,
					keep_open: true
				);
				$tgcalls->set_stream(microphone : $audio);
				$handler = $tgcalls->createUpdateHandler();
				$handler->handlePhoneCall($update);
				$calls[$chat_id] = $update->phone_call->id;
			}
			break;
		case 'phoneCallDiscarded':
			foreach($calls as $chat_id => $call_id){
				if($call_id === $update->phone_call->id){
					unset($calls[$chat_id]);
				}
			}
			break;
	}
}

Loop::queue(static function() use($settings) : void {
	try {
		$client = new Client('phptgcalls','sqlite',$settings);
		$client->start();
	} finally {
		$client->stop();
	}
});

Loop::run();

?>