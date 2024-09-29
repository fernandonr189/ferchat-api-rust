import requests
import json

headers = {
    "Content-Type": "application/json",
}


def print_response(response):
    print("Status code: " + str(response.status_code))
    try:
        print(f"Response: {response.json()['data']['Model'] if response.json()['data'] != None else response.json()['message']}")
    except Exception as e:
        print("Exception" + str(e))
        print(response.text)


def test_login() -> str | None:
    login_request = {
        "email": "fernandonr189@outlook.com",
        "password": "password"
    }
    data = json.dumps(login_request)
    response = requests.post("http://localhost:8000/login", headers=headers, data=data)
    print_response(response)
    if response.status_code != 200:
        return None
    return response.json()['data']['Model']['token']

def test_helloworld(token_str: str):
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {token_str}"
    }
    response = requests.get("http://localhost:8000/hello", headers=headers)
    print_response(response)

def test_signup():
    sigup_request = {
        "username": "ElFercho189",
        "password": "password",
        "email": "fernandonr189@outlook.com"
    }
    data = json.dumps(sigup_request)
    response = requests.post("http://localhost:8000/signup", headers=headers, data=data)
    print_response(response)


if __name__ == "__main__":
    test_signup()
    token = test_login()
    test_helloworld(token)
