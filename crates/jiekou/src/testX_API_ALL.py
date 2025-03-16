
import http.client
import json
import datetime
import time

"""五代eslworkingAPI接口设计_V1.3.docx"""


req_url = "172.17.120.25:9010"
_headers = {'Content-Type': 'application/json'}
ESL_LIST = ['61-8B-37-88'] # , '50-7E-46-53'
UC = 'shi.002'
MAC = "98:6D:35:79:C5:A2"
BAK_URL = "http://10.11.163.211:8080/shopweb-webapp/ogi/ew/httpHandler"


def update():
    ##########################
    #        PUT             #
    # /api3/default/esls     #
    #        4.1             #
    ##########################
    _headers = {'Content-Type': 'application/json'}
    # req_url = EW_IP
    esl_list = ESL_LIST
    params = list()
    for e in esl_list:
        d = \
            {
                "esl_id": e.strip(),
                "sid": "19940510",
                "priority": "1",
                "template": "sct",
                "back_url": "http://10.11.173.32:9091/shopweb-webapp/ogi/ew/httpHandler",
                "ap_mac": "98:6D:35:76:6D:B8",
                "force_update": "false",
                "screen": {
                    "name": "",
                    "default_page": "",
                    "default_page_id": "",
                    "pages": [
                        {
                            "id": 0,
                            "name": "normal",
                            "image": "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"
                        },
                        {
                            "id": 1,
                            "name": "normal1",
                            "image": "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"
                        }

                    ],
                    "args": {
                        "price": 1
                    },
                    "images": {
                        # 模板中用到的图片内容
                    },

                    "control": {
                        "switch_page": {
                            "page_id": "",  # 页号
                            "stay_time": "",  # 停留时间
                            "switch_page_mode": "",  # 0=保持 1，标准 2 永久
                            "refresh_type": "",  # 0=标准，1快刷
                        }
                    },
                    "flash_light": {
                        "colors": [],               # 最多可设置7个值
                        "on_time": "",              # 0~255
                        "off_time": "",             # 0~255
                        "sleep_time": "",           # 0~65535
                        "loop_count": "",           # 0~65535
                        "task_id": ""               # 0~255 仅五代价签
                    },
                    "store_config": {  # 干簧管策略
                        "magnet_actions": ["send_heartbeats", ""],
                        "switch_page": {  # magnet_action为switch_page时的参数设定
                            "page_id": 1,  # 翻页页号
                            "stay_time": 30,  # 翻页停留时间0~65535
                            "stay_type": 0,  # 切页类型 0标准 1保持 2永久，仅五代价签
                            "refresh_type": 1  # 0标准 1快刷，仅五代价签支持
                        },
                        "flash_light": {  # magnet_action为flash_light时的参数设定
                            "colors": ["violet"],
                            "on_time": 2,  # 0~255 2
                            "off_time": 2,  # 0~255 2
                            "sleep_time": 2,  # 0~65535 2
                            "loop_count": 25,  # 0~65535 25
                            # "task_id": 0  # 0~255 仅五代价签
                        },
                        "auto_refresh": False,             # 定时刷屏开关
                        "gather_temperature": True,        # 是否开启温度采集
                        "set_wor": 1,                      # 听帧周期设置
                        "quick_refresh": True,             # 启用价签快刷功能
                        "auto_refresh_timestamp": 1000000, # 启用价签快刷功能
                        "auto_refresh_period": 1000000,    # 定时刷屏间隔
                        "error_temperature_check": False,  # 异常温度检测开关
                        "temperature_threshold": [20],     # 设置价签温度区间  ?
                        "page_switch_type": 1,      # 切页类型
                        "mobile_communication": {   # 价签移动场景开关
                            "on": 1,
                            "level1": 1,            # 第一门限值
                            "level2": 1,            # 第二门限值
                            "stable_times": 1,      # 判断稳定的次数
                            "mobile_times": 1,      # 判断移动的次数
                        },
                        "aoa_heartbeat": False,     # 价签定位
                        "light_combine": False,     # 闪灯合并开关
                        "hf_mode": {
                            "on": 1,
                            "set_receive_time": 100,
                            "sync_receive_time": 50

                        }
                    }
                }
            }

        params.append(d)
    data = {"data": params}
    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', '/api3/default/esls', json.dumps(data), _headers)
    res = conn.getresponse()
    print(f"update eight template finished :={datetime.datetime.now()}")


