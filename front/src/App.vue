<template>
  <!-- <Home /> -->
  <router-view />
  <div class="mb-4">
    <el-button type="warning" :onclick="openBrowserHandle">打开浏览器</el-button>
    <el-button type="danger" :onclick="closeBrowserHandle">关闭浏览器</el-button>
    <el-button type="danger" :onclick="statusBrowserHandle">浏览器状态</el-button>
  </div>
</template>

<script setup lang="ts">
import { starts, stops, status } from '@/command/environment'

let environment = {
  "ID": 1, // TODO: 注意这里一定要大写.
  "name": "DEV Environment",
  "owner_id": "owner_123",
  "domain_name": "example.com",
  "open_urls": "http://example.com, http://test.com",
  "repeat_config": "{\"repeat\": true, \"max_retries\": 3}",
  "username": "testuser",
  "password": "testpassword123",
  "fakey": "2FASecretKey",
  "cookie": "sessionid=abc123; user=admin;",
  "ignore_cookie_error": 0,
  "group_id": 2,
  "fp_info_id": 101,
  "ua": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
  "os": "Windows 10",
  "country": "United States",
  "region": "California",
  "city": "Los Angeles",
  "remark": "This is a test environment",
  "ipchecker": "ip-api.com",
  "sys_app_cate_id": "12345",
  "user_proxy_config": "{\"proxy_type\": \"SOCKS5\", \"proxy_address\": \"proxy.example.com\"}",
  "proxy": "192.168.0.1:1080",
  "proxy_enable": 1,
  "is_tz": 1,
  "is_pos": 1,
  "user_data_file": "/path/to/user_data",
  "driver_location": "/path/to/chromedriver",
  "status": 0,
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-02T15:00:00Z",
  "deleted_at": null
};
let fingerprint = {
  "id": 1,
  "ua_version": 105,
  "ua": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36",
  "language_type": 1,
  "languages": "en-US,en;q=0.9",
  "gmt": "GMT+8",
  "geography": "Asia/Shanghai",
  "geo_tips": 1,
  "geo_rule": 0,
  "longitude": "121.4737",
  "latitude": "31.2304",
  "radius": 50,
  "height": 1080,
  "width": 1920,
  "fonts_type": 1,
  "fonts": "Arial, Helvetica, sans-serif",
  "font_fingerprint": 1,
  "web_rtc": 3,
  "web_rtc_local_ip": "192.168.1.1",
  "canvas": 0,
  "webgl": 1,
  "hardware_acceleration": 1,
  "webgl_info": 0,
  "audio_context": 0,
  "speech_voices": 1,
  "media": 0,
  "cpu": 8,
  "memory": 8,
  "do_not_track": 1,
  "battery": 1,
  "port_scan": 0,
  "white_list": "192.168.1.1, 192.168.1.2",
  "created_at": "2024-01-01T10:00:00Z",
  "updated_at": "2024-01-10T12:00:00Z",
  "deleted_at": null
}

const openBrowserHandle = async () => {
  await starts([{ environment: environment, fingerprint: fingerprint }])
}
const closeBrowserHandle = async () => {
  await stops([1])
}
const statusBrowserHandle = async () => {
  let browser_status = await status()
  console.log(browser_status);
}
</script>

<style>
body {
  margin: 0px;
}
</style>
