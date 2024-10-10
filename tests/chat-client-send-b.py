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
async def send_to_socket(send_uri, user):
    async with websockets.connect(send_uri, extra_headers=get_auth(user)) as send_socket:
        while True:
            try:
                user_input = input("Enter a message to send: ")
                await send_socket.send(user_input)
            except websockets.ConnectionClosed:
                print("Send connection closed.")
                break

async def main(send_uri, user):
    await asyncio.gather(
        send_to_socket(send_uri, user)
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
    send_uri = "ws://localhost:8000/yell/10"  # Replace with your sending WebSocket URI
    asyncio.run(main(send_uri, 1))
