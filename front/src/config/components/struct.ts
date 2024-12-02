

// 基础数据表
interface BaseInfo {
  id?:number; //获取id
  browserName?: string; //获取浏览器名称    
  groupName?: string; //获取分组名称
  priority?:string ; //获取便签信息
  goBrowser?:string ; //获取谷歌浏览器版本
  fiBrowser?: string; //获取火狐浏览器版本
  os?:string ; //获取操作系统信息
  userAgent?:string; //获取用户代理信息(User Agent)
  cookie?:string ; //获取cookie信息
  //如果合并为string类型
  combineCookie?:boolean; //是否合并cookie
  remark?: string; //获取备注信息
}

// 代理数据
interface ProxyInfo {
  id:number; //获取id
  //proxyEnabled暂且未知或全部类型
  proxyEnabled: boolean; //是否启用添加代理
  proxyMode?: string; //获取代理模式
  primary?: number;//检查网络（占位）
  ipLocation?: string; //ip渠道查询
}

//平台数据
interface PlatformInfo {
  id:number; //获取id
  platform?: string; //获取平台信息
  accountUrl?: string; //获取账号地址
}

//指纹设置
interface FingeInfo {
  id:number; //获取id
  //timeZone存入数据库为string类型
  timeZone?: boolean; //获取时区关于ip匹配
  webRTC?: string; //获取webRTC信息
  //geLocation存入数据库为string类型
  geLocation?: boolean; //获取地理位置询问或禁止
  //geIp存入数据库为string类型
  geIp?: boolean; //地理位置是否使用ip匹配
  //langIp存入数据库为string类型
  langIp?: boolean; //获取语言是否使用ip匹配
  //res存入数据库为number类型
  res?: boolean; //是否使用真实分辨率或者自定义分辨率
  //font存入数据库为string类型
  font?: boolean; //是否使用真实字体或者自定义字体
  //canvas暂且未知或全部类型
  canvas?: boolean; //是否使用真实canvas或者自定义canvas
  //webGLimage存入数据库为string类型
  webGLimage?: boolean; //是否使用真实webGL图像或者自定义webGL图像
  //webGLmetadata存入数据库为string类型
  webGLmetadata?: boolean; //是否使用真实webGL元数据或者自定义webGL元数据
  //audioText存入数据库为string类型
  audioText?: boolean; //是否使用真实音频文本或者噪音音频文本
  //clientRects存入数据库为string类型
  clientRects?: boolean; //是否使用真实ClientRects或者噪音ClientRects
  //speech存入数据库为string类型
  speech?: boolean; //是否使用真实语音或者噪音语音
  track?: string; //是否开启追踪（默认1）
  hardNum?: number; //获取硬件并发数
  equiNum?: number; //获取设备内存大小
  //equiName存入数据库为string类型
  equiName?: boolean; //是否使用自定义设备名称
  equiNames?: string; //获取设备名称
  //macAddress存入数据库为string类型
  macAddress?: boolean; //是否使用自定义mac地址
  macNames?: string; //获取mac地址
  //portPro存入数据库为number类型
  portPro?: boolean; //是否开启端口保护
  //macPro存入数据库为string类型
  macPro?: boolean; //是否开启mac地址保护
  //hardAcc存入数据库为string类型
  hardAcc?: boolean; //是否使用真实硬件加速（默认1）
  startup?: string; //填写启动参数
}

export type{
  BaseInfo,
  ProxyInfo,
  PlatformInfo,
  FingeInfo
}