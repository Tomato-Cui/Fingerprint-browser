import { defineStore } from "pinia";
import { useForm } from "vee-validate";


interface EnvironmentInfo {
    name?: string
    description?:string
}

export const useEnvironmentCreateFromStore = defineStore('environment-create-form-store', () => {

    const {
        defineField, handleSubmit, errors
    } = useForm<EnvironmentInfo>({
        validationSchema: {
            name: (name?: string) => {
                return name ? true : "请输入环境名称";
            },
            description: (description?: string) => {
                return description ? true : "请输入描述";
            },
        },
    });


    const [environmentName, environmentNameProps] = defineField('name');
    const [environmentDescription, environmentDescriptionProps] = defineField('description');

    const forms = {
        environmentName,
        environmentNameProps,
        environmentDescription,
        environmentDescriptionProps
    }

    return {
        defineField, handleSubmit, errors, forms
    }
})