def control():
    ##########################
    #        PUT             #
    # /api3/default/esls/control     #
    #        4.2             #
    ##########################


    esl_list = ESL_LIST
    params = list()
    for e in esl_list:
        d = \
            {
                "esl_id": e.strip(),
                "sid": "19940510",
                "priority": "1",
                "back_url": "http://10.11.173.32:9091/shopweb-webapp/ogi/ew/httpHandler",
                 "switch_page": {
                            "page_id": "",      # 页号
                            "stay_time": "",    # 停留时间
                            "switch_page_mode": "",  # 0=保持 1，标准 2 永久
                            "refresh_type": "",  # 0=标准，1快刷
                },
                "flash_light": {
                    "colors": ["blue", "white"],           # 最多可设置7个值
                    "on_time": "",          # 0~255
                    "off_time": "",         # 0~255
                    "sleep_time": "",       # 0~65535
                    "loop_count": "",       # 0~65535
                    "task_id": ""           # 0~255 仅五代价签
                },
                "set_cmd": {
                    "global_cmd": cmd,
                    "set_args": args,
                    "time_stamp": 0
                }
            }

        params.append(d)
    data = {"data": params}
    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', '/api3/default/control', json.dumps(data), _headers)
    res = conn.getresponse()
    print(f"update eight template finished :={datetime.datetime.now()}")


def magnet():
    ##########################
    #        PUT             #
    # /api3/default/esls/magnet     #
    #        4.3             #
    ##########################


    esl_list = ESL_LIST
    params = list()
    # for e in esl_list:
    #     d =  \
    #         {
    #             "esl_id": e.strip(),
    #             "sid": "19940510",
    #             "back_url": "http://127.0.0.1:9091/shopweb-webapp/ogi/ew/httpHandler",
    #             "priority": "1",
    #             "force_update": "true",
    #
    #             "switch_page": {  # magnet_action为switch_page时的参数设定
    #                 "page_id": 1,  # 翻页页号
    #                 "stay_time": 10,  # 翻页停留时间0~65535
    #                 "stay_type": 0,  # 切页类型 0标准 1保持 2永久，仅五代价签
    #                 "refresh_type": 0   # 0标准 1快刷，仅五代价签支持
    #             },
    #             "quick_light_step": 1,
    #             "quick_refresh": True
    #             # ,
    #             # "flash_light": {  # magnet_action为flash_light时的参数设定
    #             #     "colors": ["red"],  # 仅第一个颜色值生效
    #             #     "on_time":2,  # 0~255
    #             #     "off_time":2,  # 0~255
    #             #     "sleep_time":2,  # 0~65535
    #             #     "loop_count":25,  # 0~65535
    #             #     "task_id":0  # 0~255 仅五代价签
    #             # },
    #             # "force_clean_times": "5" , # 清屏次数
    #             # "auto_refresh": "true" #24小时自动刷屏开关
    #         # }
    #             # "gather_temperature":,
    #             # "set_wor":,
    #             # "quick_refresh":,
    #             # "error_temperature_check":,
    #             # "temperature_threshold": [],
    #             # "page_switch_type":,
    #             # "mobile_communication":,
    #             # "aoa_heartbeat":,
    #             # "light_combine":,
    #             # "hf_mode":
    #
    #         }]
    #
    # }


