import os
import random
import json
import base64
import re
import socket
import subprocess
import time
import uuid
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime

import requests
from DrissionPage._configs.chromium_options import ChromiumOptions
from DrissionPage._pages.chromium_page import ChromiumPage
from faker import Faker


# 随机数
def random_audio(num):
    return random.randint(0, num - 1)


# 随机小数
def generate_random_float():
    return round(random.uniform(0, 0.5), 2)  # 保留两位小数


# 随机字符
def random_hex_char():
    hex_chars = 'abcdef'
    return random.choice(hex_chars)


# 随机小数
def generate_random_in_range():
    return random.uniform(1.0000001, 1.0000003)


def shuffle(array):
    random.shuffle(array)
    return array


def generate_unique_randoms(min_val, max_val, count):
    if count > (max_val - min_val + 1):
        raise ValueError(f"Cannot generate {count} unique numbers in the range [{min_val}, {max_val}]")

    values = list(range(min_val, max_val + 1))
    shuffled = shuffle(values)
    return shuffled[:count]


def check_(proxy):
    # url = "https://ipapi.co/json"
    url = "https://link.osnb.net/cityjson.php?ip="
    payload = {}
    headers = {
        'priority': 'u=0, i',
        # 'sec-fetch-user': '?1',
        # 'upgrade-insecure-requests': '1',
        'User-Agent': Faker().chrome()
    }
    p_password = proxy.split(":")[2] + ":" + proxy.split(":")[3]
    p_host = proxy.split(":")[0] + ":" + proxy.split(":")[1]
    proxy_ = {
        "http": f"http://{p_password}@{p_host}",
        "https": f"http://{p_password}@{p_host}"
    }
    try:
        # response = requests.request("GET", url, headers=headers, data=payload, proxies=proxy_, timeout=50)
        response = requests.request("GET", url, headers=headers,  proxies=proxy_, timeout=50)
        print(response.text)
        json_match = re.search(r'var returnCitySN\s*=\s*(\{.*\});', response.text, re.DOTALL)
        if json_match:
            json_data = json_match.group(1)
            # 解析 JSON 数据
            data = json.loads(json_data)
            return {
                "timezone": data['timezone'],
                # "languages": get_language_code_from_timezone(data['timezone']),
            }
        else:
            print("无法提取 JSON 数据")
            return None
    except Exception as e:
        print(f"Failed to check proxy {proxy}: {e}")
        return None

def is_remote_debugging_available(port, retries=5, delay=1):
    debugger_address = f'http://127.0.0.1:{port}/json'
    for attempt in range(retries):
        try:
            response = requests.get(debugger_address, timeout=2)
            if response.status_code == 200:
                print(f"Remote debugging available at {debugger_address}")
                return True
        except requests.RequestException:
            print(f"Waiting for remote debugging on port {port} (Attempt {attempt + 1}/{retries})")
            time.sleep(delay)
    print(f"Remote debugging not available on port {port}")
    return False

