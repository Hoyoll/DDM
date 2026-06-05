import './style.css'
import 'player.style/tailwind-audio';

type Message = 
	| { key: "READY" }
	| { key: "CONFIG", field: string }
	| { key: "CONFIGERR", field: string }

type Source = {
	name: string,
	url: string,
	token: string,
}

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

let c_token = ""
let q_cache: QCache = new Map()
let history_stack: string[] = []
let source_list: Source[] = []

let html: HTMLElement = document.querySelector("#app")!
let player: HTMLAudioElement = document.querySelector("#player")!
let song_title: HTMLElement = document.querySelector(".song-title")!


window.send = (msg) => {
	window.ipc.postMessage(JSON.stringify(msg))
}

window.receive = (msg) => {
	switch (msg.key) {
		case "CONFIG":
			source_list = JSON.parse(msg.field) as Source[]
			bootstrap(source_list)
			break
		case "CONFIGERR":
			err(msg.field)
			break
	}
}

export type Contents = 
	| { type: "file", name: string, download_url: string }
	| { type: "dir", name: string, url: string }

type QCache = Map<string, {
	name: string;
	url: string;
}[]>

function err(err: string) {
	html.replaceChildren()
	html.appendChild(err_component(`Error: ${err}`))
	html.appendChild(err_component(`Also make sure that the json follow this schema!:`))
	html.appendChild(err_component(`[`))
	html.appendChild(err_component(`{ "name": string, "url": string, "token": string }`))
	html.appendChild(err_component(`]`))
}

function err_component(err: string): HTMLLIElement {
	const li = document.createElement("li")

    const span = document.createElement("span")
    span.className = "file"
    span.textContent = err

    li.appendChild(span)

    return li 
}

function dir_component(url: string, name: string): HTMLLIElement {
	const li = document.createElement("li")

    const span = document.createElement("span")
    span.className = "dir"
    span.textContent = `~${name}/`

    span.addEventListener("click", () => {
        hydrate(url)
    })

    li.appendChild(span)

    return li
}

function bootstrap(sources: Source[]) {
	html.replaceChildren()
	for (const source of sources) {
		html.appendChild(repo_component(source))
	}	
}

function repo_component(source: Source): HTMLLIElement {
	const li = document.createElement("li")

    const span = document.createElement("span")
    span.className = "dir"
    span.textContent = `~${source.name}/`

    span.addEventListener("click", () => {
		c_token = source.token
        hydrate(source.url)
    })

    li.appendChild(span)

    return li
}

function file_component(url: string, name: string, index: number): HTMLLIElement {
	const li = document.createElement("li")

    const span = document.createElement("span")
    span.className = "file"
    span.textContent = `~${name}`

    span.addEventListener("click", () => {
        swap_player(url, index)
    })

    li.appendChild(span)

    return li
}

function hydrate(url: string) {
	html.replaceChildren()
	html.appendChild(root(history_stack))
	html.appendChild(reload(url, history_stack))
	html.appendChild(back(history_stack))
	history_stack.push(url)
	html.insertAdjacentHTML('beforeend', loader())
	fetch(url, {
		headers: {
			Authorization: "Bearer " + c_token,
		}
	})
	.then((res) => res.json())
	.then((json) => {
	html.removeChild(html.lastElementChild!)
	let arr: {
		name: string;
		url: string;
	}[] = []
	for (const [index, item] of json.entries()) {
		item as Contents
		switch (item.type) {
			case "file":
				arr.push({name: item.name, url: item.download_url})
				html.appendChild(file_component(url, item.name, index))
				break
			case "dir":  
				html.appendChild(dir_component(item.url, item.name))
				break
		}

	}
	if (arr.length > 0) {
		q_cache.set(url, arr)
	}
	})
}

function swap_player(url: string, index: number) {
	let queue = q_cache.get(url)
	if (queue) {	
		player.src = queue[index].url
		player.play()
		song_title.textContent = "~" + queue[index].name
		player.addEventListener('ended', () => {
			if (index >= queue.length - 1) {
				swap_player(url, 0)
			} else {
				swap_player(url, index + 1)	
			}
		})
	}
}

function reload(url: string, stack: string[]): HTMLLIElement {

	const li = document.createElement("li")
	
    const span = document.createElement("span")
    span.className = "dir"
    span.textContent = `~./`

    span.addEventListener("click", () => {
		stack.pop()
        hydrate(url)
    })

    li.appendChild(span)

    return li
}

function root(history: string[]): HTMLLIElement {
	const li = document.createElement("li")
    const span = document.createElement("span")
    span.className = "dir"
    span.textContent = `~/`

    span.addEventListener("click", () => {
        history.length = 0
		bootstrap(source_list)
    })

    li.appendChild(span)

	return li
}

function back(history: string[]): HTMLLIElement {
	const li = document.createElement("li")

    const span = document.createElement("span")
    span.className = "dir"
    span.textContent = `~../`

    span.addEventListener("click", () => {
        history.pop()
		let hist = history.pop()
		if (hist) {
			hydrate(hist)
		} else {
			history.length = 0
			bootstrap(source_list)
		}	
    })

    li.appendChild(span)

	return li
}

function loader(): string {
	return `<li><span class="loader"></span><span class="loader"></span><span class="loader"></span><span class="loader"></span></li>`
}

window.send({key: "READY"})

