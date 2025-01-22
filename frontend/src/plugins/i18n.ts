import { ref, watchEffect } from 'vue';
import { createI18n } from 'vue-i18n'


const defaultLocale = 'zh-CN';


export let locale = ref<'zh-CN' | 'en-US'>(defaultLocale);

export const setLocale = (newLocale: 'zh-CN' | 'en-US') => {
    locale.value = newLocale;
    i18n.global.locale = newLocale;
    localStorage.setItem('locale', newLocale);
}

const messages = {
    'en-US': {
        message: {
            appName: 'Fantasy Browser',
        }
    },
    'zh-CN': {
        message: {
            appName: '幻境浏览器',
        }
    }
}

const i18n = createI18n({
    locale: locale.value,
    messages
})

export default i18n

// 从localstorate中加载
const loadlocalStorage = () => {
    const localeStr = localStorage.getItem('locale');
    if (!localeStr) {
        locale.value = defaultLocale;
        localStorage.setItem('locale', defaultLocale)
    } else {
        locale.value = localeStr as 'zh-CN' | 'en-US';
    }
}

loadlocalStorage()
watchEffect(() => {
    window.addEventListener('storage', () => {
        loadlocalStorage()
    })
})