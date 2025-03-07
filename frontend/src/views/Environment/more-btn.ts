import { h } from 'vue'
import { More, MoreContent, MoreItem, MoreTrigger } from '@/components/more'
import { EllipsisIcon, SquarePenIcon, Trash2Icon, CircleUser, Activity, Rotate3d } from 'lucide-vue-next'

export const MoreBtn = (props: any) => {

    const { transferEnvBtn, editProxyBtn, editAccountBtn, editEnvBtn, removeEnv } = props;

    return h(More,
        [
            h(MoreTrigger, h('div', { class: 'px-2' }, h(EllipsisIcon, { class: 'h-6 w-6 cursor-pointer' }))),
            h(MoreContent, [
                h(MoreItem, { class: 'cursor-pointer', onClick: () => editEnvBtn() }, [
                    h(SquarePenIcon, { class: 'w-4 h-4' }),
                    '编辑环境'
                ]),
                h(MoreItem, { class: 'cursor-pointer', onClick: () => editAccountBtn() }, [
                    h(CircleUser, { class: 'w-4 h-4' }),
                    '编辑账号'
                ]),
                h(MoreItem, { class: 'cursor-pointer', onClick: () => editProxyBtn() }, [
                    h(Activity, { class: 'w-4 h-4' }),
                    '编辑代理'
                ]),
                h(MoreItem, { class: 'cursor-pointer', onClick: () => transferEnvBtn() }, [
                    h(Rotate3d, { class: 'w-4 h-4' }),
                    '转移环境'

                ]),
                h(MoreItem, { class: 'cursor-pointer', onClick: () => removeEnv() }, [
                    h(Trash2Icon, { class: 'w-4 h-4' }),
                    '移除'
                ])
            ])
        ]
    )
}
