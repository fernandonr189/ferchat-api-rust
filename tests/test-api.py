import requests, json

def testUser():
    jsonUser = {
            "username": "Fernando",
            "password": "Password",
            "email": "email",
            "id": 123,
            "is_active": True
        }
    data = json.dumps(jsonUser)
    response = requests.post("http://localhost:8000/todo", data=data).json()
    print(response)

if __name__ == "__main__":
    testUser()
