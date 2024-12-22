<script setup>
import { SidebarProvider, SidebarTrigger, useSidebar } from '@/components/ui/sidebar/index'
import AppSidebar from './app-sidebar.vue'
import AppTopbar from './app-topbar/index.vue'
</script>

<template>
    <SidebarProvider>
        <AppSidebar />
        <main class="`w-full md:w-[calc(100%-var(--sidebar-width))] relative`">
            <div class="h-screen overflow-hidden">
                <AppTopbar />
                <div class="bg-[#F9F9F9FF] overflow-auto">
                    <slot></slot>
                </div>
            </div>
        </main>
    </SidebarProvider>
</template>
