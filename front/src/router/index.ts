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

  },
];

// 创建路由实例
const router = createRouter({
  history: createWebHistory(),  // 使用HTML5历史模式
  routes,

  // 路由配置
});

router.beforeEach(async (to, from) => {
  if (
    // 检查用户是否已登录
    !Authorization &&
    // ❗️ 避免无限重定向
    to.name !== 'Login'
  ) {
    // 将用户重定向到登录页面
    return { name: 'Login' }
  }
})


// 导出路由实例
export default router;
