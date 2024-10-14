<template>
  <div class="sidebar">
    <div class="workspace">
      <div class="workspace-name">{{ workspaceName }}</div>
      <button class="add-channel">+</button>
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
        <li v-for="user in users" :key="user.id">
          <img :src="user.avatar" class="avatar" alt="Avatar" /> {{ user.name }}
        </li>
      </ul>
    </div>
  </div>
</template>
<script>
export default {
  data() {
    return {
      workspaceName: "My Workspace",
      channels: [
        { id: 1, name: "general" },
        { id: 2, name: "random" },
      ],
      users: [
        { id: 1, name: "Alice", avatar: "https://via.placeholder.com/24" },
        { id: 2, name: "Bob", avatar: "https://via.placeholder.com/24" },
      ],
    };
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