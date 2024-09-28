import requests
import json

token_str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWJqZWN0X2lkIjoxLCJleHAiOjE3Mjc1NTM4ODV9.WGlzBP0oiiyher9GwkCcLMLBVq0JZeT9m0ZIBhuk1VQ"

headers = {
    "Authorization": f"Bearer {token_str}"
}


def print_response(response):
    print("Status code: " + str(response.status_code))
    try:
        print("Response" + str(response.json()))
    except Exception as e:
        print("Exception" + str(e))
        print(response.text)


def test_login():
    response = requests.get("http://localhost:8000/hello", headers=headers)
    print_response(response)

def test_get_users():
    response = requests.get("http://localhost:8000/users/get")
    print_response(response)

def get_token():
    response = requests.post("http://localhost:8000/login")
    print_response(response)

def test_insert_user():
    jsonUser = {
        "username": "Fercho1892",
        "password": "Password",
        "email": "email33",
        "id": 123,
        "is_active": True,
    }

    data = json.dumps(jsonUser)
    response = requests.post("http://localhost:8000/users/signup", data=data)
    print_response(response)


if __name__ == "__main__":
    get_token()
    test_login()
