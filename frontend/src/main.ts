import { createApp } from 'vue';
import { createPinia } from 'pinia'
import init from '@/plugins/init'
import i18n from '@/plugins/i18n'
import App from './App.vue';
import router from './router';
import "./index.css"



const pinia = createPinia();
const app = createApp(App);

app.use(router);
app.use(pinia);
app.use(i18n);
app.use(init);
app.mount('#app');


// 关闭 Vue 3 的所有 Warn
app.config.warnHandler = () => null;