import requests

if __name__ == "__main__":
    # test index
    response = requests.get("http://localhost:8000/").content
    print(f'Index: {response}')

    # test /world
    response = requests.get("http://localhost:8000/world").content
    print(f'World: {response}')

    # test /hello

    response = requests.get("http://localhost:8000/hello/Fernando").content
    print(f'Hello: {response}')
