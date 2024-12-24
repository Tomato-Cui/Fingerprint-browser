import { h } from 'vue'
import { More, MoreContent, MoreItem, MoreTrigger } from '@/components/more'
import { EllipsisIcon, Settings2Icon, SquarePenIcon, Trash2Icon } from 'lucide-vue-next'

export const MoreBtn = () => {
    return h(More,
        [
            h(MoreTrigger, h('div', { class: 'px-2' }, h(EllipsisIcon, { class: 'h-6 w-6 cursor-pointer' }))),
            h(MoreContent, [
                h(MoreItem, { class: 'cursor-pointer' }, [
                    h(Settings2Icon, { class: 'w-4 h-4' }),
                    '配置'
                ]),
                h(MoreItem, { class: 'cursor-pointer', },
                    [
                        h(SquarePenIcon, { class: 'w-4 h-4' }),
                        '更新'
                    ]),
                h(MoreItem, { class: 'cursor-pointer', }, [
                    h(Trash2Icon, { class: 'w-4 h-4' }),
                    '移除'
                ]),
            ])
        ]);
}