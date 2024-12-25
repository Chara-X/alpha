# from testcase import HealthCheckCompleted 
import requests

# HealthCheckCompleted.health_check_completed()
status = requests.get("https://www.google.com")

print(status.status_code)
