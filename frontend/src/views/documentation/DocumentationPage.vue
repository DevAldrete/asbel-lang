<template>
  <div class="flex flex-col md:flex-row min-h-screen">
    <!-- Sidebar -->
    <aside class="w-full md:w-64 bg-gray-100 p-4 md:p-6 shadow-lg md:min-h-screen print:hidden">
      <h2 class="text-xl font-semibold mb-4 text-gray-700">ASBEL Docs</h2>
      <nav>
        <ul>
          <li v-for="section in docSections" :key="section.path" class="mb-2">
            <router-link
              :to="'/documentation/' + section.path"
              class="block px-3 py-2 rounded-md text-gray-600 hover:bg-gray-200 hover:text-gray-800 transition-colors duration-150"
              active-class="bg-yellow-400 text-gray-900 font-semibold"
            >
              {{ section.title }}
            </router-link>
          </li>
        </ul>
      </nav>
      <hr class="my-6 border-gray-300" />
      <h3 class="text-lg font-semibold mb-3 text-gray-700">Quick Links</h3>
        <ul>
          <li class="mb-2">
            <router-link
              to="/getting-started"
              class="block px-3 py-2 rounded-md text-gray-600 hover:bg-gray-200 hover:text-gray-800 transition-colors duration-150"
              active-class="bg-yellow-400 text-gray-900 font-semibold"
            >
              Getting Started
            </router-link>
          </li>
          <li class="mb-2">
            <router-link
              to="/cli-reference"
              class="block px-3 py-2 rounded-md text-gray-600 hover:bg-gray-200 hover:text-gray-800 transition-colors duration-150"
              active-class="bg-yellow-400 text-gray-900 font-semibold"
            >
              CLI Reference
            </router-link>
          </li>
        </ul>
    </aside>

    <!-- Main Content Area -->
    <main class="flex-grow p-6 md:p-10 bg-white">
      <!-- This router-view will display the content of /documentation/:section -->
      <router-view v-slot="{ Component }">
        <transition name="fade" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </main>
  </div>
</template>

<script setup>
import { ref } from 'vue';

const docSections = ref([
  { title: 'Introduction', path: 'introduction' },
  { title: '1. Variables & Types', path: 'variables-types' },
  { title: '2. Functions', path: 'functions' },
  { title: '3. Lambdas & UFCS', path: 'lambdas-ufcs' },
  { title: '4. Control Flow', path: 'control-flow' },
  { title: '5. Loops & Ranges', path: 'loops-ranges' },
  { title: '6. Pipelines', path: 'pipelines' },
  { title: '7. Structs, Interfaces, Macros', path: 'structs-interfaces-macros' },
  { title: '8. FFI (C Interop)', path: 'ffi' },
  { title: '9. Tooling & Ecosystem', path: 'tooling-ecosystem' },
  { title: '10. Concurrency & Parallelism', path: 'concurrency-parallelism' },
  { title: '11. Integrated Testing', path: 'testing' },
  { title: '12. Future Directions', path: 'future-directions' },
  // Add more sections as they are defined in DOCS-ABLE.md
]);

// The actual content for these sections will be loaded into DocSectionPage.vue
// based on the :section route parameter.
</script>

<style scoped>
.router-link-exact-active {
  /* Tailwind's active-class in router-link should handle this,
     but this is a fallback or for more specific styling if needed */
}

/* Page transition animations */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
