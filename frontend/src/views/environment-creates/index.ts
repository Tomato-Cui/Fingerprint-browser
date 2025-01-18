export { default as EnvironmentSimpleCreate } from './views/simple-create.vue'
export { default as EnvironmentAdvancedCreate } from './views/advanced-create.vue'
export { default as EnvironmentBatchImport } from './views/batch-import.vue'
export { default as EnvironmentLayout } from './layout.vue'

import { useEnvironmentAdvancedFormStore, templaeFormtData } from "@/stores/form/environment-advanced";
import { type EnvironmentDetailWithAdvanceCreateRequest, environment_advanced_create } from '@/commands/environment';

export const batchCreateHandle = async (name: string) => {
    const forms = useEnvironmentAdvancedFormStore().forms;

    let { numbers, encryPtSwitch }: { numbers: number, encryPtSwitch: any } = forms;
    encryPtSwitch = encryPtSwitch.value == '关闭' ? false : true;

    let payload: EnvironmentDetailWithAdvanceCreateRequest = {
        name: name,
        description: forms.description || templaeFormtData.description.default,
        group_id: forms.group || undefined,
        tag_id: forms.tag || undefined,
        browser: forms.browser.value,
        ua: forms.ua,
        os: forms.os.value || 'windows',
        language: getRandomLanguage() || navigator.language,
        languages: navigator.languages.join(','),
        timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
        height: window.screen.height,
        width: window.screen.width,
        fonts: Array.from(document.fonts).map(font => font.family).join(','),
        web_rtc: forms.fingerprint.webRTC.value != 'disabled',
        web_rtc_local_ip: await getLocalIP(),
        canvas: getCanvasFingerprint(),
        webgl: true,
        hardware_acceleration: true,
        webgl_info: getWebGLInfo(),
        audio_context: !!window.AudioContext,
        speech_voices: !!window.speechSynthesis,
        media: !!navigator.mediaDevices,
        cpu: parseInt(forms.fingerprint.hardwareConcurrency.value),
        memory: parseInt(forms.fingerprint.deviceMemory.value),
        do_not_track: forms.fingerprint.doNotTrack.value,
        battery: forms.fingerprint.battery.value == 'real',
        port_scan: forms.fingerprint.portScanning.value,
    };

    return await environment_advanced_create(numbers, encryPtSwitch, payload)

}


export const advancedCreateHandle = async () => {
    const forms = useEnvironmentAdvancedFormStore().forms;

    let { numbers, encryPtSwitch }: { numbers: number, encryPtSwitch: any } = forms;
    encryPtSwitch = encryPtSwitch.value == '关闭' ? false : true;

    let payload: EnvironmentDetailWithAdvanceCreateRequest = {
        name: forms.name || templaeFormtData.name.default,
        description: forms.description || templaeFormtData.description.default,
        group_id: forms.group || undefined,
        tag_id: forms.tag || undefined,
        browser: forms.browser.value,
        ua: forms.ua,
        os: forms.os.value || 'windows',
        language: getRandomLanguage() || navigator.language,
        languages: navigator.languages.join(','),
        timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
        height: 888,// || window.screen.height,
        width: 888,// || window.screen.width,
        fonts: Array.from(document.fonts).map(font => font.family).join(','),
        web_rtc: forms.fingerprint.webRTC.value != 'disabled',
        web_rtc_local_ip: await getLocalIP(),
        canvas: getCanvasFingerprint(),
        webgl: true,
        hardware_acceleration: true,
        webgl_info: getWebGLInfo(),
        audio_context: !!window.AudioContext,
        speech_voices: !!window.speechSynthesis,
        media: !!navigator.mediaDevices,
        cpu: parseInt(forms.fingerprint.hardwareConcurrency.value),
        memory: parseInt(forms.fingerprint.deviceMemory.value),
        do_not_track: forms.fingerprint.doNotTrack.value,
        battery: forms.fingerprint.battery.value == 'real',
        port_scan: forms.fingerprint.portScanning.value,
    };

    return await environment_advanced_create(numbers, encryPtSwitch, payload)

}

const getCanvasFingerprint = () => {
    const canvas = document.createElement('canvas');
    const ctx = canvas.getContext('2d') as any;
    let random = Math.random();
    ctx.textBaseline = 'top';
    ctx.font = '14px Arial';
    ctx.fillStyle = '#f60';
    ctx.fillRect(125, 1, 62, 20);
    ctx.fillStyle = '#069';
    ctx.fillText('lius' + random, 2, 15);
    return canvas.toDataURL();
};

const getLocalIP = async (): Promise<string | undefined> => {
    const peerConnection = new RTCPeerConnection({ iceServers: [] });
    peerConnection.createDataChannel('');
    const offer = await peerConnection.createOffer();
    await peerConnection.setLocalDescription(offer);

    return new Promise((resolve) => {
        peerConnection.onicecandidate = (event) => {
            if (event.candidate) {
                const ip = event.candidate.candidate.split(' ')[4];
                resolve(ip);
            } else {
                resolve(undefined);
            }
        };
    });
};

const getWebGLInfo = (): string => {
    const canvas = document.createElement('canvas');
    const gl: any = canvas.getContext('webgl') || canvas.getContext('experimental-webgl');
    if (!gl) return "";

    const debugInfo = gl.getExtension('WEBGL_debug_renderer_info');
    return JSON.stringify({
        vendor: gl.getParameter(debugInfo.UNMASKED_VENDOR_WEBGL),
        renderer: gl.getParameter(debugInfo.UNMASKED_RENDERER_WEBGL),
    });
};

const getRandomLanguage = () => {
    const languageList = [
        "en", "zh", "es", "fr", "de", "ja", "ko", "ru", "ar", "pt", "it", "hi", "nl", "tr", "vi", "th", "pl", "sv", "fi", "el",
    ];
    const randomIndex = Math.floor(Math.random() * languageList.length);
    return languageList[randomIndex];
};
