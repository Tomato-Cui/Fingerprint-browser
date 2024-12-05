<!-- 本地设置 -->
<template>
  <div class="footer">
    <el-button @click="submit">保存设置</el-button>
  </div>
  <div class="main-container">
    <div class="local-setting-title">
      <div style="font-size: 18px; font-weight: 800; margin: 15px 0">
        浏览器环境
      </div>
      <el-row>
        <el-col :span="8"> 窗口位置 </el-col>
        <el-col :span="16">
          <el-radio-group
            v-model="form.windowPosition"
            style="display: flex; flex-direction: column"
          >
            <el-radio label="0" :value="0">始终在屏幕左上角打开</el-radio>
            <el-radio label="1" :value="1">在上一次关闭位置打开</el-radio>
            <el-radio label="2" :value="2">以任务栏最小化的方式打开</el-radio>
            <el-radio label="3" :value="3">以屏幕最大化方式打开</el-radio>
          </el-radio-group>
        </el-col>
      </el-row>
      <el-row>
        <el-col :span="8"> FB静态资源 </el-col>
        <el-col :span="16" style="display: flex; flex-direction: column">
          <el-checkbox v-model="form.useFBGlobalSettings">
            使用【全局设置-FB静态资源】的设置
          </el-checkbox>
          <el-checkbox
            v-model="form.useLocalNetworkLoadFBStaticResource"
            v-if="!form.useFBGlobalSettings"
          >
            当前电脑使用本地网络加载FB静态资源
          </el-checkbox>
        </el-col>
      </el-row>
      <el-row>
        <el-col :span="8">性能</el-col>
        <el-col :span="16">
          <el-switch
            v-model="form.useHardwareAcceleration"
            active-text="使用硬件加速模式"
          />
          <div style="font-size: 12px">
            启用该选项后，不同的设备可能会影响硬件相关的指纹
          </div>
          <el-switch
            v-model="form.useUIAutomationTool"
            active-text="使用UI自动化工具"
          />
          <div style="font-size: 12px">
            启用该选项后，可兼容影刀RPA、UiPath等自动化工具
          </div>
        </el-col>
      </el-row>
    </div>
    <div class="RPA-task-setting">
      <div style="font-size: 18px; font-weight: 800; margin: 15px 0">
        RPA任务
      </div>
      <el-row>
        <el-col :span="8">任务线程数</el-col>
        <el-col :span="16">
          <el-input-number
            v-model="form.rpaTaskThread"
            :min="0"
            :max="100"
            :step="1"
            controls-position="right"
          />
        </el-col>
      </el-row>
      <el-row>
        <el-col :span="8">自动删除日志</el-col>
        <el-col :span="16">
          <el-switch v-model="form.autoDeleteLog" active-text="每日自动删除" />
          <el-input-number
            v-model="form.autoDeleteLogDays"
            :min="0"
            :max="30"
            :step="1"
            controls-position="right"
            :disabled="!form.autoDeleteLog"
          />
          <div>天前的日志</div>
        </el-col>
      </el-row>
    </div>
    <div class="cache-setting">
      <div style="font-size: 18px; font-weight: 800; margin: 15px 0">
        缓存设置
      </div>
      <el-row>
        <el-col :span="8">缓存信息</el-col>
        <el-col :span="16">
          <div>记录打开过的环境缓存，下次再打开会提高加载速度</div>
          <div>
            <el-input v-model="form.cacheDirectory" disabled />
            <el-input type="file" v-model="form.cacheDirectory" />
          </div>
          <el-link type="primary">自定义数据缓存目录</el-link>
          <div style="color: red; font-size: 12px">当前磁盘使用率: 19%</div>
        </el-col>
      </el-row>
      <el-row>
        <el-col :span="8">缓存空间</el-col>
        <el-col :span="16">
          <div>当硬盘空间不足</div>
          <el-input-number
            v-model="form.diskSpace"
            :min="0"
            :max="100"
            :step="1"
            controls-position="right"
          />
          <div>M时，禁止打开浏览器</div>
        </el-col>
      </el-row>
      <el-row>
        <el-col :span="8">备份快照</el-col>
        <el-col :span="16">
          <el-switch
            v-model="form.autoBackup"
            active-text="在此电脑上自动备份"
          />
          <div>
            每次关闭浏览器时，自动备份Local
            storage、IndexDB、扩展插件、历史记录的数据。
          </div>
        </el-col>
      </el-row>
      <el-row>
        <el-col :span="8">删除缓存</el-col>
        <el-col :span="16">
          <el-checkbox v-model="form.autoDeleteCache">自动删除</el-checkbox>
          <el-input-number
            v-model="form.autoDeleteCacheDays"
            :min="0"
            :max="30"
            :step="1"
            controls-position="right"
            :disabled="!form.autoDeleteCache"
          />
          <div>天未打开的环境的数据</div>
          <div>
            <el-button v-if="form.autoDeleteCache">立即删除</el-button>
            <el-button>删除全部缓存</el-button>
          </div>
        </el-col>
      </el-row>
    </div>
    <div class="version-info">
      <div style="font-size: 18px; font-weight: 800; margin: 15px 0">
        版本信息
      </div>
      <el-row>
        <el-col :span="8">软件版本</el-col>
        <el-col :span="16">
          <div>Version 6.10.20</div>
          <div>
            <el-link type="primary">立即安装</el-link>
            <div>
              <el-icon color="green"><Check /></el-icon>已是最新
            </div>
          </div>
        </el-col>
      </el-row>
      <el-row>
        <el-col :span="8">补丁版本</el-col>
        <el-col :span="16">
          <div>Version 6.10.20</div>
          <div>
            <el-link type="primary">立即安装</el-link>
            <div>
              <el-icon color="green"><Check /></el-icon>已是最新
            </div>
          </div>
        </el-col>
      </el-row>
      <el-row>
        <el-col :span="8">SunBrowser</el-col>
        <el-col :span="16">
          <ul>
            <li>
              <div>Version 6.10.20</div>
              <div>
                <el-link type="primary">立即安装</el-link>
                <div>
                  <el-icon color="green"><Check /></el-icon>已是最新
                </div>
              </div>
            </li>
            <li>
              <div>Version 6.10.20</div>
              <div>
                <el-link type="primary">立即安装</el-link>
                <div>
                  <el-icon color="green"><Check /></el-icon>已是最新
                </div>
              </div>
            </li>
          </ul>
        </el-col>
      </el-row>
      <el-row>
        <el-col :span="8">FlowerBrowser</el-col>
        <el-col :span="16">
          <ul>
            <li>
              <div>Version 6.10.20</div>
              <div>
                <el-link type="primary">立即安装</el-link>
                <div>
                  <el-icon color="green"><Check /></el-icon>已是最新
                </div>
              </div>
            </li>
            <li>
              <div>Version 6.10.20</div>
              <div>
                <el-link type="primary">立即安装</el-link>
                <div>
                  <el-icon color="green"><Check /></el-icon>已是最新
                </div>
              </div>
            </li>
          </ul>
        </el-col>
      </el-row>
    </div>
    <div class="other-setting">
      <div style="font-size: 18px; font-weight: 800; margin: 15px 0">
        其他设置
      </div>
      <el-row>
        <el-col :span="8">页面缩放</el-col>
        <el-col :span="16">
          <el-switch
            v-model="form.autoAdaptSystemScale"
            active-text="开启后，自动适配系统缩放比例"
          />
          <div v-if="!form.autoAdaptSystemScale">
            <el-input-number
              v-model="form.systemScale"
              :min="50"
              :max="150"
              :step="1"
              controls-position="right"
            />
            <el-slider v-model="form.systemScale" :min="50" :max="150" />
          </div>
        </el-col>
      </el-row>
      <el-row>
        <el-col :span="8">系统日志</el-col>
        <el-col :span="16">
          <el-button>查看详细</el-button>
        </el-col>
      </el-row>
      <el-row>
        <el-col :span="8">网络检测</el-col>
        <el-col :span="16">
          <div>
            <el-button>检测本地网络</el-button>
            <div style="color: greenyellow; font-size: 12px">
              当前本地网络为：125.119.27.182 cn / zhejiang / hangzhou
            </div>
          </div>
          <div>
            <el-button>检测系统代理</el-button>
            <div style="color: greenyellow; font-size: 12px">
              当前本地网络为：125.119.27.182 cn / zhejiang / hangzhou
            </div>
          </div>
        </el-col>
      </el-row>
      <el-row>
        <el-col :span="8">网络连接</el-col>
        <el-col :span="16">
          <el-switch
            v-model="form.useSystemProxy"
            active-text="使用系统代理连接代理"
          />
        </el-col>
      </el-row>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from "vue";
