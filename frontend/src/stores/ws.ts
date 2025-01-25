import WebSocket from '@tauri-apps/plugin-websocket';
import { defineStore } from "pinia";
import { reactive, watch } from "vue";

export interface SocketSendMessageInterface {
    type: string,
    content?: {
        data: any
    }
}


export interface SocketInterface {
    ws: null | WebSocket,
    message: string[],
    isConnected: boolean,
    heartBeatTimer: null | number,
}

const HEART_BEAT_INTERVAL = 10;
export const useWsStore = defineStore('websocket-store', () => {
    let state = reactive<SocketInterface>({
        ws: null,
        message: [],
        isConnected: false,
        heartBeatTimer: null,
    });

    const connect = async (token: string) => {
        try {
            state = {
                ws: await WebSocket.connect('ws://192.168.6.122:5678/ws', {
                    headers: {
                        'authorization': `Bearer ${token}`
                    }
                }),
                message: [],
                isConnected: true,
                heartBeatTimer: null,
            };

            // 初始化完成开始发送心跳.
            state.heartBeatTimer = setInterval(() => {
                send({ type: "Heartbeat" })
            }, HEART_BEAT_INTERVAL * 1000) as any as number;
        } catch {
            console.log('failed initializer to websocket server connect.');
        }
    }

    const disconnect = async () => {
        if (!state.ws || !state.isConnected) {
            console.log('failed send to websocket server disconnect.');
        }
        try {
            await state.ws?.disconnect()
        } catch {
            state.isConnected = false
        }
    }

    const send = async (message: SocketSendMessageInterface) => {
        if (!state.ws || !state.isConnected) {
            console.log('failed send to websocket server disconnect.');
        }
        try {
            await state.ws?.send(JSON.stringify(message))
        } catch {
            state.isConnected = false
        }
    }

    watch(state, (_newV) => {
        if (!state.isConnected && state.heartBeatTimer) {
            clearInterval(state.heartBeatTimer)
        }
        if (state.ws && state.isConnected) {
            state.ws.addListener((msg) => {
                console.log('msg: {}', msg);
            })
        }
    })

    return {
        connect, disconnect, send, isConnected: state.isConnected
    }
})