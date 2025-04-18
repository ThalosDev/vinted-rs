import requests
import time

from lib import get_brands_names

class Color:
    def __init__(self, id, title, hex):
        self.id = id
        self.title = title
        self.hex = hex


def get_data(url: str, sess: requests.Session):
    response = sess.get(url)
    if response.status_code == 200:
        data = response.json()
        return data
    else:
        return None

def deserialize_json(data) -> list[Color]:
    colors = []
    for color in data["colors"]:
        d_color = Color(
            id=color["id"],
            title=color["title"],
            hex=color["hex"]
        )
        colors.append(d_color)

    return colors

def exec(outfilename):

    outfile= open(outfilename, "w")
    outfile.truncate()
    outfile.write("ID, COLOR, HEX\n")
    
    res=""

    URL = "https://www.vinted.es/api/v2/colors"

    URL_COOKIES = 'https://www.vinted.es/auth/token_refresh'

    sess = requests.Session()
    
    HEADERS = {
                "User-Agent": "PostmanRuntime/7.28.4",  # random.choice(USER_AGENTS),
                "Host": "www.vinted.es",
    }

    sess.headers.update(HEADERS)

    sess.post(URL_COOKIES)

    # Obtener los datos
    data = get_data(URL, sess)

    if data is not None:
        # Serializar el JSON
        deserialized_data = deserialize_json(data)
        # Imprimir los resultados deserializados
        if len(deserialized_data) > 0:
            for color in deserialized_data:
                res+=(f"{color.id} , \'{color.title}\' , \'#{color.hex}\'\n")
                print(color.title+ " ✔️")
    else:
        print("Error al obtener los datos.")

    outfile.write(res)
    outfile.close()