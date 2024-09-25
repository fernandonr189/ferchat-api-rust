import requests
import json


def print_response(response):
    print("Status code: " + str(response.status_code))
    try:
        print("Response" + str(response.json()))
    except Exception as e:
        print("Exception" + str(e))
        print(response.text)

def test_get_users():
    response = requests.get("http://localhost:8000/users/get")
    print_response(response)

def test_insert_user():
    jsonUser = {
        "username": "puchatron",
        "password": "Password",
        "email": "email2",
        "id": 123,
        "is_active": True,
    }

    data = json.dumps(jsonUser)
    response = requests.post("http://localhost:8000/users/signup", data=data)
    print_response(response)


if __name__ == "__main__":
    #test_insert_user()
    test_get_users()
