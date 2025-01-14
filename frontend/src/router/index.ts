import { createRouter, createWebHistory } from 'vue-router';
import type { RouteRecordRaw } from 'vue-router';
import Login from '@/views/login/index.vue';
import group from '@/views/Team/Group/index.vue';
import member from '@/views/Team/Member/index.vue';
import forbid from '@/views/Team/Forbid/index.vue';
import apply from '@/views/Team/Apply/index.vue';
import personalCenter from '@/views/personalCenter/index.vue';
import account from '@/views/personalCenter/Account/index.vue';
import preference from '@/views/personalCenter/Preference/index.vue';
import local from '@/views/personalCenter/Local/index.vue';
import Api from '@/views/Api/index.vue';
import WindowSync from '@/views/window-sync/index.vue';
import ApplicationCenter from '@/views/application-center/index.vue';
import EnvironmentActionLayout from '@/views/environment-action/layout.vue';
import EnvironmentActionCreate from '@/views/environment-action/views/create.vue';
import EnvironmentActionCreates from '@/views/environment-action/views/creates.vue';
import EnvironmentActionImports from '@/views/environment-action/views/imports.vue';
import myAccout from '@/views/myAccout/myaccout.vue';
import mydaili from '@/views/mydaili/mydaili.vue';
import adddaili from '@/views/adddaili/adddaili.vue';
import mylog from '@/views/mylog/mylog.vue';
import Environment from '@/views/Environment/index.vue';
import Divide from '@/views/Divide/index.vue';
import TransferEnvironment from '@/views/transferEnvironment/index.vue';
import Recycle from '@/views/Recycle/index.vue';
import NotFound from '@/views/not-found/index.vue';
import EnvironmentN from '@/views/EnvironmentN/index.vue'

import extensions from '@/views/extensions/index.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'login',
    component: Login,
  },
  {
    path: '/environment',
    name: 'environment',
    component: EnvironmentN,
  },
  {
    path: '/environment-action',
    name: 'environment-action',
    component: EnvironmentActionLayout,
    children: [
      {
        path: 'create',
        name: 'create',
        component: EnvironmentActionCreate,
      },
      {
        path: 'creates',
        name: 'creates',
        component: EnvironmentActionCreates,
      },
      {
        path: 'imports',
        name: 'imports',
        component: EnvironmentActionImports,
      },
    ],
  },
  {
    path: '/divide/:id',
    name: 'divide',
    component: Divide,
  },
  {
    path: '/transferEnvironment',
    name: 'transferEnvironment',
    component: TransferEnvironment,
  },
  {
    path: '/recycle',
    name: 'recycle',
    component: Recycle,
  },
  {
    path: '/window-sync',
    name: 'window-sync',
    component: WindowSync,
  },
  {
    path: '/extensions',
    name: 'extensions',
    component: extensions,
  },
  {
    path: '/team/group',
    name: '/team/group',
    component: group,
  },
  {
    path: '/team/member',
    name: '/team/member',
    component: member,
  },
  {
    path: '/team/forbid',
    name: '/team/forbid',
    component: forbid,
  },
  {
    path: '/team/apply',
    name: '/team/apply',
    component: apply,
  },
  {
    path: '/myAccount',
    name: 'myAccout',
    component: myAccout,
  },
  {
    path: '/mydaili',
    name: 'mydaili',
    component: mydaili,
  },
  {
    path: '/mylog',
    name: 'mylog',
    component: mylog,
  },
  {
    path: '/api',
    name: 'api',
    component: Api,
  },
  {
    path: '/adddaili',
    name: 'adddaili',
    component: adddaili,
  },
  {
    path: '/personal',
    name: 'personal',
    component: personalCenter,
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
  { path: '/:pathMatch(.*)*', name: 'NotFound', component: NotFound },
];

// 创建路由实例
const router = createRouter({
  // 使用 HTML5 history 模式
  history: createWebHistory(),
  // 路由配置
  routes,
});

export default router;
