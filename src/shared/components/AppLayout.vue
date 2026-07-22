<script setup lang="ts">
import Sidebar from './Sidebar.vue'
</script>

<template>
  <div class="app-layout">
    <Sidebar />
    <main class="app-main">
      <router-view />
    </main>
  </div>
</template>

<style scoped>
.app-layout {
  display: flex;
  min-height: 100vh;
  background: var(--gym-bg);
  color: var(--gym-text);
}

.app-main {
  flex: 1;
  min-width: 0;
  padding: clamp(20px, 2.4vw, 40px);
  overflow-y: auto;
}

@media (max-width: 760px) {
  .app-layout {
    flex-direction: column;
  }

  .app-main {
    padding: 16px;
  }
}
</style>
