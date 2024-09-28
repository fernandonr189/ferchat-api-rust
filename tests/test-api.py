import requests
import json

token_str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWJqZWN0X2lkIjoxLCJleHAiOjE3Mjc1NTU1NTZ9.G_3J_fEete8Om1Atiz9Ztc5yektSGKch_1twf8L6jYo"

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


def test_login():
    login_request = {
        "email": "fernandonr189@outlook.com",
        "password": "password"
    }
    data = json.dumps(login_request)
    response = requests.post("http://localhost:8000/login", headers=headers, data=data)
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
    test_login()
