import { defineStore } from "pinia";
import { useForm } from "vee-validate";


interface EnvironmentInfo {
    name?: string;
    description?: string;
    browser: string;
    ua: string;
    os: string;
    kind:string;
    country?: string;
    region?: string;
    city?: string;
    language_type: number;
    languages: string;
    gmt: string;
    geography: string;
    geo_tips: number;
    geo_rule: number;
    longitude?: string;
    latitude?: string;
    radius?: number;
    height?: number;
    width?: number;
    fonts_type: number;
    fonts?: string;
    font_fingerprint: number;
    web_rtc: number;
    web_rtc_local_ip?: string;
    canvas: number;
    webgl: number;
    hardware_acceleration: number;
    webgl_info: number;
    audio_context: number;
    speech_voices: number;
    media: number;
    cpu: number;
    memory: number;
    do_not_track: number;
    battery: number;
    port_scan: number;
}

export const useEnvironmentCreateFromStore = defineStore('environment-create-form-store', () => {

    const {
        defineField, handleSubmit, errors
    } = useForm<EnvironmentInfo>({
        validationSchema: {
            name: (name?: string) => {
                return name ? true : "请输入环境名称";
            },
            description: (description?: string) => {
                return description ? true : "请输入描述";
            },
            browser: (browser?: string) => {
                return browser ? true : "请选择浏览器";
            },
            os: (os?: string) => {
                return os ? true : "请输入操作系统";
            },
            ua: (ua?: string) => {
                return ua ? true : "请输入浏览器类型";
            },
            kind: (kind?: string) => {
                return kind ? true : "请输入代理类型";
            },
            // language_type: (language_type?: number) => {
            //     return language_type !== undefined ? true : "请选择语言类型";
            // },
            // languages: (languages?: string) => {
            //     return languages && languages.length > 0 ? true : "请输入支持的语言";
            // },
            // gmt: (gmt?: string) => {
            //     return gmt ? true : "请输入GMT时间";
            // },
            // geography: (geography?: string) => {
            //     return geography ? true : "请输入地理位置";
            // },
            // geo_tips: (geo_tips?: number) => {
            //     return geo_tips !== undefined ? true : "请输入Geo提示";
            // },
            // geo_rule: (geo_rule?: number) => {
            //     return geo_rule !== undefined ? true : "请输入Geo规则";
            // },
            // fonts_type: (fonts_type?: number) => {
            //     return fonts_type !== undefined ? true : "请输入字体类型";
            // },
            // font_fingerprint: (font_fingerprint?: number) => {
            //     return font_fingerprint !== undefined ? true : "请输入字体指纹";
            // },
            // web_rtc: (web_rtc?: number) => {
            //     return web_rtc !== undefined ? true : "请输入WebRTC值";
            // },
            // canvas: (canvas?: number) => {
            //     return canvas !== undefined ? true : "请输入Canvas值";
            // },
            // webgl: (webgl?: number) => {
            //     return webgl !== undefined ? true : "请输入WebGL值";
            // },
            // hardware_acceleration: (hardware_acceleration?: number) => {
            //     return hardware_acceleration !== undefined ? true : "请输入硬件加速值";
            // },
            // webgl_info: (webgl_info?: number) => {
            //     return webgl_info !== undefined ? true : "请输入WebGL信息";
            // },
            // audio_context: (audio_context?: number) => {
            //     return audio_context !== undefined ? true : "请输入音频上下文值";
            // },
            // speech_voices: (speech_voices?: number) => {
            //     return speech_voices !== undefined ? true : "请输入语音声音值";
            // },
            // media: (media?: number) => {
            //     return media !== undefined ? true : "请输入媒体值";
            // },
            // cpu: (cpu?: number) => {
            //     return cpu !== undefined ? true : "请输入CPU值";
            // },
            // memory: (memory?: number) => {
            //     return memory !== undefined ? true : "请输入内存值";
            // },
            // do_not_track: (do_not_track?: number) => {
            //     return do_not_track !== undefined ? true : "请输入Do Not Track值";
            // },
            // battery: (battery?: number) => {
            //     return battery !== undefined ? true : "请输入电池值";
            // },
            // port_scan: (port_scan?: number) => {
            //     return port_scan !== undefined ? true : "请输入端口扫描值";
            // }
        },
    });

    const [environmentName, environmentNameProps] = defineField('name');
    const [environmentDescription, environmentDescriptionProps] = defineField('description');
    const [ua,uaProps] = defineField('ua');
    const [browser, browserProps] = defineField('browser');
    const [os, osProps] = defineField('os');
    const [kind,kindProps] =defineField('kind');
    const [languageType, languageTypeProps] = defineField('language_type');
    const [languages, languagesProps] = defineField('languages');
    const [gmt, gmtProps] = defineField('gmt');
    const [geography, geographyProps] = defineField('geography');
    const [geoTips, geoTipsProps] = defineField('geo_tips');
    const [geoRule, geoRuleProps] = defineField('geo_rule');
    const [longitude, longitudeProps] = defineField('longitude');
    const [latitude, latitudeProps] = defineField('latitude');
    const [radius, radiusProps] = defineField('radius');
    const [height, heightProps] = defineField('height');
    const [width, widthProps] = defineField('width');
    const [fontsType, fontsTypeProps] = defineField('fonts_type');
    const [fonts, fontsProps] = defineField('fonts');
    const [fontFingerprint, fontFingerprintProps] = defineField('font_fingerprint');
    const [webRtc, webRtcProps] = defineField('web_rtc');
    const [canvas, canvasProps] = defineField('canvas');
    const [webgl, webglProps] = defineField('webgl');
    const [hardwareAcceleration, hardwareAccelerationProps] = defineField('hardware_acceleration');
    const [webglInfo, webglInfoProps] = defineField('webgl_info');
    const [audioContext, audioContextProps] = defineField('audio_context');
    const [speechVoices, speechVoicesProps] = defineField('speech_voices');
    const [media, mediaProps] = defineField('media');
    const [cpu, cpuProps] = defineField('cpu');
    const [memory, memoryProps] = defineField('memory');
    const [doNotTrack, doNotTrackProps] = defineField('do_not_track');
    const [battery, batteryProps] = defineField('battery');
    const [portScan, portScanProps] = defineField('port_scan');

    const forms = {
        environmentName,
        environmentNameProps,
        ua,
        uaProps,
        environmentDescription,
        environmentDescriptionProps,
        browser,
        browserProps,
        os,
        osProps,
        kind,
        kindProps,
        languageType,
        languageTypeProps,
        languages,
        languagesProps,
        gmt,
        gmtProps,
        geography,
        geographyProps,
        geoTips,
        geoTipsProps,
        geoRule,
        geoRuleProps,
        longitude,
        longitudeProps,
        latitude,
        latitudeProps,
        radius,
        radiusProps,
        height,
        heightProps,
        width,
        widthProps,
        fontsType,
        fontsTypeProps,
        fonts,
        fontsProps,
        fontFingerprint,
        fontFingerprintProps,
        webRtc,
        webRtcProps,
        canvas,
        canvasProps,
        webgl,
        webglProps,
        hardwareAcceleration,
        hardwareAccelerationProps,
        webglInfo,
        webglInfoProps,
        audioContext,
        audioContextProps,
        speechVoices,
        speechVoicesProps,
        media,
        mediaProps,
        cpu,
        cpuProps,
        memory,
        memoryProps,
        doNotTrack,
        doNotTrackProps,
        battery,
        batteryProps,
        portScan,
        portScanProps
    }

    return {
        defineField, handleSubmit, errors, forms
    }
})

