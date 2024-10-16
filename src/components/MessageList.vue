<template>
  <div class="message-list">
    <div v-if="messages.length === 0" class="no-messages">
      No messages in this channel yet.
    </div>
    <div v-else>
      <div v-for="message in messages" :key="message.id" class="message">
        <img
          :src="`https://ui-avatars.com/api/?name=${getSender(
            message.senderId
          ).fullname.replace(' ', '+')}`"
          class="avatar"
          alt="Avatar"
        />
        <div class="message-content">
          <div class="message-header">
            <span class="message-user">{{
              getSender(message.senderId).fullname
            }}</span>
            <span class="message-time">{{
              formatTime(message.createdAt)
            }}</span>
          </div>
          <div class="message-text">{{ message.content }}</div>
        </div>
      </div>
    </div>
  </div>
</template>
<script>
export default {
  computed: {
    messages() {
      return this.$store.getters.getMessagesForActiveChannel;
    },
    activeChannelId() {
      let channel = this.$store.state.activeChannel;
      if (!channel) {
        return null;
      }
      return channel.id;
    },
  },
  watch: {
    activeChannelId(newChannelId) {
      if (newChannelId) {
        this.fetchMessages(newChannelId);
      }
    },
  },
  methods: {
    formatTime(time) {
      const date = new Date(time);
      return date.toLocaleTimeString([], {
        hour: "2-digit",
        minute: "2-digit",
      });
    },
    fetchMessages(channelId) {
      this.$store.dispatch("fetchMessagesForChannel", channelId);
    },
    getSender(userId) {
      return this.$store.getters.getUserById(userId);
    },
  },
  mounted() {
    if (this.activeChannelId) {
      this.fetchMessages(this.activeChannelId);
    }
  },
};
</script>
<style scoped>
/* Container styling */
.message-list {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}
/* Individual message styling */
.message {
  display: flex;
  align-items: flex-start;
  margin-bottom: 20px;
}
.avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  margin-right: 10px;
}
.message-content {
  max-width: 80%;
}
/* Header styling: username and timestamp */
.message-header {
  display: flex;
  align-items: center;
  margin-bottom: 5px;
}
.message-user {
  font-weight: bold;
  margin-right: 10px;
}
.message-time {
  font-size: 12px;
}
/* Message text styling */
.message-text {
  font-size: 14px;
  line-height: 1.4;
  word-wrap: break-word;
  white-space: pre-wrap;
}

.no-messages {
  text-align: center;
  color: #b9bbbe;
  margin-top: 20px;
}
</style>