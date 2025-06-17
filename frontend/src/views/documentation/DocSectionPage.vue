<template>
  <div class="doc-section-container">
    <component :is="currentDocComponent" />
  </div>
</template>

<script setup>
import { ref, watch, defineProps, shallowRef } from 'vue';
import { useRoute } from 'vue-router';

const props = defineProps({
  section: String // This prop comes from the route param: /documentation/:section
});

const route = useRoute();
const currentDocComponent = shallowRef(null);

// Helper function to map section path to component name
// Ensure component names match the created files (e.g., 'introduction' -> 'IntroductionDoc.vue')
const getComponentName = (sectionPath) => {
  if (!sectionPath) return null;
  // Capitalize first letter and remove hyphens for typical component naming conventions
  // e.g., 'variables-types' becomes 'VariablesTypes'
  const componentName = sectionPath.split('-').map(word => word.charAt(0).toUpperCase() + word.slice(1)).join('');
  return componentName + 'Doc'; // e.g., IntroductionDoc, VariablesTypesDoc
};

const loadDocComponent = async () => {
  const sectionPath = route.params.section || props.section;
  if (sectionPath) {
    const componentName = getComponentName(sectionPath);
    try {
      // Dynamically import the component
      // Note: Vite requires the path to be somewhat static for analysis.
      // Using a direct mapping or ensuring filenames are predictable.
      const component = await import(`./content/${componentName}.vue`);
      currentDocComponent.value = component.default;
    } catch (e) {
      console.error(`Failed to load documentation component for section: ${sectionPath}`, e);
      // Optionally, load a 'NotFound' or 'ContentComingSoon' component
      const fallback = await import('../../NotFoundPage.vue'); // Or a specific DocNotFound.vue
      currentDocComponent.value = fallback.default;
    }
  } else {
    // Default content if no section is specified (e.g. for /documentation itself)
    // You might want a specific 'DocumentationHome.vue' component here
    try {
        const component = await import(`./content/IntroductionDoc.vue`); // Default to introduction
        currentDocComponent.value = component.default;
    } catch (e) {
        const fallback = await import('../../NotFoundPage.vue');
        currentDocComponent.value = fallback.default;
    }
  }
};

// Watch for changes in the route parameter (section)
watch(() => route.params.section, loadDocComponent, { immediate: true });
// Also watch the prop if it's the primary way to pass the section
// Ensure only one watcher is active if both route.params.section and props.section are expected to be the same
// For direct route children, route.params.section is canonical.
// If this component could be used programmatically with props, then props.section might be primary.
// Given it's a route component, route.params.section is usually preferred.
// Removing the props.section watch if route.params.section is always available and preferred.
// If props.section is indeed used independently, keep it. For now, assuming route.params is primary for this page.
// watch(() => props.section, loadDocComponent, { immediate: true }); // Kept for now, but review if causes double loads

</script>

<style scoped>
.doc-section-container {
  width: 100%;
}
</style>
