import { invoke } from "@tauri-apps/api/core";

const URL_BASE = "http://localhost:6688/api";
const SSE_URL = "http://localhost:6687/events";
let config = null;
try {
  config = await invoke("get_config");
} catch (e) {
  console.error("failed to get config:", e);
}
console.log("config:", config);

const getUrlBase = () => {
  if (config && config.server.chat) {
    return config.server.chat;
  }
  return URL_BASE;
};

const getSseBase = () => {
  if (config && config.server.notify) {
    return config.server.notify;
  }
  return SSE_URL;
};

const initSSE = (store) => {
  const sseUrl = getSseBase();
  let url = `${sseUrl}?token=${store.state.token}`;
  const sse = new EventSource(url);
  sse.addEventListener("NewMessage", (e) => {
    let data = JSON.parse(e.data);
    console.log("message:", e.data);
    delete data.event;
    store.commit("addMessage", { channelId: data.chatId, message: data });
  });
  sse.onmessage = (event) => {
    console.log("got event:", event);
    // const data = JSON.parse(event.data);
    // commit('addMessage', data);
  };
  sse.onerror = (error) => {
    console.error("EventSource failed:", error);
    sse.close();
  };
  return sse;
};

export { getUrlBase, initSSE };
