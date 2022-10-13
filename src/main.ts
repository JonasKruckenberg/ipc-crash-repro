import { invoke } from "@tauri-apps/api/tauri";
import { listen, Event } from '@tauri-apps/api/event'

let messagesEl: HTMLDivElement | null

window.addEventListener("DOMContentLoaded", () => {
  messagesEl = document.querySelector("#messages");

  setInterval(() => {
    invoke('emit_all', { event: 'debug://event', payload: Math.random().toString() })
  }, 200)

  listen('debug://event', (ev: Event<string>) => {
    let child = document.createElement('li')
    child.innerText = ev.payload
    messagesEl?.appendChild(child)
  })
});

async function new_window() {
  await invoke('new_window')
}

window.new_window = new_window

async function message_windows() {
  invoke('emit_all', { event: 'debug://event', payload: Math.random().toString() })
}

window.message_windows = message_windows
