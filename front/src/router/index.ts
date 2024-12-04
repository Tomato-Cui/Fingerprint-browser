// 导入Vue Router相关方法和类型
import { createRouter, createWebHistory } from 'vue-router';
import type { RouteRecordRaw } from 'vue-router';  // 使用 `import type` 导入类型

// 导入各个页面组件
import newBrowser from '@/views/newBrowser/index.vue';  // 新建浏览器页面
import environment from '@/views/Environment/index.vue'; // 环境管理页面
import divide from '@/views/divide/index.vue';          // 分组管理页面
import agency from '@/views/Agency/index.vue';          // 代理管理页面
import plugIn from '@/views/plugIn/index.vue';         // 插件管理页面
import recycle from '@/views/Recycle/index.vue';       // 回收站页面
import api from '@/views/Api/index.vue';              // API页面
import team from '@/views/team/index.vue';             // 团队管理页面
import login from '@/views/login/index.vue';
// 登录页面
// 团队管理子页面
import group from '@/views/Team/Group/index.vue';      // 分组管理页面
import member from '@/views/Team/Member/index.vue';    // 所有成员页面
import forbid from '@/views/Team/Forbid/index.vue';    // 已禁止页面
import apply from '@/views/Team/Apply/index.vue';      // 成员申请页面
// 个人中心页面
import personalCenter from '@/views/personalCenter/index.vue'; // 个人中心页面
import account from '@/views/personalCenter/Account/index.vue'; // 我的账户
import preference from '@/views/personalCenter/Preference/index.vue'; // 偏好设置
import local from '@/views/personalCenter/Local/index.vue'; // 本地设置



// 定义路由配置数组
const routes: Array<RouteRecordRaw> = [
  // 登录路由
  {
    path: '/',      // 路由路径
    name: 'login',       // 路由名称
    component: login,    // 对应组件
  },

  // 新建浏览器路由 (默认首页)
  {
    path: '/home',          // 根路径作为默认路由
    name: 'newBrowser',
    component: newBrowser,
  },
  // 环境管理路由
  {
    path: '/environment',
    name: 'environment',
    component: environment,
  },
  // 分组管理路由
  {
    path: '/divide',
    name: 'divide',
    component: divide,
  },
  // 代理管理路由
  {
    path: '/agency',
    name: 'agency',
    component: agency,
  },
  // 插件管理路由
  {
    path: '/plugIn',
    name: 'plugIn',
    component: plugIn,
  },
  // 回收站路由
  {
    path: '/recycle',
    name: 'recycle',
    component: recycle,
  },
  // API管理路由
  {
    path: '/api',
    name: 'api',
    component: api,
  },
  // 团队管理路由
  {
    path: '/team',
    name: 'team',
    component: team,
    children: [
      {
        path: 'group',    // 动态路由参数
        name: 'group',
        component: group,
      },
      {
        path: 'member',
        name: 'member',
        component: member,
      },
      {
        path: 'forbid',
        name: 'forbid',
        component: forbid,
      },
      {
        path: 'apply',
        name: 'apply',
        component: apply,
      },
    ],
  },
  // 个人中心路由
  {
    path: '/personalCenter',
    name: 'personalCenter',
    component: personalCenter,
    redirect: { name: 'account' },
    children: [
      {
        path: 'account',
        name: 'account',
        component: account,
      },
      {
        path: 'preference',
        name: 'preference',
        component: preference,
      },
      {
        path: 'local',
        name: 'local',
        component: local,
      },
    ],
  },
];

// 创建路由实例
const router = createRouter({
  history: createWebHistory(),  // 使用HTML5历史模式
  routes,

  // 路由配置
});

router.beforeEach(async (to, from) => {
  let token = localStorage.getItem('token');
  if (
    !token && to.name !== 'login'
  ) {
    return { name: 'login' }
  }
})


// 导出路由实例
export default router;
