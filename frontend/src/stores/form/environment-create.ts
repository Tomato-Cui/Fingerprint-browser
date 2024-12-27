import { defineStore } from "pinia";
import { useForm } from "vee-validate";


interface EnvironmentInfo {
    name?: string
}

export const useEnvironmentCreateFromStore = defineStore('environment-create-form-store', () => {

    const {
        defineField, handleSubmit, errors
    } = useForm<EnvironmentInfo>({
        validationSchema: {
            name: (name?: string) => {
                return name ? true : "请输入环境名称";
            },
        },
    });


    const [environmentName, environmentNameProps] = defineField('name');

    const forms = {
        environmentName, environmentNameProps
    }

    return {
        defineField, handleSubmit, errors, forms
    }
})