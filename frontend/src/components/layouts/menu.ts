import {
    IconPlate,
    IconTrashBin,
    IconExtensions,
    IconUsersGroupRounded,
    IconHomeSmile,
} from "@/assets/icons/sidebar/index";

export const menus = [
    {
        title: "环境管理",
        url: "/environment/0",
        icon: IconHomeSmile,
        isActive: true,
    },
    {
        title: "代理",
        url: "/private-proxy",
        icon: IconPlate,
    },
    {
        title: "回收站",
        url: "/environment-trash",
        icon: IconTrashBin,
    },
    {
        title: "拓展",
        url: "/extensions",
        icon: IconExtensions,
    },
    {
        title: "团队",
        url: "/team/",
        icon: IconUsersGroupRounded,
    },
];