import requests
import json


def print_response(response):
    print("Status code: " + str(response.status_code))
    try:
        print("Response" + str(response.json()))
    except Exception as e:
        print("Exception" + str(e))
        print(response.text)


def testInsertUser():
    jsonUser = {
        "username": "Fernando",
        "password": "Password",
        "email": "email",
        "id": 123,
        "is_active": True,
    }

    data = json.dumps(jsonUser)
    response = requests.post("http://localhost:8000/users/signup", data=data)
    print_response(response)


if __name__ == "__main__":
    testInsertUser()
