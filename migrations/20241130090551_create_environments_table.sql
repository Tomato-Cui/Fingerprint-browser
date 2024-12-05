-- -- Environments Table
CREATE TABLE
    environments (
        id INTEGER PRIMARY KEY AUTOINCREMENT, -- 自增ID
        owner_id TEXT NOT NULL,
        name TEXT NOT NULL UNIQUE, -- 环境名称
        description TEXT, -- 环境名称
        domain_name TEXT NOT NULL, -- 域名
        open_urls TEXT NOT NULL, -- 其他URL
        repeat_config TEXT NOT NULL, -- 去重配置
        username TEXT NOT NULL, -- 账号
        password TEXT NOT NULL, -- 密码
        fakey TEXT NOT NULL, -- 2FA密钥
        cookie TEXT NOT NULL, -- Cookie
        ignore_cookie_error INTEGER NOT NULL DEFAULT 0, -- 校验Cookie失败时的行为
        group_id INTEGER NOT NULL DEFAULT 0, -- 分组ID
        fp_info_id INTEGER NOT NULL DEFAULT 0, -- 指纹信息ID
        ua TEXT NOT NULL, -- 用户代理
        os TEXT NOT NULL, -- 操作系统
        country TEXT, -- 国家/地区
        region TEXT, -- 省/州
        city TEXT, -- 城市
        remark TEXT, -- 备注
        ipchecker TEXT NOT NULL DEFAULT 'ip2location', -- IP查询渠道
        sys_app_cate_id TEXT NOT NULL DEFAULT '0', -- 应用分类ID
        user_proxy_config TEXT, -- 环境代理配置
        proxy TEXT, -- 代理IP
        proxy_enable INTEGER NOT NULL DEFAULT 0, -- 代理启用（0为禁用，1为启用）
        is_tz INTEGER NOT NULL DEFAULT 0, -- 是否启用时区
        is_pos INTEGER NOT NULL DEFAULT 0, -- 是否启用地理位置
        user_data_file TEXT NOT NULL, -- 用户数据文件路径
        driver_location TEXT, -- 浏览器驱动位置
        status INTEGER NOT NULL DEFAULT 1, -- 浏览器状态（1为启用，0为禁用）
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 创建时间
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 更新时间
        lasted_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 创建时间
        deleted_at DATETIME -- 删除时间
        -- FOREIGN KEY (group_id) REFERENCES groups (id) -- 外键关联group表
        -- FOREIGN KEY (fp_info_id) REFERENCES fingerprints (id) -- 外键关联group表
    );