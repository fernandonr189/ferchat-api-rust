import requests
import json
import sys
import asyncio
import websockets
import time

def print_colored(text, color, end='\n'):
    colors = {'red': '\x1b[31m', 'green': '\x1b[32m', 'yellow': '\x1b[33m', 'blue': '\x1b[34m'}
    reset = '\x1b[0m'
    sys.stdout.write(colors.get(color, '') + text + reset + end)

users = [
    {
        "username": "ElFercho189",
        "password": "password",
        "email": "fernandonr189@outlook.com"
    },
    {
        "username": "Finr32",
        "password": "password",
        "email": "finr32@outlook.com"
    }
]

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


def test_login(user: int) -> str | None:
    login_request = users[user]
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

def test_signup(user: int):
    sigup_request = users[user]
    data = json.dumps(sigup_request)
    response = requests.post("http://localhost:8000/signup", headers=headers, data=data)
    print_response(response)

def send_friend_request(token_str: str, target: int):
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {token_str}"
    }
    friend_request = {
        "friend_id": target
    }
    data = json.dumps(friend_request)
    response = requests.post("http://localhost:8000/friends/request", headers=headers, data=data)
    print_response(response)

def reject_friend_request(token_str: str, target: int):
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {token_str}"
    }
    friend_request = {
        "friend_id": target,
        "accept": False
    }
    data = json.dumps(friend_request)
    response = requests.post("http://localhost:8000/friends/accept", headers=headers, data=data)
    print_response(response)

def accept_friend_request(token_str: str, target: int):
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {token_str}"
    }
    friend_request = {
        "friend_id": target,
        "accept": True
    }
    data = json.dumps(friend_request)
    response = requests.post("http://localhost:8000/friends/accept", headers=headers, data=data)
    print_response(response)

def get_friends(token_str: str):
    print_colored('Friends:', color='green')
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {token_str}"
    }
    response = requests.get("http://localhost:8000/friends/get/accepted", headers=headers)
    print_response(response)

def get_pending_requests(token_str: str):
    print_colored('Pending requests:', color='green')
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {token_str}"
    }
    response = requests.get("http://localhost:8000/friends/get/pending", headers=headers)
    print_response(response)

def get_sent_requests(token_str: str):
    print_colored('Sent requests:', color='green')
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {token_str}"
    }
    response = requests.get("http://localhost:8000/friends/get/sent", headers=headers)
    print_response(response)

def cancel_request(token_str: str, target: int):
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {token_str}"
    }
    cancel_request = {
        "friend_id": target,
    }
    data = json.dumps(cancel_request)
    response = requests.post("http://localhost:8000/friends/delete", data=data,headers=headers)
    print_response(response)

async def chat_client(token_str: str, target, source):
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {token_str}"
    }
    uri = "ws://localhost:8000/echo/" + str(target) + "/" + str(source) # WebSocket server URI

    # Connect to the WebSocket server
    async with websockets.connect(uri, extra_headers=headers) as websocket:
        print("Connected to the server.")

        # Send a message to the server
        working = True
        while working:
            message = input("Enter your message: ")
            if(message == "exit"):
                working = False
                # await websocket.close()
                print("Closing connection")
            else:
                await websocket.send(message)

                # Wait for a response from the server
                # response = await websocket.recv()
                # print(f"Server response: {response}")


if __name__ == "__main__":
    # test_signup(1)
    # token = test_login(1)
    asyncio.get_event_loop().run_until_complete(chat_client("token", 3, 2))
    # test_helloworld(token)
    # send_friend_request(token, 11)
    # cancel_request(token, 11)
    # accept_friend_request(token, 10)
    # reject_friend_request(token, 9)
    # get_friends(token)
    # get_pending_requests(token)
    # get_sent_requests(token)
