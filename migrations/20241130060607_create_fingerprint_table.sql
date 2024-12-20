CREATE TABLE
    fingerprints
(
    id                    INTEGER PRIMARY KEY AUTOINCREMENT,                           -- 自增ID
    owner_id              INTEGER,
    ua                    TEXT    NOT NULL,                                            -- 自定义UA
    language_type         INTEGER NOT NULL CHECK (language_type IN (0, 1, 2)),         -- 语言类型 0-跟随IP，1-自定义，2-跟随电脑
    languages             TEXT    NOT NULL,                                            -- 渲染语言
    gmt                   TEXT    NOT NULL,                                            -- 时区
    geography             TEXT    NOT NULL,                                            -- 地理
    geo_tips              INTEGER NOT NULL CHECK (geo_tips IN (0, 2)),                 -- 地理位置请求行为
    geo_rule              INTEGER NOT NULL CHECK (geo_rule IN (0, 1)),                 -- 地理位置规则
    longitude             TEXT,                                                        -- 自定义经度
    latitude              TEXT,                                                        -- 自定义纬度
    radius                INTEGER,                                                     -- 自定义半径
    height                INTEGER,                                                     -- 分辨率高
    width                 INTEGER,                                                     -- 分辨率宽
    fonts_type            INTEGER NOT NULL CHECK (fonts_type IN (0, 1)),               -- 字体列表保护 0-隐私，1-真实
    fonts                 TEXT,                                                        -- 字体列表
    font_fingerprint      INTEGER NOT NULL CHECK (font_fingerprint IN (0, 1)),         -- 字体指纹
    web_rtc               INTEGER NOT NULL CHECK (web_rtc BETWEEN 0 AND 4),            -- WebRTC配置
    web_rtc_local_ip      TEXT,                                                        -- 内网IP
    canvas                INTEGER NOT NULL CHECK (canvas IN (0, 1)),                   -- Canvas隐私保护
    webgl                 INTEGER NOT NULL CHECK (webgl IN (0, 1)),                    -- WebGL隐私保护
    hardware_acceleration INTEGER NOT NULL CHECK (hardware_acceleration IN (0, 1)),    -- 硬件加速
    webgl_info            INTEGER NOT NULL CHECK (webgl_info IN (0, 1)),               -- WebGL信息
    audio_context         INTEGER NOT NULL CHECK (audio_context IN (0, 1)),            -- AudioContext隐私保护
    speech_voices         INTEGER NOT NULL CHECK (speech_voices IN (0, 1)),            -- SpeechVoices
    media                 INTEGER NOT NULL CHECK (media IN (0, 1)),                    -- 媒体设备隐私保护
    cpu                   INTEGER NOT NULL CHECK (cpu IN (0, 2, 4, 6, 8, 10, 12, 16)), -- CPU
    memory                INTEGER NOT NULL CHECK (memory IN (0, 2, 4, 6, 8)),          -- 内存
    do_not_track          INTEGER NOT NULL CHECK (do_not_track IN (0, 1, 2)),          -- 追踪设置
    battery               INTEGER NOT NULL CHECK (battery IN (0, 1, 2)),               -- 电池隐私保护
    port_scan             INTEGER NOT NULL CHECK (port_scan IN (0, 1)),                -- 本地端口扫码
    white_list            TEXT,                                                        -- 端口白名单
    created_at            DATETIME DEFAULT CURRENT_TIMESTAMP,                          -- 创建时间
    updated_at            DATETIME DEFAULT CURRENT_TIMESTAMP,                          -- 更新时间
    deleted_at            DATETIME                                                     -- 删除时间
);


-- 插入一条指纹数据
INSERT INTO
    fingerprints (
        ua,
        language_type,
        languages,
        gmt,
        geography,
        geo_tips,
        geo_rule,
        longitude,
        latitude,
        radius,
        height,
        width,
        fonts_type,
        fonts,
        font_fingerprint,
        web_rtc,
        web_rtc_local_ip,
        canvas,
        webgl,
        hardware_acceleration,
        webgl_info,
        audio_context,
        speech_voices,
        media,
        cpu,
        memory,
        do_not_track,
        battery,
        port_scan,
        white_list
    )
VALUES
    (
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36', -- ua: 自定义UA字符串
        1, -- language_type: 自定义语言
        'en-US,en;q=0.9', -- languages: 渲染语言
        'UTC+0', -- gmt: 时区（UTC）
        'United States', -- geography: 地理位置
        0, -- geo_tips: 地理位置请求行为 (0)
        0, -- geo_rule: 地理位置规则 (0)
        '0.0', -- longitude: 经度（假设为默认值0.0）
        '0.0', -- latitude: 纬度（假设为默认值0.0）
        100, -- radius: 半径（例如100米）
        600, -- height: 分辨率高度（600px）
        600, -- width: 分辨率宽度（600px）
        1, -- fonts_type: 字体保护（1表示真实字体）
        'Arial, Times New Roman', -- fonts: 字体列表
        1, -- font_fingerprint: 字体指纹（1表示使用真实字体）
        2, -- web_rtc: WebRTC配置（2表示启用）
        '192.168.1.1', -- web_rtc_local_ip: 内网IP（假设为192.168.1.1）
        0, -- canvas: Canvas隐私保护（0表示无保护）
        1, -- webgl: WebGL隐私保护（1表示启用WebGL）
        1, -- hardware_acceleration: 硬件加速（1表示启用）
        1, -- webgl_info: WebGL信息（1表示提供WebGL信息）
        1, -- audio_context: AudioContext隐私保护（1表示启用）
        1, -- speech_voices: SpeechVoices（1表示启用）
        1, -- media: 媒体设备隐私保护（1表示启用）
        8, -- cpu: CPU核心数（假设为8核）
        8, -- memory: 内存大小（假设为8GB）
        1, -- do_not_track: 追踪设置（1表示启用）
        2, -- battery: 电池隐私保护（2表示启用）
        0, -- port_scan: 本地端口扫码（0表示禁用）
        '80, 443' -- white_list: 端口白名单（假设为80和443端口）
    );