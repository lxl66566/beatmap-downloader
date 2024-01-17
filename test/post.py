from requests import post

j = {"cmd": "beatmaplist", "type": "hot", "limit": 5}
print(j)
# res = post("https://api.sayobot.cn/?post", json=j)
res = post("http://127.0.0.1:5000", json=j)
