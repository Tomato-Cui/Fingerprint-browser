import { defineStore } from "pinia";
import { useForm } from "vee-validate";


interface EnvironmentInfo {
    name?: string
    numbers?: string
}

export const useEnvironmentCreatesFromStore = defineStore('environment-creates-form-store', () => {
    const {
        defineField, handleSubmit, errors
    } = useForm<EnvironmentInfo>({
        validationSchema: {
            name: (name?: string) => {
                return name ? true : "请输入环境名称";
            },
            numbers: (numbers?: number) => {
                return numbers ? true : "请指定环境数量";
            },
        },
    });


    const [environmentName, environmentNameProps] = defineField('name');
    const [environmentNumbers, environmentNumbersProps] = defineField('numbers');

    const forms = {
        environmentName, environmentNameProps,
        environmentNumbers, environmentNumbersProps
    }

    return {
        defineField, handleSubmit, errors, forms
    }
})