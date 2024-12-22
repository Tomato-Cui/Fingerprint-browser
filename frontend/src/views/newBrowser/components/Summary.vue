<template>
  <div class="summary">
    <div class="header">
      <span>概要</span>
      <a href="http://#" id="a">生成新指纹</a>
    </div>
    <div class="main">
      <li>
        <span>浏览器</span>
        <div>Google Chrome128</div>
      </li>
      <li>
        <span>User-Agent</span>
        <div>Google Chrome128as dfasdfasdfasfdasfdasf</div>
      </li>
      <li>
        <span>浏览器</span>
        <div>Google Chrome128</div>
      </li>
      <li>
        <span>浏览器</span>
        <div>Google Chrome128</div>
      </li>
      <li>
        <span>浏览器</span>
        <div>Google Chrome128</div>
      </li>
      <li>
        <span>浏览器</span>
        <div>Google Chrome128</div>
      </li>
      <li>
        <span>浏览器</span>
        <div>Google Chrome128</div>
      </li>
      <li>
        <span>浏览器</span>
        <div>Google Chrome128</div>
      </li>
      <li>
        <span>浏览器</span>
        <div>Google Chrome128</div>
      </li>
      <li>
        <span>浏览器</span>
        <div>Google Chrome128</div>
      </li>
    </div>
  </div>
</template>

<script setup>
</script>

<style scoped>
.summary {
  width: 100%;
  height: 80%;
  background-color: #f7f8fa;
  border-radius: 8px 8px 8px 8px;
  padding: 14px;
  margin-top: 26px;
  box-sizing: border-box;
}
.header {
  height: 40px;
  line-height: 40px;
  border-bottom: 1px solid #d8d8d8;
}
.main {
  margin-top: 16px;
}
.main li {
  border-bottom: 1px solid #e6e6e6;
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 14px;
}
.main li div {
  width: 50%;
  color: #4f5866;
  float: right;
}
.main span {
  width: 50%;
}
#a {
  padding-right: 10px;
  float: right;
  color: #4f5866;
  text-decoration: none;
}
</style>