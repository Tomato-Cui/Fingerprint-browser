<template>
    <div class="qrcode-container">
      <canvas ref="qrCanvas"></canvas>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted, watch, defineProps } from 'vue';
  import QRCode from 'qrcode';
  
  // 定义 QRCanvas 引用的类型为 HTMLCanvasElement
  const qrCanvas = ref<HTMLCanvasElement | null>(null);
  
  // 定义组件的 props 类型
  interface QRCodeProps {
    value: string;
  }
  
  // 通过 defineProps 定义传入的 props
  const props = defineProps<QRCodeProps>();
  
  // 生成二维码的方法
  const generateQRCode = (text: string) => {
    if (qrCanvas.value) {
      QRCode.toCanvas(qrCanvas.value, text, (error: any) => {
        if (error) console.error(error);
      });
    }
  };
  
  // 组件挂载后生成二维码
  onMounted(() => {
    const defaultText = 'https://example.com';
    generateQRCode(defaultText);
  });
  
  // 监听 props.value 变化并重新生成二维码
  watch(() => props.value, (newVal) => {
    generateQRCode(newVal);
  });
  </script>
  
  <style scoped>
  .qrcode-container {
    display: flex;
    justify-content: center;
    align-items: center;
  }
  
  canvas {
    margin: 10px;
  }
  </style>
  