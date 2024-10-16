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
    users: {}, // users hashmap under workspace, keyed by user ID
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
    setUsers(state, users) {
      state.users = users;
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
      const storedUsers = localStorage.getItem("users");

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
      if (storedUsers) {
        state.users = JSON.parse(storedUsers);
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

        const user = await loadState(response, commit);

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

        const user = await loadState(response, commit);
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
      // filter out channels that type == 'single'
      return state.channels.filter((channel) => channel.type !== "single");
    },
    getSingleChannels(state) {
      const channels = state.channels.filter(
        (channel) => channel.type == "single"
      );
      // return channel member that is not myself
      return channels.map((channel) => {
        const id = channel.members.filter(
          (member) => member !== state.user.id
        )[0];
        channel.recipient = state.users[id];
        return channel;
      });
    },
    getChannelMessages: (state) => (channelId) => {
      return state.messages[channelId] || [];
    },
  },
});

async function loadState(response, commit) {
  const token = response.data.token;
  const user = jwtDecode(token); // Decode the JWT to get user info
  const workspace = { id: user.wsId, name: user.wsName };

  try {
    // fetch all workspace users
    const userResp = await axios.get(`${getUrlBase()}/users`, {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
    const users = userResp.data;
    const usersMap = {};
    users.forEach((user) => {
      usersMap[user.id] = user;
    });
    // fetch all my channels
    const chatsResp = await axios.get(`${getUrlBase()}/chats`, {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
    const channels = chatsResp.data;
    // Store user info, token, and workspace in localStorage
    localStorage.setItem("user", JSON.stringify(user));
    localStorage.setItem("token", token);
    localStorage.setItem("workspace", JSON.stringify(workspace));
    localStorage.setItem("users", JSON.stringify(usersMap));
    localStorage.setItem("channels", JSON.stringify(channels));

    // Commit the mutations to update the state
    commit("setUser", user);
    commit("setToken", token);
    commit("setWorkspace", workspace);
    commit("setChannels", channels);
    commit("setUsers", usersMap);
    return user;
  } catch (error) {
    console.error("Failed to fetch channels:", error);
  }

  return user;
}
