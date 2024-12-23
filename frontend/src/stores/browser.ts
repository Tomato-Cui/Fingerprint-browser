import { defineStore } from 'pinia'
import { ref, watchEffect } from 'vue'

interface BrowserStatus {
    id: number
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

    const updateStatus = (id: number, status: boolean) => {
        const index = browserStatus.value.findIndex((item) => item.id === id)
        if (index !== -1) {
            browserStatus.value[index].status = status
        } else {
            browserStatus.value.push({ id, status })
        }
        localStorage.setItem('browser-status', JSON.stringify(browserStatus.value))
    }

    const getStatus = (id: number): boolean | undefined => {
        const browser = browserStatus.value.find((item) => item.id === id)
        return browser?.status
    }

    return {
        browserStatus,
        updateStatus,
        getStatus,
    }
})