def light():
    ##########################
    #        PUT             #
    # /api3/{uc}/esls/control/light     #
    #        4.4             #
    ##########################


    esl_list = ESL_LIST
    params = list()
    d =  \
            {
                "esl_type": "EPD",  # [epd, lcd]
                "mode": "0",        #
                "back_url": "http://127.0.0.1:9091/shopweb-webapp/ogi/ew/httpHandler",
                "times": "1",       # not support 5gen esl
                "netlinked": False,  # 只发已组网
                "duration": 16,   # 发送时长，默认价签听帧
                "timestamp": 1334243232,            # 时间戳毫秒
                "esl_version": 3,          # [3,5]

            }
    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', '/api3/default/control/light', json.dumps(d), _headers)
    res = conn.getresponse()
    print(f"finished :={datetime.datetime.now()}")


def page():
    ##########################
    #        PUT             #
    # /api3/{uc}/esls/control/page     #
    #        4.5             #
    ##########################


    esl_list = ESL_LIST
    params = list()
    d =  \
            {
                "esl_type": "EPD",  # [epd, lcd]
                "page_id": "0",             # 页号
                "back_url": "http://127.0.0.1:9091/shopweb-webapp/ogi/ew/httpHandler",
                "stay_time": "3",       # not support 5gen esl
                "times": "1",       # not support 5gen esl
                "netlinked": True,  # 只发已组网
                "duration": 16,   # 发送时长，默认价签听帧
                "timestamp": 1334243232,            # 时间戳毫秒
                "esl_version": 3,          # [3,5]

            }
    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', '/api3/default/control/page', json.dumps(d), _headers)
    res = conn.getresponse()
    print(f"finished :={datetime.datetime.now()}")


def firmwares():
    ##########################
    #        GET             #
    # /api3/esls/firmwares     #
    #        4.6             #
    ##########################

    esl_list = ESL_LIST
    params = list()

    conn = http.client.HTTPConnection(req_url)
    conn.request('GET', '/api3/esls/firmwares')
    res = conn.getresponse()
    print(res.read())
    print(f"finished :={datetime.datetime.now()}")


def get_firmwares(firmware_id=None):
    ##########################
    #        GET 指定id            #
    # /api3/esls/firmware/firmwareid     #
    #        4.7             #
    ##########################

    esl_list = ESL_LIST
    conn = http.client.HTTPConnection(req_url)
    conn.request('GET', f'/api3/default/esls/firmware/{firmware_id}')
    res = conn.getresponse()
    print(res.read())
    print(f"finished :={datetime.datetime.now()}")


def get_users():
    ##########################
    #        GET            #
    # /api3/users    #
    #        4.8             #
    ##########################

    esl_list = ESL_LIST
    conn = http.client.HTTPConnection(req_url)
    conn.request('GET', f'/api3/users', headers=_headers)
    res = conn.getresponse()
    print(res.read())
    print(f"finished :={datetime.datetime.now()}")


def get_user():
    ##########################
    #        GET           #
    # /api3/users    #
    #        4.9              #
    #        获取指定用户            #
    ##########################

    esl_list = ESL_LIST
    default= 'default'
    conn = http.client.HTTPConnection(req_url)
    conn.request('GET', f'/api3/{default}/user', headers=_headers)
    res = conn.getresponse()
    print(res.read())
    print(f"finished :={datetime.datetime.now()}")


def add_aps():
    ##########################
    #        PUT            #
    # /api3/{}/user/aps    #
    #        4.10             #
    #    为该用户分配多个基站             #
    ##########################
    usercode = 'default'
    _d = \
        {
            "aps":[
                {
                    "mac": "",                      # 基站的MAC
                    "allow_bind_v1esl": False,      # 是否可以绑定一代基站
                    "roaming_netlink": False,       # 漫游价签组网
                    "mobile_wor": 2,                # 移动听帧
                }
            ]

        }

    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', f'/api3/{usercode}/user/aps', json.dumps(_d), headers=_headers)
    res = conn.getresponse()
    print(f"finished :={datetime.datetime.now()}")


