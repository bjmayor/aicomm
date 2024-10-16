<template>
  <div class="sidebar">
    <div class="workspace">
      <div class="workspace-name" @click="toggleDropdown">
        <span>{{ workspaceName }}</span>
        <button class="dropdown-toggle">&nbsp;▼</button>
      </div>
      <div v-if="dropdownVisible" class="dropdown-menu">
        <ul>
          <li @click="logout">Logout</li>
          <!-- Add more dropdown items here as needed -->
        </ul>
      </div>
      <button class="add-channel" @click="addChannel">+</button>
    </div>
    <div class="channels">
      <h2>Channels</h2>
      <ul>
        <li v-for="channel in channels" :key="channel.id">
          # {{ channel.name }}
        </li>
      </ul>
    </div>
    <div class="direct-messages">
      <h2>Direct Messages</h2>
      <ul>
        <li v-for="c in singleChannels" :key="c.id">
          <img
            :src="`https://ui-avatars.com/api/?name=${c.recipient.fullname.replace(
              ' ',
              '+'
            )}`"
            class="avatar"
            alt="Avatar"
          />
          {{ c.recipient.fullname }}
        </li>
      </ul>
    </div>
  </div>
</template>
<script>
export default {
  data() {
    return {
      dropdownVisible: false, // Control visibility of the dropdown menu
    };
  },
  computed: {
    workspaceName() {
      return this.$store.getters.getWorkspace.name || "No Workspace";
    },
    channels() {
      return this.$store.getters.getChannels;
    },
    singleChannels() {
      // Placeholder for direct messages, if needed.
      // This could be another state managed by Vuex.
      return this.$store.getters.getSingleChannels;
    },
  },
  methods: {
    toggleDropdown() {
      this.dropdownVisible = !this.dropdownVisible;
    },
    logout() {
      this.$store.dispatch("logout");
      this.$router.push("/login"); // Redirect to login after logout
    },
    handleOutsideClick(event) {
      if (!this.$el.contains(event.target)) {
        this.dropdownVisible = false;
      }
    },
    addChannel() {
      // Trigger an action to add a new channel
      const newChannel = {
        id: Date.now().toString(), // Unique ID for the new channel
        name: `Channel ${this.channels.length + 1}`,
      };
      this.$store.dispatch("addChannel", newChannel);
    },
  },
  mounted() {
    document.addEventListener("click", this.handleOutsideClick);
  },
  beforeDestroy() {
    document.removeEventListener("click", this.handleOutsideClick);
  },
};
</script>
<style scoped>
/* Base sidebar styling */
.sidebar {
  width: 250px;
  background-color: #2f3136;
  color: #fff;
  display: flex;
  flex-direction: column;
  height: 100vh;
  padding: 10px;
  font-size: 14px;
}
/* Workspace section */
.workspace {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;
}
.workspace-name {
  font-weight: bold;
  font-size: 16px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.dropdown-toggle {
  background: none;
  border: none;
  color: #b9bbbe;
  font-size: 16px;
  cursor: pointer;
  padding: 0;
  margin: 0;
}
.dropdown-menu {
  position: absolute;
  top: 40px;
  left: 0;
  width: 200px;
  background-color: #2f3136;
  border: 1px solid #3a3e44;
  border-radius: 4px;
  padding: 10px;
  box-shadow: 0px 4px 8px rgba(0, 0, 0, 0.15);
  z-index: 1000;
}
.dropdown-menu ul {
  list-style: none;
  padding: 0;
  margin: 0;
}
.dropdown-menu li {
  padding: 8px;
  cursor: pointer;
  color: #b9bbbe;
}
.dropdown-menu li:hover {
  background-color: #3a3e44;
  color: #fff;
}

.add-channel {
  background: none;
  border: none;
  color: #b9bbbe;
  font-size: 20px;
  cursor: pointer;
  padding: 0;
  margin: 0;
}
.add-channel:hover {
  color: #fff;
}
/* Channels section */
.channels {
  margin-bottom: 20px;
}
.channels h2 {
  font-size: 12px;
  text-transform: uppercase;
  margin-bottom: 10px;
  color: #b9bbbe;
}
.channels ul {
  list-style-type: none;
  padding: 0;
  margin: 0;
}
.channels li {
  padding: 5px 10px;
  cursor: pointer;
  border-radius: 4px;
}
.channels li:hover {
  background-color: #3a3e44;
}
/* Direct Messages section */
.direct-messages h2 {
  font-size: 12px;
  text-transform: uppercase;
  margin-bottom: 10px;
  color: #b9bbbe;
}
.direct-messages ul {
  list-style-type: none;
  padding: 0;
  margin: 0;
}
.direct-messages li {
  display: flex;
  align-items: center;
  padding: 5px 10px;
  cursor: pointer;
  border-radius: 4px;
}
.direct-messages li:hover {
  background-color: #3a3e44;
}
.avatar {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  margin-right: 10px;
}
</style>