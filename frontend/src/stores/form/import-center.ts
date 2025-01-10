import { defineStore } from "pinia";
import { useForm } from "vee-validate";


interface ImportInfo {
    file: File
}

export const useImportCenterFromStore = defineStore('import-center-form-store', () => {

    const {
        defineField, handleSubmit, errors
    } = useForm<ImportInfo>({
        validationSchema: {
            file: (file?: File) => {
                if (!file) {
                    return "请传输文件";
                }
                return true;
            },
        },
    });


    const [file, fileProps] = defineField('file');

    const forms = {
        file, fileProps
    };

    return {
        defineField, handleSubmit, errors, forms
    }
})