def del_mac(user=None, apMac=None):
    ##########################
    #        DELETE            #
    # /api3/{usercode}/user/ap/{apmac}    #
    #        4.11             #
    #    从default移除一个基站             #
    ##########################

    conn = http.client.HTTPConnection(req_url)
    conn.request('DElETE', f'/api3/{user}/user/ap/{apMac}',  headers=_headers)
    res = conn.getresponse()
    print(f"finished :={datetime.datetime.now()}")


def bind():
    ##########################
    #        DELETE            #
    # /api3/{usercode}/esls/bind    #
    #        4.12             #
    #    批量解绑价签             #
    ##########################


    esl_list = ESL_LIST
    params = list()
    for e in esl_list:
        d =  \
            {
                "esl_id": e.strip(),  # [epd, lcd]
                "sid": "234",        #
                "back_url": "http://127.0.0.1:9091/shopweb-webapp/ogi/ew/httpHandler",
                "template": "_UNBIND",       # not support 5gen esl
                "set_free": False,  #

            }
        params.append(d)
    data = {"data": params}

    conn = http.client.HTTPConnection(req_url)
    conn.request('DElETE', f'/api3/{UC}/esls/bind', headers=_headers)
    res = conn.getresponse()
    print(f"finished :={datetime.datetime.now()}")


