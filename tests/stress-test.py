import requests
import json
import concurrent.futures
import threading
import string
import random
import time

lock = threading.Lock()

signup_errors = 0;
login_errors = 0;
hello_world_errors = 0;

signup_attempts = 0;
login_attempts = 0;
hello_world_attempts = 0;

def increment_signup_errors():
    global signup_errors
    with lock:
        signup_errors += 1

def increment_login_errors():
    global login_errors
    with lock:
        login_errors += 1

def increment_hello_world_errors():
    global hello_world_errors
    with lock:
        hello_world_errors += 1

def increment_signup_attempts():
    global signup_attempts
    with lock:
        signup_attempts += 1

def increment_login_attempts():
    global login_attempts
    with lock:
        login_attempts += 1

def increment_hello_world_attempts():
    global hello_world_attempts
    with lock:
        hello_world_attempts += 1


headers = {
    "Content-Type": "application/json",
}

def generate_random_string(length: int) -> str:
    return ''.join(random.choices(string.ascii_uppercase + string.digits, k=length))

base_url = "http://localhost:8000"

def create_user() -> tuple[str, str]:
    increment_signup_attempts()
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
        increment_signup_errors()
        raise Exception("User not created")
    else:
        return email, password

def login_user(user: tuple[str, str]) -> str:
    increment_login_attempts()
    login_request = {
        "email": user[0],
        "password": user[1]
    }
    data = json.dumps(login_request)
    try:
        response = requests.post("http://localhost:8000/login", headers=headers, data=data)
        if response.status_code != 200:
            increment_login_errors()
            raise Exception("Could not login")
        else:
            return response.json()['data']['Model']['token']
    except Exception as e:
        print(e)

def hello_world(token: str):
    increment_hello_world_attempts()
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {token}"
    }
    response = requests.get(base_url + "/hello", headers=headers)
    if response.status_code != 200:
        increment_hello_world_errors()
        raise Exception("Could not get hello world")

def test_instance():
    try:
        user = create_user()
        token = login_user(user)
        hello_world(token)
    except Exception as e:
        print(e)

if __name__ == "__main__":
    start = time.time()
    with concurrent.futures.ThreadPoolExecutor() as executor:
        runs = 10000
        for _ in range(runs):
            executor.submit(test_instance)
    end = time.time()
    print(f"Time taken: {end - start}")
    print(f"Signup attempts: {signup_attempts}")
    print(f"Login attempts: {login_attempts}")
    print(f"Hello world attempts: {hello_world_attempts}")
    print(f"Signup errors: {signup_errors}")
    print(f"Login errors: {login_errors}")
    print(f"Hello world errors: {hello_world_errors}")