import { ElMessage } from "element-plus";

const submit = async () => {//提交设置
  ElMessage.warning("提交");
};
const form = reactive({
  windowPosition: 0,
  useFBGlobalSettings: false, // 是否使用FB全局设置
  useLocalNetworkLoadFBStaticResource: false, // 是否使用本地网络加载FB静态资源
  useHardwareAcceleration: false, // 使用硬件加速
  useUIAutomationTool: false, // 使用UI自动化工具
  rpaTaskThread: 0, // RPA任务线程数
  autoDeleteLog: false, // 是否自动删除日志
  autoDeleteLogDays: 0, // 自动删除多少天前的日志
  cacheDirectory: "", // 缓存目录
  diskSpace: 0, // 硬盘不足多少时，禁止打开浏览器
  autoBackup: false, // 是否自动备份
  autoDeleteCache: false, // 是否自动删除缓存
  autoDeleteCacheDays: 0, // 自动删除多少天前的缓存
  autoAdaptSystemScale: false, // 是否开启自动适配系统缩放比例
  systemScale: 0, // 设置缩放比例
  useSystemProxy: false, // 是否使用系统代理连接代理
});
</script>

<style scoped lang="less">
.footer {
  position: fixed;
  bottom: 0;
  width: 100%;
  height: 80px;
  display: flex;
  align-items: center;
  // left: 300px;
  background-color: #ffffff;
  padding-left: 50px;
  :deep(.el-button) {
    width: 100px;
    height: 40px;
    background-color: #1e4cfc;
    color: #fff;
    font-size: 18px;
    border-radius: 10px;
  }
}

.main-container{
  width: 800px;
}
</style>
