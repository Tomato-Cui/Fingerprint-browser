import { createApp } from 'vue';
import { createPinia } from 'pinia'
import App from './App.vue';
import router from './router';
import "./index.css"
import "./isolation"

const pinia = createPinia();
const app = createApp(App);

app.use(router);
// app.use(ElementPlus);
app.use(pinia);
app.mount('#app');
