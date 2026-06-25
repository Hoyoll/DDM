import {start, send_queue, pause_play} from './play.js'

type Message = 
	| { key: "READY" }
	| { key: "CONFIG", field: string }
	| { key: "CONFIGERR", field: string }
	| { key: "QUEUE", field: {
			id: number,
			songs: {
				name: string;
				url: string;
			}[]
		}
	}
	| { key: "HIDEPLAY"}

	
declare global {
	interface Window {
		send(msg: Message): void
		receive(msg: Message): void
		hydrate(url: string): void
		swap_player(url: string, index: number): void
		ipc: {
			postMessage(data: any): void
		}
	}
}

start()
window.receive = (msg) => {
	switch (msg.key) {
		case 'QUEUE':
			send_queue(msg.field)
			break
		case 'HIDEPLAY':
			pause_play()
			break
	}
}
