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
async def read_from_socket(read_uri, user):
    async with websockets.connect(read_uri, extra_headers=get_auth(user)) as read_socket:
        while True:
            try:
                message = await read_socket.recv()
                print(f"Received: {message}")
            except websockets.ConnectionClosed:
                print("Read connection closed.")
                break

async def main(read_uri, user):
    await asyncio.gather(
        read_from_socket(read_uri, user),
    )

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

if __name__ == "__main__":
    read_uri = "ws://localhost:8000/hear/11"  # Replace with your reading WebSocket URI
    asyncio.run(main(read_uri, 0))
