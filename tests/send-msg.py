import requests


uri = "http://localhost:8000/msg/3/2"


def print_response(response):
    print("Status code: " + str(response.status_code))
    try:
        print(f"Response: {response.json()['data']['Model'] if response.json()['data'] is not None else response.json()['message']}")
    except Exception as e:
        print("Exception" + str(e))
        print(response.text)

response = requests.get(uri)
print_response(response)