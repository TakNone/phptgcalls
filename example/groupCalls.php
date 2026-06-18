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
use Tak\Liveproto\Filters\Filter\Regex;
use Tak\Liveproto\Filters\Filter\Command;

use Tak\Liveproto\Filters\Events\NewMessage;

use Tak\Liveproto\Filters\Interfaces\Incoming;
use Tak\Liveproto\Filters\Interfaces\IsSuperGroup;

use Tak\Tgcalls\Driver;

use Tak\Asyncio\Loop;

use function Tak\Asyncio\delay;

$settings = new Settings();
$settings->setApiId(21724);
$settings->setApiHash('3e0cb5efcd52300aec5994fdfc5bdc16');
$settings->setHideLog(false);
$settings->setReceiveUpdates(false);

$calls = array();

#[Filter(new NewMessage(new Command('join')))]
function joinCall(Incoming & IsSuperGroup $update) : void {
	global $calls;
	$client = $update->getClient();
	$call = $client->get_full_peer($update->message->peer_id)->call;
	if(is_null($call) === false){
		$chat_id = $update->getPeerId();
		$tgcalls = new Driver(client : $client,chat_id : $chat_id);
		$params_json = $tgcalls->create();
		$audio = new TgCalls\AudioDescription(
			media_source : TgCalls\MediaSource::FILE,
			input : realpath('./audio.wav'),
			sample_rate : 96000,
			channel_count : 2,
			keep_open: true
		);
		$tgcalls->set_stream(microphone : $audio);
		$result = $client->phone->joinGroupCall(call : $call,join_as : $client->get_input_peer('me'),params : $client->dataJSON(data : $params_json));
		foreach($result->updates as $update){
			if($update->getClass() === 'updateGroupCallConnection'){
				$tgcalls->connect(params_json : $update->params->data);
				$calls[$chat_id] = $tgcalls;
				$update->reply(message : '✅ We are successfully broadcasting audio in the group call');
				return;
			}
		}
		$update->reply(message : '❌ The process failed !');
	} else {
		$update->reply(message : '⚠️ No calls have been made in this group...');
	}
}

#[Filter(new NewMessage(new Command('play')))]
function playCall(Incoming & IsSuperGroup $update) : void {
	global $calls;
	$client = $update->getClient();
	$chat_id = $update->getPeerId();
	$tgcalls = $calls[$chat_id] ?? null;
	if(is_null($tgcalls) === false){
		$video = new TgCalls\VideoDescription(
			media_source : TgCalls\MediaSource::SHELL,
			input : 'ffmpeg -i http://docs.evostream.com/sample_content/assets/sintel1m720p.mp4 -f rawvideo -r 30 -pix_fmt yuv420p -vf scale=1280:720 pipe:1',
			width : 1280,
			height : 720,
			fps : 30,
			keep_open : true
		);
		$audio = new TgCalls\AudioDescription(
			media_source : TgCalls\MediaSource::SHELL,
			input : 'ffmpeg -i http://docs.evostream.com/sample_content/assets/sintel1m720p.mp4 -f s16le -ac 2 -ar 96k pipe:1',
			sample_rate : 96000,
			channel_count : 2,
			keep_open : true
		);
		$tgcalls->set_stream(microphone : $audio,camera : $video);
		$update->reply(message : '✅ We are re-broadcasting...');
	} else {
		$update->reply(message : '⚠️ The bot has not yet joined call...');
	}
}

#[Filter(new NewMessage(new Command('state')))]
function stateCall(Incoming & IsSuperGroup $update) : void {
	global $calls;
	$client = $update->getClient();
	$chat_id = $update->getPeerId();
	$tgcalls = $calls[$chat_id] ?? null;
	if(is_null($tgcalls) === false){
		$state = $tgcalls->state();
		$cpu = $tgcalls->cpu();
		$update->reply(message : '✨ State Call : '.chr(10).
														'Muted : '.($state->muted ? '✅' : '❌').chr(10).
														chr(10).
														'Video Paused : '.($state->video_paused ? '✅' : '❌').chr(10).
														'Video Stopped : '.($state->video_stopped ? '✅' : '❌').chr(10).
														'Presentation Paused : '.($state->presentation_paused ? '✅' : '❌').chr(10).
														chr(10).
														'Cpu usage : '.$cpu);
	} else {
		$update->reply(message : '⚠️ The bot has not yet joined call...');
	}
}

#[Filter(new NewMessage(new Regex('~^/(?<feature>pause|resume|mute|unmute)$~i')))]
function toggleFeature(Incoming & IsSuperGroup $update) : void {
	global $calls;
	$client = $update->getClient();
	$chat_id = $update->getPeerId();
	$tgcalls = $calls[$chat_id] ?? null;
	if(is_null($tgcalls) === false){
		$feature = strtolower($update->regex->matched['feature']);
		if(call_user_func(array($tgcalls,$feature))){
			$update->reply(message : '✅ Successfully '.$feature.'d');
		} else {
			$update->reply(message : '❌ The process failed !');
		}
	} else {
		$update->reply(message : '⚠️ The bot has not yet joined call...');
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