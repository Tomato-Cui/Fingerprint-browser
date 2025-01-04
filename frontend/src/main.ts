import { createApp } from 'vue';
import { createPinia } from 'pinia'
import init from '@/plugins/init'
import App from './App.vue';
import router from './router';
import "./index.css"

const pinia = createPinia();
const app = createApp(App);

app.use(init)
app.use(router);
app.use(pinia);
app.mount('#app');
