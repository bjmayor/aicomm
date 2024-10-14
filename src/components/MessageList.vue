<template>
  <div class="message-list">
    <div v-for="message in messages" :key="message.id" class="message">
      <img :src="message.avatar" class="avatar" alt="Avatar" />
      <div class="message-content">
        <div class="message-header">
          <span class="message-user">{{ message.user }}</span>
          <span class="message-time">{{ formatTime(message.time) }}</span>
        </div>
        <div class="message-text">{{ message.text }}</div>
      </div>
    </div>
  </div>
</template>
<script>
export default {
  data() {
    return {
      messages: [
        {
          id: 1,
          user: "Alice",
          avatar: "https://via.placeholder.com/40",
          text: "Hello there!",
          time: "2024-08-17T09:00:00Z",
        },
        {
          id: 2,
          user: "Bob",
          avatar: "https://via.placeholder.com/40",
          text: "Hi Alice! How are you?",
          time: "2024-08-17T09:05:00Z",
        },
      ],
    };
  },
  methods: {
    formatTime(time) {
      const date = new Date(time);
      return date.toLocaleTimeString([], {
        hour: "2-digit",
        minute: "2-digit",
      });
    },
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
</style>