def gen_fp(proxy, port):
    proxy_check = check_(proxy)
    if proxy_check is None:
        return None
    # 数据生成
    unique_three_values = generate_unique_randoms(0, 74, 3)
    string_three_values = ",".join(map(str, unique_three_values))

    unique_forty_eight_values = generate_unique_randoms(0, 118, 48)
    string_forty_eight_values = ",".join(map(str, unique_forty_eight_values))

    unique_font_eight_values = generate_unique_randoms(0, 3455, random.randint(190, 750))
    unique_font_eight_values.sort()
    string_font_eight_values = ",".join(map(str, unique_font_eight_values))

    data = {
        "gl_ven": "Intel",  # gpu 厂商
        "gl_rend": "Intel, Intel(R) UHD Graphics 630 Direct3D11 vs_5_0 ps_5_0, D3D11-24.20.100.6345",  # gpu 渲染
        "gl_rd": generate_random_float(),  # gpu 随机数 0-0.5 之间的浮点数
        "os_ver": "9.6.0",  # 系统版本
        "os_mem": 4,  # 系统内存
        "proc_num": 2,  # 系统cpu 逻辑处理器数量
        "audio": random_audio(49),  # 音频指纹  0-49 之间的整数
        # "phone": "HuaWei P30",  # 可选参数
        # "cv_n": generate_random_float(),  # canvas 随机 0-0.5 的浮点数
        # "cv_s": random_hex_char(),  # canvas 随机 a-f 的字符
        "p": "0",  # 端口扫描防护
        # "css": random_audio(19),  # css 字体  随机 0-19
        "h": 1024,  # 屏幕高度
        "w": 1280,  # 屏幕宽度
        "c_r": generate_random_in_range(),  # ClientRects 指纹
        "la": 37.8496,  # 纬度
        "lo": 112.5656,  # 经度
        # "tz": "America/New_York",  # 时区id
        "tz": proxy_check['timezone'],  # 时区id
        "lang": "zh-CN",  # 语言数组
        "v_l": string_forty_eight_values,
        # win: [stringThreeValues] 0-74 随机3个 mac: [stringFortyEightValues] 0-118 随机48个
        "f_l": string_font_eight_values,  # 字体列表 生成 0-3455 的随机190-750个不重复的值
        "tag": "我的浏览器",
        "pro_xy": "socks5://64.137.104.68:5678:uivmlvqk:cn1ptrb20v66"
    }

    json_str = json.dumps(data, ensure_ascii=False)

    bs64en_str = base64.b64encode(json_str.encode('utf-8')).decode('utf-8')

    # p_password = proxy.split(":")[2] + ":" + proxy.split(":")[3]
    # p_host = proxy.split(":")[0] + ":" + proxy.split(":")[1]
    # proxies = {"all://": f"socks5://{p_password}@{p_host}"}

    proxy_ = {
        "Proxy": {
            "type": "socks5",
            "user": proxy.split(":")[2],
            "pass": proxy.split(":")[3]
        },
    }
    host_and_port = proxy.split(":")[0] + ":" + proxy.split(":")[1]
    escaped_proxy = json.dumps(proxy_, separators=(",", ":")).replace('"', r'\"')
    # user_data_dir = os.path.join(os.getcwd(), f"data\\User_Data_1")
    user_data_dir = os.path.join(os.getcwd(), f"data\\User_Data_{int(time.time())}-{uuid.uuid4()}")
    run_args = rf'C:\Users\Administrator\AppData\Local\Chromium\Application\chrome.exe --remote-debugging-port={port} --user-data-dir={user_data_dir} --breeze-fp="{bs64en_str}" --cfbrowserpro="{escaped_proxy}" --proxy-server=socks5://{host_and_port}'

    print(run_args)
    # # 启动浏览器进程并继续执行后续操作
    process = subprocess.Popen(run_args, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

    # 继续执行后续操作（不会等待浏览器启动完成）
    print(f"Chrome launched on port {port} with process ID {process.pid}")
    time.sleep(2)
    debugger_address = f'127.0.0.1:{port}'
    while True:
        if is_remote_debugging_available(port):
            break
        time.sleep(1)
    co = ChromiumOptions().set_address(debugger_address)
    co.set_pref(arg='profile.managed_default_content_settings.images', value='2')
    page = ChromiumPage(addr_or_opts=co)
    page.get('https://x.com/home')


def run(ip, port):
    p = find_free_port()
    print(f"IP: {ip}, Port: {p}")
    gen_fp(ip, p)


def find_free_port():
    # 创建一个 socket 对象，指定协议为 IPv4 和 TCP
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        # 绑定到主机的任意可用端口，端口号为 0 表示让操作系统自动分配一个未占用的端口
        s.bind(('', 0))
        # 获取分配的端口号
        port = s.getsockname()[1]
    # 返回找到的未被占用的端口号
    return port

if __name__ == '__main__':
    # 为每个端口分配不同的 IP 地址和端口组合
    with open('1.txt', 'r', encoding="utf-8") as f:
        lines = [l.strip() for l in f.readlines()]
    address_list = []
    count = 15733
    for line in lines:
        address_list.append((line, count))
        count += 1

    # 创建线程池并传递每个 IP 地址和端口组合
    with ThreadPoolExecutor(max_workers=10) as executor:
        # 使用 executor.map 传递每个组合到 run 函数
        executor.map(lambda args: run(*args), address_list)

