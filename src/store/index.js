import { createStore } from "vuex";
import axios from "axios";
import { jwtDecode } from "jwt-decode";
import { getUrlBase } from "../utils";

export default createStore({
  state: {
    user: null, // User information
    token: null, // Authentication token
    workspace: {}, // Current workspace
    channels: [], // List of channels
    messages: {}, // Messages hashmap, keyed by channel ID
  },
  mutations: {
    setUser(state, user) {
      state.user = user;
    },
    setToken(state, token) {
      state.token = token;
    },
    setWorkspace(state, workspace) {
      state.workspace = workspace;
    },
    setChannels(state, channels) {
      state.channels = channels;
    },
    setMessages(state, messages) {
      state.messages = messages;
    },
    addChannel(state, channel) {
      state.channels.push(channel);
      state.messages[channel.id] = []; // Initialize message list for the new channel
    },
    addMessage(state, { channelId, message }) {
      if (state.messages[channelId]) {
        state.messages[channelId].push(message);
      } else {
        state.messages[channelId] = [message];
      }
    },
    loadUserState(state) {
      const storedUser = localStorage.getItem("user");
      const storedToken = localStorage.getItem("token");
      const storedWorkspace = localStorage.getItem("workspace");
      const storedChannels = localStorage.getItem("channels");
      const storedMessages = localStorage.getItem("messages");

      if (storedUser) {
        state.user = JSON.parse(storedUser);
      }
      if (storedToken) {
        state.token = storedToken;
      }
      if (storedWorkspace) {
        state.workspace = JSON.parse(storedWorkspace);
      }
      if (storedChannels) {
        state.channels = JSON.parse(storedChannels);
      }
      if (storedMessages) {
        state.messages = JSON.parse(storedMessages);
      }
    },
  },
  actions: {
    async signup({ commit }, { email, fullname, password, workspace }) {
      try {
        const response = await axios.post(`${getUrlBase()}/signup`, {
          email,
          fullname,
          password,
          workspace,
        });

        const user = saveUser(response, commit);

        return user;
      } catch (error) {
        console.error("Signup failed:", error);
        throw error;
      }
    },
    async signin({ commit }, { email, password }) {
      try {
        const response = await axios.post(`${getUrlBase()}/signin`, {
          email,
          password,
        });

        const user = saveUser(response, commit);
        return user;
      } catch (error) {
        console.error("Login failed:", error);
        throw error;
      }
    },
    logout({ commit }) {
      // Clear local storage and state
      localStorage.removeItem("user");
      localStorage.removeItem("token");
      localStorage.removeItem("workspace");
      localStorage.removeItem("channels");
      localStorage.removeItem("messages");

      commit("setUser", null);
      commit("setToken", null);
      commit("setWorkspace", "");
      commit("setChannels", []);
      commit("setMessages", {});
    },
    addChannel({ commit }, channel) {
      commit("addChannel", channel);

      // Update the channels and messages in local storage
      localStorage.setItem("channels", JSON.stringify(this.state.channels));
      localStorage.setItem("messages", JSON.stringify(this.state.messages));
    },
    addMessage({ commit }, { channelId, message }) {
      commit("addMessage", { channelId, message });

      // Update the messages in local storage
      localStorage.setItem("messages", JSON.stringify(this.state.messages));
    },
    loadUserState({ commit }) {
      commit("loadUserState");
    },
  },
  getters: {
    isAuthenticated(state) {
      return !!state.token;
    },
    getUser(state) {
      return state.user;
    },
    getWorkspace(state) {
      return state.workspace;
    },
    getChannels(state) {
      return state.channels;
    },
    getChannelMessages: (state) => (channelId) => {
      return state.messages[channelId] || [];
    },
  },
});

function saveUser(response, commit) {
  const token = response.data.token;
  const user = jwtDecode(token); // Decode the JWT to get user info
  const workspace = { id: user.wsId, name: user.wsName };

  // Store user info, token, and workspace in localStorage
  localStorage.setItem("user", JSON.stringify(user));
  localStorage.setItem("token", token);
  localStorage.setItem("workspace", JSON.stringify(workspace));

  // Commit the mutations to update the state
  commit("setUser", user);
  commit("setToken", token);
  commit("setWorkspace", workspace);
  return user;
}
