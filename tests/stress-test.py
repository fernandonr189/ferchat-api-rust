import requests
import json
import concurrent.futures
import threading
import string
import random
import time

headers = {
    "Content-Type": "application/json",
}

def generate_random_string(length: int) -> str:
    return ''.join(random.choices(string.ascii_uppercase + string.digits, k=length))

base_url = "http://localhost:8000"

def create_user() -> tuple[str, str]:
    password = generate_random_string(10)
    email = generate_random_string(10) + "@gmail.com"
    username = generate_random_string(10)
    sigup_request = {
        "username": username,
        "password": password,
        "email": email,
    }
    data = json.dumps(sigup_request)
    response = requests.post("http://localhost:8000/signup", headers=headers, data=data)
    if response.status_code != 200:
        raise Exception("User not created")
    else:
        return (email, password)

def login_user(user: tuple[str, str]) -> str:
    login_request = {
        "email": user[0],
        "password": user[1]
    }
    data = json.dumps(login_request)
    try:
        response = requests.post("http://localhost:8000/login", headers=headers, data=data)
        if response.status_code != 200:
            raise Exception("Could not login")
        else:
            return response.json()['data']['Model']['token']
    except Exception as e:
        print(e)

def hello_world(token: str):
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {token}"
    }
    response = requests.get(base_url + "/hello", headers=headers)
    if response.status_code != 200:
        raise Exception("Could not get hello world")

def test_instance():
    try:
        user = create_user()
        token = login_user(user)
        hello_world(token)
    except Exception as e:
        print(e)

def test_instance():
    user = create_user()
    token = login_user(user)
    hello_world(token)

if __name__ == "__main__":
    with concurrent.futures.ThreadPoolExecutor() as executor:
        runs = 100000
        for _ in range(runs):
            executor.submit(test_instance)
