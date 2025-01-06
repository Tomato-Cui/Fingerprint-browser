import { createApp } from 'vue';
import { createPinia } from 'pinia'
import { Init, Update } from '@/plugins'
import App from './App.vue';
import router from './router';
import "./index.css"

const pinia = createPinia();
const app = createApp(App);

app.use(router);
app.use(pinia);
app.use(Init);
app.use(Update);
app.mount('#app');
