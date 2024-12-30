import { defineStore } from "pinia";
import { useForm } from "vee-validate";


interface ExtensionInfo {
    url: string
}

export const useExtensionCenterFromStore = defineStore('extension-center-form-store', () => {

    const {
        defineField, handleSubmit, errors
    } = useForm<ExtensionInfo>({
        validationSchema: {
            url: (url?: string) => {
                if (!url) {
                    return "请输入chrome中插件地址";
                }
                if (!url?.includes('https://chromewebstore.google.com/detail')) {
                    return "请输入合法的chrome插件地址";
                }
                return true;
            },
        },
    });


    const [url, urlProps] = defineField('url');

    const forms = {
        url, urlProps
    };

    return {
        defineField, handleSubmit, errors, forms
    }
})