import { createRouter, createWebHistory } from 'vue-router';
import type { RouteRecordRaw } from 'vue-router';
import Login from '@/views/login/index.vue';
import group from '@/views/Teams/Group/index.vue';
import member from '@/views/Teams/Member/index.vue';
import forbid from '@/views/Teams/Forbid/index.vue';
import apply from '@/views/Teams/Apply/index.vue';
import personalCenter from '@/views/personalCenter/index.vue';
import account from '@/views/personalCenter/Account/index.vue';
import preference from '@/views/personalCenter/Preference/index.vue';
import local from '@/views/personalCenter/Local/index.vue';
import Api from '@/views/Api/index.vue';
import WindowSync from '@/views/window-sync/index.vue';
import EnvironmentActionLayout from '@/views/environment-action/layout.vue';
import EnvironmentActionCreate from '@/views/environment-action/views/create.vue';
import EnvironmentActionCreates from '@/views/environment-action/views/creates.vue';
import EnvironmentActionImports from '@/views/environment-action/views/imports.vue';
import myAccout from '@/views/myAccout/myaccout.vue';
import mydaili from '@/views/mydaili/mydaili.vue';
import adddaili from '@/views/adddaili/adddaili.vue';
import mylog from '@/views/mylog/mylog.vue';
import Divide from '@/views/Divide/index.vue';
import TransferEnvironment from '@/views/transferEnvironment/index.vue';
import Recycle from '@/views/Recycle/index.vue';
import NotFound from '@/views/not-found/index.vue';
import Environment from '@/views/EnvironmentN/index.vue'
import AccountSet from '@/views/system-operation/system-set/account-set.vue';
import NotifySet from '@/views/system-operation/system-set/notify-set.vue';
import GlobeSet from '@/views/system-operation/system-set/globe-set.vue'
import QuickStart from '@/views/system-operation/help-center/quick-start.vue'; 
import { EnvironmentSimpleCreate, EnvironmentAdvancedCreate, EnvironmentBatchImport, EnvironmentLayout } from '@/views/environment-creates/index'
import EnvironmentTrash from '@/views/environment-trash/index.vue'
import Extensions from '@/views/extensions/index.vue'
import EnvironmentManagerBookmark from '@/views/environment-manager-bookmark/index.vue'
import EnvironmentGroupManager from '@/views/environment-group-manager/index.vue'
import ProxyManage from '@/views/proxy-manage/index.vue'
import Team from '@/views/team/index.vue';

import EmptyProxy from '@/views/proxy-manage/empty-proxy.vue'

import SingleNewProxy from '@/views/proxy-manage/create-proxy-view/single-new-proxy.vue'
import BatchNewProxy from '@/views/proxy-manage/create-proxy-view/batch-new-proxy.vue'


const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'login',
    component: Login,
  },
  {
    path: '/environment/:id',
    name: 'environment',
    component: Environment,
  },
  {
    path: '/environment/create',
    name: 'environment/create',
    component: EnvironmentLayout,
    children:
      [

        {
          path: 'environment-simple-create',
          name: 'simple-create',
          component: EnvironmentSimpleCreate,
        },
        {
          path: 'environment-advanced-create',
          name: 'advanced-create',
          component: EnvironmentAdvancedCreate,
        },
        {
          path: 'environment-batch-import',
          name: 'batch-import',
          component: EnvironmentBatchImport,
        },
      ]
  },
  {
    path: '/environment-trash',
    name: 'environment-trash',
    component: EnvironmentTrash,
  },
  {
    path: '/environment-manager-bookmark',
    name: 'environment-manager-bookmark',
    component: EnvironmentManagerBookmark,
  },
  {
    path: '/environment-group-manager',
    name: 'environment-group-manager',
    component: EnvironmentGroupManager,
  },
  {
    path: '/proxy-manage',
    name: 'proxy-manage',
    component: ProxyManage,
  },
  {
    path: '/enpty-proxy',
    name: 'enpty-proxy',
    component: EmptyProxy,
  },
  {
    path: '/single-new-proxy',
    name: 'single-new-proxy',
    component: SingleNewProxy,
  },
  {
    path: '/batch-new-proxy',
    name: 'batch-new-proxy',
    component: BatchNewProxy,
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
    component: Extensions,
  },
  {
    path: '/team/:id?',
    name: '/team',
    component: Team,
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
  {
    path: '/accountSet',
    name: 'accountSet',
    component: AccountSet,
  },
  {
    path: '/notifySet',
    name: 'notifySet',
    component: NotifySet,
  },
  {
    path: '/globeSet',
    name: 'globeSet',
    component: GlobeSet,
  },
  {
    path: '/quickStart',
    name: 'quickStart',
    component: QuickStart
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
