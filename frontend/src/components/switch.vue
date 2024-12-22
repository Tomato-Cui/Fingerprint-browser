<script setup lang="ts">
import { computed, type HTMLAttributes } from 'vue'
import { Switch } from '@/components/ui/switch'
import { cn } from '@/util/lib'
import {
    type SwitchRootEmits,
    type SwitchRootProps,
    useForwardPropsEmits,
} from 'radix-vue'

const props = defineProps<SwitchRootProps & { class?: HTMLAttributes['class'] }>()

const emits = defineEmits<SwitchRootEmits>()

const delegatedProps = computed(() => {
    const { class: _, ...delegated } = props

    return delegated
})

const forwarded = useForwardPropsEmits(delegatedProps, emits)
</script>
<template>
    <Switch v-bind="forwarded" :class="cn('', props.class)" />
</template>
