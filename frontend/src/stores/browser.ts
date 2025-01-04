import { defineStore } from 'pinia'
import { ref } from 'vue'

interface BrowserStatus {
    uuid: string
    status: boolean
}

export const useBrowserStatusStore = defineStore('browserStatus', () => {
    const browserStatus = ref<Array<BrowserStatus>>([])

    const updateStatus = (uuid: string, status: boolean) => {
        const index = browserStatus.value.findIndex((item) => item.uuid === uuid)
        if (index !== -1) {
            browserStatus.value[index].status = status
        } else {
            browserStatus.value.push({ uuid, status })
        }
    }

    const getStatus = (uuid: string): boolean | undefined => {
        const browser = browserStatus.value.find((item) => item.uuid === uuid)
        return browser?.status
    }

    return {
        browserStatus,
        updateStatus,
        getStatus,
    }
})
