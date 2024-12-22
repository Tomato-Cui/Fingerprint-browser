<script setup>
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@/components/ui/tooltip";

const props = defineProps({
    title: "",
    class: "",
});

const emit = defineEmits(['click'])

const clickHandle = (event) => {
    emit('click', event)
}
</script>

<template>
    <Tooltip>
        <TooltipTrigger>
            <button @click="clickHandle"
                :class="`${props.class}`">
                <slot />
            </button>
        </TooltipTrigger>
        <TooltipContent class="bg-black bg-opacity-50 text-white p-2 rounded">
            <p>{{ props.title }}</p>
        </TooltipContent>
    </Tooltip>
</template>