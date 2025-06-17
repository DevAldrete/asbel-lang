import { createRouter, createWebHistory } from 'vue-router';
import HomePage from '../views/HomePage.vue';
import DocumentationPage from '../views/documentation/DocumentationPage.vue';
import DocSectionPage from '../views/documentation/DocSectionPage.vue'; // For /documentation/:section
import GettingStartedPage from '../views/GettingStartedPage.vue';
import CliReferencePage from '../views/CliReferencePage.vue';
import RoadmapPage from '../views/RoadmapPage.vue';
import NotFoundPage from '../views/NotFoundPage.vue'; // Import NotFoundPage

const routes = [
  {
    path: '/',
    name: 'Home',
    component: HomePage,
  },
  {
    path: '/getting-started',
    name: 'GettingStarted',
    component: GettingStartedPage,
  },
  {
    path: '/documentation',
    name: 'Documentation',
    component: DocumentationPage,
  },
  {
    path: '/documentation/:section',
    name: 'DocSection',
    component: DocSectionPage,
    props: true,
  },
  {
    path: '/cli-reference',
    name: 'CliReference',
    component: CliReferencePage,
  },
  {
    path: '/roadmap',
    name: 'Roadmap',
    component: RoadmapPage,
  },
  {
    path: '/:catchAll(.*)*', // Catch-all route
    name: 'NotFound',
    component: NotFoundPage,
  },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
  scrollBehavior(to, from, savedPosition) {
    if (savedPosition) {
      return savedPosition;
    } else {
      return { top: 0 };
    }
  }
});

export default router;
