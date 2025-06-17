<template>
  <div class="min-h-screen flex flex-col font-sans">
    <!-- Header -->
    <header class="bg-gray-800 text-white shadow-md">
      <nav class="container mx-auto px-6 py-3 flex justify-between items-center">
        <div class="flex items-center">
          <router-link to="/" class="text-2xl font-bold hover:text-gray-300">
            <img src="/img/firefly-logo-placeholder.svg" alt="ASBEL Firefly Mascot" class="h-10 w-10 inline-block mr-2" />
            ASBEL
          </router-link>
        </div>
        <div class="hidden md:flex items-center space-x-4">
          <router-link to="/" class="px-3 py-2 rounded hover:bg-gray-700">Home</router-link>
          <router-link to="/getting-started" class="px-3 py-2 rounded hover:bg-gray-700">Getting Started</router-link>
          <router-link to="/documentation" class="px-3 py-2 rounded hover:bg-gray-700">Documentation</router-link>
          <router-link to="/cli-reference" class="px-3 py-2 rounded hover:bg-gray-700">CLI</router-link>
          <router-link to="/roadmap" class="px-3 py-2 rounded hover:bg-gray-700">Roadmap</router-link>
          <a href="#" class="px-3 py-2 rounded hover:bg-gray-700">Community (Soon)</a>
        </div>
        <div class="md:hidden">
          <button @click="toggleMobileMenu" class="text-white focus:outline-none">
            <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7"></path>
            </svg>
          </button>
        </div>
      </nav>
      <!-- Mobile Menu -->
      <div v-if="isMobileMenuOpen" class="md:hidden bg-gray-800">
        <router-link to="/" class="block px-4 py-2 text-sm hover:bg-gray-700" @click="closeMobileMenu">Home</router-link>
        <router-link to="/getting-started" class="block px-4 py-2 text-sm hover:bg-gray-700" @click="closeMobileMenu">Getting Started</router-link>
        <router-link to="/documentation" class="block px-4 py-2 text-sm hover:bg-gray-700" @click="closeMobileMenu">Documentation</router-link>
        <router-link to="/cli-reference" class="block px-4 py-2 text-sm hover:bg-gray-700" @click="closeMobileMenu">CLI</router-link>
        <router-link to="/roadmap" class="block px-4 py-2 text-sm hover:bg-gray-700" @click="closeMobileMenu">Roadmap</router-link>
        <a href="#" class="block px-4 py-2 text-sm hover:bg-gray-700" @click="closeMobileMenu">Community (Soon)</a>
      </div>
    </header>

    <!-- Main Content -->
    <main class="flex-grow container mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <router-view /> <!-- This is where page content will be injected by Vue Router -->
    </main>

    <!-- Footer -->
    <footer class="bg-gray-900 text-white py-8">
      <div class="container mx-auto px-6 text-center">
        <div class="mb-4">
          <!-- Firefly icon can go here too if desired -->
          <p class="text-lg font-semibold">ASBEL Language</p>
          <p class="text-sm text-gray-400">Write like Python – Run like C – Be safe like Rust</p>
        </div>
        <div class="mb-4">
          <a href="#" class="text-gray-400 hover:text-white px-2">About</a>
          <span class="text-gray-500">|</span>
          <a href="#" class="text-gray-400 hover:text-white px-2">GitHub (Soon)</a>
          <span class="text-gray-500">|</span>
          <a href="#" class="text-gray-400 hover:text-white px-2">Contributing (Soon)</a>
        </div>
        <p class="text-xs text-gray-500">&copy; 2024 The ASBEL Project. All rights reserved.</p>
      </div>
    </footer>
  </div>
</template>

<script setup>
import { ref } from 'vue';

const isMobileMenuOpen = ref(false);

const toggleMobileMenu = () => {
  isMobileMenuOpen.value = !isMobileMenuOpen.value;
};

const closeMobileMenu = () => {
  isMobileMenuOpen.value = false;
};
</script>

<style scoped>
/* Scoped styles for AppLayout if needed */
.router-link-exact-active {
  @apply bg-gray-700 text-white;
}
/* Ensure content pushes footer down */
.min-h-screen {
  display: flex;
  flex-direction: column;
}
main {
  flex-grow: 1;
}
</style>
