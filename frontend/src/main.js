import { createApp } from 'vue';
import App from './App.vue';
import router from './router'; // Import the router
import { createPinia } from 'pinia'; // Import Pinia
import './style.css'; // Existing CSS import

const app = createApp(App);
const pinia = createPinia(); // Create Pinia instance

app.use(router); // Use the router
app.use(pinia); // Use Pinia

app.mount('#app');
