import { defineStore } from 'pinia'
import { ref, watchEffect } from 'vue'

interface BrowserStatus {
    uuid: string
    status: boolean
}

export const useBrowserStatusStore = defineStore('browserStatus', () => {
    const browserStatus = ref<Array<BrowserStatus>>([])

    const loadStatusFromLocalStorage = () => {
        const savedStatus = JSON.parse(localStorage.getItem('browser-status') || '[]')
        browserStatus.value = savedStatus
    }

    loadStatusFromLocalStorage()

    watchEffect(() => {
        window.addEventListener('storage', () => {
            loadStatusFromLocalStorage()
        })
    })

    const updateStatus = (uuid: string, status: boolean) => {
        const index = browserStatus.value.findIndex((item) => item.uuid === uuid)
        if (index !== -1) {
            browserStatus.value[index].status = status
        } else {
            browserStatus.value.push({ uuid, status })
        }
        localStorage.setItem('browser-status', JSON.stringify(browserStatus.value))
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
