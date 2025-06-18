<template>
  <AppLayout>
    <router-view class="bg-red-500 prose">
      <input type="text" class="form-input" placeholder="Test input" />
    </router-view>
  </AppLayout>
</template>

<script setup>
import AppLayout from './layouts/AppLayout.vue';
</script>

<style>
/* Global styles can be added here or in index.css / main.css */
/* Ensure router-view fills the layout's main content area */
#app {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}
</style>
