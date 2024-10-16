<template>
  <div class="message-send">
    <input
      v-model="message"
      @keyup.enter="sendMessage"
      placeholder="Type a message..."
      class="message-input"
      type="text"
    />
    <button @click="sendMessage" class="send-button">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="icon"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M5 12h14M12 5l7 7-7 7"
        />
      </svg>
    </button>
  </div>
</template>
<script>
export default {
  data() {
    return {
      message: "",
    };
  },
  computed: {
    userId() {
      return this.$store.state.user.id;
    },

    activeChannelId() {
      const channel = this.$store.state.activeChannel;
      if (!channel) {
        return null;
      }
      return channel.id;
    },
  },
  methods: {
    async sendMessage() {
      if (this.message.trim() === "") return;

      const payload = {
        chatId: this.activeChannelId,
        content: this.message,
      };
      console.log("Sending message:", payload);
      try {
        this.$store.dispatch("sendMessage", payload);
        this.message = ""; // Clear the input after sending
      } catch (error) {
        console.error("Failed to send message:", error);
      }
    },
  },
};
</script>
<style scoped>
/* Container styling */
.message-send {
  display: flex;
  align-items: center;
  padding: 10px 0px;
  background-color: #fff;
  border-top: 1px solid #eee;
}
/* Input field styling */
.message-input {
  flex: 1;
  padding: 12px 12px;
  border: none;
  border-radius: 10px;
  background-color: #eee;
  font-size: 14px;
  margin: 0px 10px;
}
.message-input::placeholder {
  color: #72767d;
}
/* Send button styling */
.send-button {
  background-color: #5865f2;
  border: none;
  border-radius: 50%;
  padding: 10px;
  margin-right: 5px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
}
.send-button .icon {
  width: 20px;
  height: 20px;
}
.send-button:hover {
  background-color: #4752c4;
}
</style>