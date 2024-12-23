import {
    IconEnvironmentListActive,
    IconEnvironmentList,
    IconGroupActive,
    IconGroup,
    IconTransferActive,
    IconTransfer,
    IconTrashActive,
    IconTrash,
    IconTeamActive,
    IconTeam,
    IconProxyActive,
    IconProxy,
    IconAccountActive,
    IconAccount,
    IconSoftwareActive,
    IconSoftware,
    IconLogActive,
    IconLog,
    IconAutomationActive,
    IconAutomation,
    IconWindowSyncActive,
    IconWindowSync,
    IconGroupManagement,
    IconGroupManagementActive,
    IconAnyMember,
    IconAnyMemberActive,
    IconBlock,
    IconBlockActive,
    IconMembershipApplication,
    IconMembershipApplicationActive,
    IconRpa,
    IconApi,
    IconApiActive,
} from "@/assets/icons/index";

import { ChevronDown, Search as SearchIcon, Folder as FolderIcon, Component as ComponentIcon } from 'lucide-vue-next'
export const menuTop = [
    {
        title: "环境列表",
        url: "/environment",
        icon: IconEnvironmentList,
        activeIcon: IconEnvironmentListActive,
        active: true,
    },
    {
        title: "环境分组",
        url: "/divide",
        icon: IconGroup,
        activeIcon: IconGroupActive,
        active: false,
        children: [
            // {
            //     title: `未分组`,
            //     url: `/divide/0`,
            //     icon: ComponentIcon,
            //     activeIcon: ComponentIcon,
            //     active: true,
            // },
            // {
            //     title: `少云1组`,
            //     url: `/divide/1`,
            //     icon: ComponentIcon,
            //     activeIcon: ComponentIcon,
            //     active: true,
            // }
        ]
    },
    {
        title: "已转移环境",
        url: "/transferEnvironment",
        icon: IconTransfer,
        activeIcon: IconTransferActive,
        active: false,
    },
    {
        title: "回收站",
        url: "/recycle",
        icon: IconTrash,
        activeIcon: IconTrashActive,
        active: false,
    },
];

export const menuBottom = [
    {
        title: "团队管理",
        url: "/team",
        icon: IconTeam,
        activeIcon: IconTeamActive,
        active: false,
        children: [
            {
                title: "分组管理",
                url: "/team/group",
                icon: IconGroupManagement,
                activeIcon: IconGroupManagementActive,
                active: false,
            },
            {
                title: "全部成员",
                url: "/team/member",
                icon: IconAnyMember,
                activeIcon: IconAnyMemberActive,
                active: false,
            },
            {
                title: "黑名单",
                url: "/team/forbid",
                icon: IconBlock,
                activeIcon: IconBlockActive,
                active: false,
            },
            {
                title: "成员申请",
                url: "/team/apply",
                icon: IconMembershipApplication,
                activeIcon: IconMembershipApplicationActive,
                active: false,
            },
        ]
    },
    {
        title: "代理管理",
        url: "/mydaili",
        icon: IconProxy,
        activeIcon: IconProxyActive,
        active: false,
        children: [
            {
                title: `未分组`,
                url: `/mydaili/1`,
                icon: ComponentIcon,
                activeIcon: ComponentIcon,
                active: true,
            },
            {
                title: `分组2`,
                url: `/mydaili/2`,
                icon: ComponentIcon,
                activeIcon: ComponentIcon,
                active: true,
            }
        ]
    },
    {
        title: "我的账户",
        url: "/myAccount",
        icon: IconAccount,
        activeIcon: IconAccountActive,
        active: false,
    },
    {
        // title: "自动化",
        // url: "/auto",
        // icon: IconAutomation,
        // activeIcon: IconAutomationActive,
        // active: false,
        // children: [
        //     {
        //         title: "RPA",
        //         url: "/api/rpa",
        //         icon: IconRpa,
        //         activeIcon: IconRpa,
        //         active: false,
        //     },
        //     {
        //         title: "API",
        //         url: "/api",
        //         icon: IconApi,
        //         activeIcon: IconApiActive,
        //         active: false,
        //     },
        // ]
        title: "API",
        url: "/api",
        icon: IconApi,
        activeIcon: IconApiActive,
        active: false,
    },
    {
        title: "窗口同步",
        url: "/window-sync",
        icon: IconWindowSync,
        activeIcon: IconWindowSyncActive,
        active: false,
    },
    {
        title: "应用中心",
        url: "/application-center",
        icon: IconSoftware,
        activeIcon: IconSoftwareActive,
        active: false,
    },
    {
        title: "操作日志",
        url: "/mylog",
        icon: IconLog,
        activeIcon: IconLogActive,
        active: false,
    },
];