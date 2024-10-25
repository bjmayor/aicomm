<template>
  <router-view />
</template>

<script>
export default {
  name: "App",
};
</script>

<style>
html,
body,
#app {
  height: 100%;
  margin: 0;
  overflow: hidden; /* Prevent scrolling */
}

#app {
  display: flex;
  flex-direction: column;
}
</style>
