import json

import asyncio
import websockets
import requests

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
def get_auth(user):
    headers = {
        "Content-Type": "application/json",
    }
    login_request = users[user]
    data = json.dumps(login_request)
    response = requests.post("http://localhost:8000/login", headers=headers, data=data)
    headers = {
        "Content-Type": "application/json",
        "Authorization": f"Bearer {response.json()['data']['Model']['token']}"
    }
    return headers
async def test_client(user):
    async with websockets.connect("ws://localhost:8000/session", extra_headers=get_auth(user) ) as websocket:
        while True:
            response = await websocket.recv()
            # await websocket.send(f"Pong: {response}")
            print(f"Received from server: {response}")

if __name__ == "__main__":
    asyncio.run(test_client(1))
