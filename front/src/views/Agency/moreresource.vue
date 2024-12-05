<template>
  <div class="page">
    <!-- Information Section -->
    <el-alert
      type="info"
      show-icon
      title="官方仅展示，代理服务质量请自行判断。如在使用代理IP过程中存在问题请咨询平台方客服。"
    ></el-alert>

    <!-- Card Grid -->
    <el-row :gutter="20" class="card-grid">
      <el-col :span="6" v-for="card in cards" :key="card.id">
        <el-card class="proxy-card" shadow="always">
          <img :src="card.image" alt="" class="proxy-img" />
          <div class="proxy-info">
            <h3>{{ card.title }}</h3>
            <p>{{ card.description }}</p>
            <el-button
              v-if="card.actionText"
              size="small"
              type="primary"
              @click="card.action"
            >
              {{ card.actionText }}
            </el-button>
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script>
import { ref } from "vue";

export default {
  name: "App",
  setup() {
    const activeTab = ref("more-resources");

    // Proxy card data
    const cards = ref([
      {
        id: 1,
        title: "kookeey",
        description: "低至 4.5 元/G",
        image: "https://via.placeholder.com/150",
        actionText: "立即抢购",
        action: () => alert("购买 kookeey"),
      },
      {
        id: 2,
        title: "PIA S5 Proxy X AdsPower",
        description: "免费 IP 1000 个 + 额外流量 200GB",
        image: "https://via.placeholder.com/150",
        actionText: "领取优惠",
        action: () => alert("领取优惠"),
      },
      {
        id: 3,
        title: "lumiproxy",
        description: "90M+ 优秀住宅 IP 代理",
        image: "https://via.placeholder.com/150",
        actionText: "立即抢购",
        action: () => alert("购买 lumiproxy"),
      },
      {
        id: 4,
        title: "Smartdali X AdsPower",
        description: "免费提供高质量的动态住宅代理",
        image: "https://via.placeholder.com/150",
        actionText: "立即使用",
        action: () => alert("使用 Smartdali"),
      },
    ]);

    return {
      activeTab,
      cards,
    };
  },
};
</script>

<style scoped>
.page {
  padding: 20px;
  font-family: Arial, sans-serif;
  background: #f5f5f5;
}

.card-grid {
  margin-top: 20px;
}

.proxy-card {
  text-align: center;
}

.proxy-img {
  width: 100%;
  height: auto;
  margin-bottom: 15px;
}

.proxy-info {
  text-align: left;
}

.proxy-info h3 {
  margin: 0 0 10px;
}

.proxy-info p {
  margin: 0 0 15px;
  font-size: 14px;
  color: #666;
}
</style>
