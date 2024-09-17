import requests

if __name__ == "__main__":
    # test index
    response = requests.get("http://localhost:8000/").content
    print(f'Index: {response}')
