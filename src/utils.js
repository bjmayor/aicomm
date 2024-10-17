const URL_BASE = "http://localhost:6688/api";
const SSE_URL = "http://localhost:6687/events";

const getUrlBase = () => {
  return URL_BASE;
};

const initSSE = (store) => {
  let url = `${SSE_URL}?token=${store.state.token}`;
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
