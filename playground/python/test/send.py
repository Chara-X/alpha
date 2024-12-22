import requests
import sys
# from msg.new import *


def send():
    res = requests.get("https://www.google.com")
    print(res.status_code)


print(sys.path)
print(dir())
