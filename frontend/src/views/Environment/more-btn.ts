import { h } from 'vue'
import { More, MoreContent, MoreItem, MoreTrigger } from '@/components/more'
import { EllipsisIcon, Settings2Icon, SquarePenIcon, Trash2Icon, CircleUser, Activity, UserCheck, Rotate3d, Star } from 'lucide-vue-next'

export const MoreBtn = (props: any) => {

    const { transferEnvBtn, authMemberBtn, editProxyBtn, editAccountBtn, editEnvBtn, removeEnv } = props;

    return h(More,
        [
            h(MoreTrigger, h('div', { class: 'px-2' }, h(EllipsisIcon, { class: 'h-6 w-6 cursor-pointer' }))),
            h(MoreContent, [
                h(MoreItem, { class: 'cursor-pointer' }, [
                    h(Settings2Icon, { class: 'w-4 h-4' }),
                    '详情'
                ]),
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
                    '修改代理'
                ]),
                h(MoreItem, { class: 'cursor-pointer', onClick: () => authMemberBtn() }, [
                    h(UserCheck, { class: 'w-4 h-4' }),
                    '授权成员'
                ]),
                h(MoreItem, { class: 'cursor-pointer', onClick: () => transferEnvBtn() }, [
                    h(Rotate3d, { class: 'w-4 h-4' }),
                    '转移环境'

                ]),
                // h(MoreItem, { class: 'cursor-pointer', onClick: () => setCommonBtn() }, [
                //     h(Star, { class: 'w-4 h-4' }),
                //     '设为常用'
                // ]),

                h(MoreItem, { class: 'cursor-pointer', onClick: () => removeEnv() }, [
                    h(Trash2Icon, { class: 'w-4 h-4' }),
                    '移除'
                ])

            ])
        ]
    )
}