def get_user_list(page=1):
    ##########################
    #        GET            #
    # /api3/{uc}/esls/page/{page}    #
    #        4.13             #
    #    获取指定us价签列表             #
    ##########################

    # req_url = EW_IP
    conn = http.client.HTTPConnection(req_url)
    conn.request('GET', f'/api3/{UC}/esls/page/{page}', headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def upload_mpd(name=None):
    ##########################
    #        PUT            #
    # /api3/{usercode}/templates/mpd/{name}    #
    #        4.15             #
    #    上传MPD点阵模板             #
    ##########################

    _d = \
        {
            "firmware": "2342",             # 选填firmwareId
            "screen": {
                "name": "",
                "default_page": "",
                "default_page_id": "",
                "pages": [
                    {
                        "id": 0,
                        "name": "normal",
                        "layout": "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"
                    },
                    {
                        "id": 1,
                        "name": "normal1",
                        "layout": "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"
                    }

                ],
                "args": {
                    "price": 1,
                    "name": "fuck",
                    "discount": "10"
                },
                "images": {
                    "0.jpg": "iVBORw0KGgoAAAANSUhEUgAAASgAAACYCAIAAADY2OOPAAAL8ElEQVR42u2di67rKAxF"
                },
            }

        }
    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', f'/api3/{UC}/templates/mpd/{name}', headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def display_mpd(name=None):
    ##########################
    #        PUT            #
    # /api3/{usercode}/templates/mpd/content/{name}    #
    #        4.16             #
    #    预览MPD点阵模板             #
    ##########################

    _d = \
        {
            "firmware": "2342",             # 选填firmwareId
            "screen": {
                "name": "",
                "default_page": "",         # 与pageid 互斥
                # "default_page_id": "",
                "pages": [
                    {
                        "id": 0,
                        "name": "normal",
                        "layout": "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"
                    },
                    {
                        "id": 1,
                        "name": "normal1",
                        "layout": "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"
                    }

                ],
                "args": {
                    "price": 1,
                    "name": "fuck",
                    "discount": "10"
                },
                "images": {
                    "0.jpg": "iVBORw0KGgoAAAANSUhEUgAAASgAAACYCAIAAADY2OOPAAAL8ElEQVR42u2di67rKAxF"
                },
            }

        }
    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', f'/api3/{UC}/templates/mpd/content/{name}', headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def get_wlt():
    ##########################
    #        GET             #
    # /api3/default/whitelist    #
    #        4.17             #
    ##########################

    esl_list = ESL_LIST
    conn = http.client.HTTPConnection(req_url)
    conn.request('GET', f'/api3/{UC}/whitelist', headers=_headers)
    res = conn.getresponse()
    print(res.read())
    print(f"finished :={datetime.datetime.now()}")


def add_wlt():
    ##########################
    #        POST             #
    # /api3/default/whitelist    #
    #        4.18             #
    ##########################
    # req_url = EW_IP
    esl_list = ESL_LIST
    conn = http.client.HTTPConnection(req_url)
    conn.request('POST', f'/api3/{UC}/whitelist', esl_list, headers=_headers)
    res = conn.getresponse()
    print(res.read())
    print(f"finished :={datetime.datetime.now()}")


def del_wlt():
    ##########################
    #        DELETE             #
    # /api3/default/whitelist    #
    #        4.19             #
    ##########################

    esl_list = ESL_LIST
    conn = http.client.HTTPConnection(req_url)
    conn.request('DELETE', f'/api3/{UC}/whitelist', json=esl_list, headers=_headers)
    res = conn.getresponse()
    print(res.read())
    print(f"finished :={datetime.datetime.now()}")


def del_esl():
    ##########################
    #        DELETE             #
    # /api3/{uc}/esls     #
    #        4.20             #
    ##########################

    esl_list = ESL_LIST
    conn = http.client.HTTPConnection(req_url)
    conn.request('DELETE', f'/api3/{UC}/esls', json=esl_list, headers=_headers)
    res = conn.getresponse()
    print(res.read())
    print(f"finished :={datetime.datetime.now()}")


def one_esl_reNetlink(isforce=False):
    ##########################
    #        PUT             #
    # /api3/{uc}/netlink/{eslid}/bind?force={boolean}     #
    #        4.21            #
    #     指定价签重组网       #
    ##########################

    esl_id = ESL_LIST[0]
    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', f'/api3/{UC}/netlink/{esl_id}/bind?force={isforce}', headers=_headers)
    res = conn.getresponse()
    print(res.read())
    print(f"finished :={datetime.datetime.now()}")


def global_upgrade(isforce=False): # pause
    ##########################
    #        PUT             #
    # /api3/{uc}/esls/upgrade#
    #        4.22            #
    #     全局广播升级         #
    ##########################

    esl_id = ESL_LIST[0]
    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', f'/api3/{UC}/esls/upgrade', headers=_headers)
    res = conn.getresponse()
    print(res.read())
    print(f"finished :={datetime.datetime.now()}")


def query_esl_info():
    ##########################
    #        PUT             #
    # /api3/{uc}/esls/query/statistics#
    #        4.23            #
    #   查询价签内部统计使用数据 #
    ##########################

    esl_id = ESL_LIST[0]
    # print(esl_id)
    _d = {

            "data": [{
                "esl_id": esl_id,
                "sid": "8964123",
                "type": 51,
                "back_url": BAK_URL

            }]
        }

    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', f'/api3/{UC}/esls/query/statistics', json.dumps(_d), headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def resync(flush=True):
    ##########################
    #        PUT             #
    # /api3/{uc}/sync/resumen#
    #        4.24            #
    #   触发门店所有基站重新同步 #
    ##########################

    # esl_id = ESL_LIST[0]
    # print(esl_id)
    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', f'/api3/{UC}/sync/resume?flush={flush}', headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def get_one_mac(mac="98:6D:35:79:C5:87"):
    ##########################
    #        GET             #
    # /api3/{uc}/aps/{mac}   #
    #        4.25            #
    #      查询门店指定基站        #
    ##########################

    # esl_id = ESL_LIST[0]
    # print(esl_id)
    conn = http.client.HTTPConnection(req_url)
    conn.request('GET', f'/api3/{UC}/aps/{mac}', headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def get_all_mac(mac="98:6D:35:79:C5:87"):
    ##########################
    #        GET             #
    #   /api3/{uc}/aps   #
    #        4.26            #
    #   查询门店中所有基站        #
    ##########################

    # esl_id = ESL_LIST[0]
    # print(esl_id)
    conn = http.client.HTTPConnection(req_url)
    conn.request('GET', f'/api3/{UC}/aps', headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def get_ap_lost_time():
    #############################################
    #                    GET                    #<
    #   /api3/{USERCODE}/sync/{MAC}/losttimes   #
    #                   4.34                    #
    #          查询制定基站失步次数(最近1h)         #
    #############################################
    mac = "${JAVA}"
    conn = http.client.HTTPConnection(req_url)
    conn.request('GET', f'/api3/{UC}/sync/{mac}/losttimes', headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def config_mac():
    #################################
    #              PUT              #<
    #   /api3/{uc}/aps/management   #
    #             4.35              #
    #            配置指定基站         #
    #################################
    mac = "98:6D:35:76:6D:B8"
    parse = list()
    e = \
        {
            "apMac": mac,
            "type": 3,              # 1 reboot ap  2 reset ap  3, reset_network    4, upgrade
            # "data": "",
            "back_url": "http://10.11.173.32:9091/shopweb-webapp/ogi/ew/httpHandler"
        }
    parse.append(e)
    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', f'/api3/{UC}/aps/management', json.dumps(parse), headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def get_one_pid():
    #########################################
    #                 GET                   #<
    #   /api3/{uc}/esls/product/{product}   #
    #                4.36                   #
    #              获取对应pid               #
    #########################################

    pid = "30003000"
    conn = http.client.HTTPConnection(req_url)
    conn.request('GET', f'/api3/{UC}/esls/product/{pid}', headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def config_neighborStore():
    ########################################################
    #                         PUT                          #<
    #   /api3/{USERCODE}/user/neighbors/{netid}/{subnetid} #
    #                         4.37                         #
    #                      配置相邻门店                      #
    ########################################################
    netid = 147
    subnetid = 6

    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', f'/api3/{UC}/user/neighbors/{netid}/{subnetid}', headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def global_conf():
    #####################################
    #              PUT                  #>
    #   /api3/{USERCODE}/esls/global    #
    #             4.38                  #
    #           全局指令配置              #
    #####################################

    _d = \
        {
            "set_cmd": {
                "global_cmd": "set_cmd_global",
                "set_args": [1, 3],
                "timestamp": 0,
                "esl_version": 0,
                "times": 0,
                "duration": 0,
                "netlinked": 0,
                "esl_type": 0
            }

        }

    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', f'/api3/{UC}/esls/global', json.dumps(_d), headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def wash_screen_conf():
    #########################################
    #        PUT                            #>
    #   /api3/{USERCODE}/esls/configuration #
    #        4.39                           #
    #      配置价签洗屏策略                    #
    #########################################

    params = list()
    for e in ESL_LIST:
        d =  \
            {
                "esl_id": e.strip(),  # [epd, lcd]
                "type": "234",        # [0-255]
                "args": "",           # base64编码
                "sid": "19890526",
                "back_url": "http://127.0.0.1:9091/shopweb-webapp/ogi/ew/httpHandler"

            }
        params.append(d)
    data = {"data": params}
    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', f'/api3/{UC}/esls/configuration', json.dumps(data), headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def wash_screen():
    #####################################
    #        PUT                        #>
    #   /api3/{uc}/esls/control/clean   #
    #        4.40                       #
    #     全体价签洗屏                    #
    #####################################

    _d = \
        {
            "mode": "3",    # 1 黑白 2 黑白红 3 黄 129黑白当 130 黑白红当 131 黄当 0 黄洗瓶 其他当前页
            "esl_type": "EPD",  # [epd, lcd]
            "back_url": "http://127.0.0.1:9091/shopweb-webapp/ogi/ew/httpHandler",
            "times": "1",  # not support 5gen esl
            "netlinked": False,  # 只发已组网
            "duration": 2,  # 发送时长，默认价签听帧
            "esl_version": 5    # 5:5gen esl 3: 3gen esl

        }

    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', f'/api3/{UC}/esls/control/clean', json.dumps(_d), headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def reset_esl():
    ####################################
    #        PUT                       #<
    #   /api3/{uc}/esls/reset/{flag}   #
    #        4.41                      #
    #     价签解绑门店                   #
    ####################################
    flag = 1 # 1 解绑门店 2 回复出厂设置
    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', f'/api3/{UC}/esls/reset/{flag}', json=ESL_LIST, headers=_headers)     # 1 解绑解绑 2 回复出厂
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def config_store():
    ###################################
    #        PUT                      #<
    #   /api3/{uc}/users/userConfig   #
    #        4.42                     #
    #       门店配置                   #
    ###################################
    _d = \
        {
            "set_wor": "1",                         # 门店听帧周期
            "query_enable": False,                  # 更新后是否需要查询
            # "mobile_enable": False,                 # 是否移动通信
            "mobile_communication": False,                 # 是否移动通信
            "netlink": False                        # t 正常组网  f  不组网

        }
    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', f'/api3/{UC}/users/userConfig', json.dumps(_d), headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def get_storeconf():
    ###################################
    #        GET                      #<
    #   /api3/{uc}/users/userConfig   #
    #        4.43                     #
    #     查询门店配置                  #
    ###################################

    conn = http.client.HTTPConnection(req_url)
    conn.request('GET', f'/api3/{UC}/users/userConfig', headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def del_storeconf():
    ###################################
    #        DELETE                   #>
    #   /api3/{uc}/users/userConfig   #
    #        4.44                     #
    #     删除门店配置                  #
    ###################################

    conn = http.client.HTTPConnection(req_url)
    conn.request('DELETE', f'/api3/{UC}/users/userConfig', headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def modify_esl_sync_channel():
    ########################################################
    #        PUT                                           #>
    #   /api3/{USERCODE}/esls/{eslid}/sync/{channel}       #
    #        4.45                                          #
    #     指定价签修改sync信道                                #
    ########################################################

    eslid = ESL_LIST[1]
    dst_channel = 2
    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', f'/api3/{UC}/esls/{eslid}/sync/{dst_channel}', headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def import_pid():
    #############################
    #        PUT                #<
    #   /api3/{uc}/esls/product #
    #        4.46               #
    #     批量更新导入product信息  #
    #############################

    p = list()
    _d = \
        {
                "product_index": 30003000,
                "resolution_y": 152,
                "resolution_x": 296,
                "max_page_and_button": {
                    "button": 0,
                    "max_page_num": 8
                },
                "screen_count": {
                    "screen_count": 1
                },
                "zip_buf": 64,
                "screen_color": 4,
                "screen_size_bitmap": 5,
                "firmware_properties": {
                    "reed": 0,
                    "freezer": 0,
                    "led": 1,
                    "screen_direction": 0
                },
                "refresh_time": 90,
                "eeprom_size": 800,
                "color_table": {
                    "gray_scale_num": 0,
                    "valid_and_color_bits": {
                        "valid": 1,
                        "color_bits": 2
                    },
                    "color_num": 4,
                    "colors": [{
                        "r": 0,
                        "b": 0,
                        "g": 0
                    }, {
                        "r": 255,
                        "b": 255,
                        "g": 255
                    }, {
                        "r": 255,
                        "b": 0,
                        "g": 255
                    }, {
                        "r": 255,
                        "b": 0,
                        "g": 0
                    }]
                },
                "school": {
                    "screen_size_flag": 0,
                    "fid": 0,
                    "nfc_type": 2,
                    "screen_type": 1,
                    "protocol_and_osd_version": 12,
                    "product_series": 11,
                    "mcu_type": 3,
                    "external_flash": 0
                },
                "flash_size": 464,
                "screen_size": 260,
                "dpi": 125,
                "hardware": {
                    "multi_values": 3204,
                    "ic_model": 24,
                    "battery_and_ext_flash": {
                        "external_flash_size": 0,
                        "battery_num": 2
                    },
                    "screen_vendor": 6,
                    "screen_status": 1
                },
                "misc": {
                    "self_discharge_rate": 30,
                    "battery_normal_threshold": 240,
                    "battery_freeze_threshold": 0,
                    "battery_total_capacity": 1000,
                    "temp_stage_refresh_currents": [1900, 1900, 1900, 1900, 1200, 1200, 1200],
                    "battery_cold_storage_threshold": 240,
                    "rf_rcv_avg_current": 7666,
                    "rf_snd_avg_current": 9098
                }

        }
    p.append(_d)
    data = {"data": p}
    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', f'/api3/{UC}/esls/product', json.dumps(data), headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def one_esl_setwor():
    ######################################
    #        PUT                         #>
    #   /api3/{uc}/esls/{eslid}/setwor   #
    #        4.47                        #
    #     指定价签id修改听帧                #
    ######################################

    _d = \
        {
            "time": 1,                      # law: 1,2,3,8,16,32
            "mode": 0,                      # 混合听帧模式 1:on,0:off, support 5gen esl
            "sid": "19890526",
            "priority": "1",
            "back_url": "http://127.0.0.1:9091/shopweb-webapp/ogi/ew/httpHandler",
        }

    eslid = ESL_LIST[1]
    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', f'/api3/{UC}/esls/{eslid}/setwor', json.dumps(_d), headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def update_storekey():
    ############################################
    #                  PUT                     #>
    #   /api3/{USERCODE}/users/userkey/{key}   #
    #                4.48                      #
    #              更新门店秘钥                  #
    ############################################
    key = "Nah"
    _d = \
        {
            "time": 1,                      # law: 1,2,3,8,16,32
            "mode": 0,                      # 混合听帧模式 1 on 0 off default off  only 5 gen
            "sid": "19949526",
            "priority": "1",
            "back_url": "http://127.0.0.1:9091/shopweb-webapp/ogi/ew/httpHandler",
        }

    eslid = ESL_LIST[1]
    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', f'/api3/{UC}/users/userkey/{key}', json.dumps(_d), headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


def get_sync_neighbors():
    ############################################
    #                  GET                     #<
    #   /api3/{USERCODE}/sync/neighbors        #
    #                4.49                      #
    #              获取相邻门店信息               #
    ############################################

    conn = http.client.HTTPConnection(req_url)
    conn.request('GET', f'/api3/{UC}/sync/neighbors', headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    print(f"finished :={datetime.datetime.now()}")


# def config_neighbor(pid="30003000"):
#     ##########################
#     #        PUT             #
#     #   /api3/{uc}/aps/neighbor   #
#     #        4.29            #
#     #     配置相邻门店         #
#     ##########################
#
#     _d = \
#         {
#             "apMac": "asd",
#             "type": 1
#         }
#
#     conn = http.client.HTTPConnection(req_url)
#     conn.request('PUT', f'/api3/{USERCODE}/aps/neighbor', json.dumps(_d), headers=_headers)
#     res = conn.getresponse()
#     print(res.read().decode('utf8'))
#     print(f"finished :={datetime.datetime.now()}")


def clear_history():
    #################################
    #        PUT                    #
    #   /api3/{uc}/esls/times/set   #
    #        4.13.1                   #
    #     清除价签计数                 #
    #################################

    params = list()
    for e in ESL_LIST:
        d =  \
            {
                "esl_id": e.strip(),
                "operation_type": 3,                # 1 设置清屏 2 设置nfc 3 设置led
                "priority": "1",                    #
                "refresh_times": "3",               #
                "nfc_times": "3",                   #
                "led_times": "3",                   #
                "sid": "12345678",
                "back_url": BAK_URL

            }
        params.append(d)
    _d = {'data': params}
    conn = http.client.HTTPConnection(req_url)
    conn.request('PUT', f'/api3/{UC}/esls/times/set', json.dumps(_d), headers=_headers)
    res = conn.getresponse()
    print(res.read().decode('utf8'))
    # print(f"finished :={datetime.datetime.now()}")


# clear_history()
# time.sleep(80)
query_esl_info()