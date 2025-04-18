import requests
import time

from lib import get_brands_names

class Brand:
    def __init__(self, id, title, slug, favourite_count, pretty_favourite_count, item_count, pretty_item_count, is_visible_in_listings, path, requires_authenticity_check, is_luxury, url, is_favourite):
        self.id = id
        self.title = title
        self.slug = slug
        self.favourite_count = favourite_count
        self.pretty_favourite_count = pretty_favourite_count
        self.item_count = item_count
        self.pretty_item_count = pretty_item_count
        self.is_visible_in_listings = is_visible_in_listings
        self.path = path
        self.requires_authenticity_check = requires_authenticity_check
        self.is_luxury = is_luxury
        self.url = url
        self.is_favourite = is_favourite

class Brands:
    def __init__(self, brands, current_page, total_pages, total_entries, per_page, time):
        self.brands = brands
        self.pagination = {
            "current_page": current_page,
            "total_pages": total_pages,
            "total_entries": total_entries,
            "per_page": per_page,
            "time": time
        }
        self.code = 0


def get_data(url, sess):
    response = sess.get(url)
    if response.status_code == 200:
        data = response.json()
        return data
    else:
        return None

def deserialize_json(data):
    brands = []
    for brand in data["brands"]:
        deserialized_brand = Brand(
            id=brand["id"],
            title=brand["title"],
            slug=brand["slug"],
            favourite_count=brand["favourite_count"],
            pretty_favourite_count=brand["pretty_favourite_count"],
            item_count=brand["item_count"],
            pretty_item_count=brand["pretty_item_count"],
            is_visible_in_listings=brand["is_visible_in_listings"],
            path=brand["path"],
            requires_authenticity_check=brand["requires_authenticity_check"],
            is_luxury=brand["is_luxury"],
            url=brand["url"],
            is_favourite=brand["is_favourite"]
        )
        brands.append(deserialized_brand)

    deserialized_data = Brands(
        brands=brands,
        current_page=data["pagination"]["current_page"],
        total_pages=data["pagination"]["total_pages"],
        total_entries=data["pagination"]["total_entries"],
        per_page=data["pagination"]["per_page"],
        time=data["pagination"]["time"]
    )

    return deserialized_data

def exec(brand_name_file, outfilename):

    brand_names= get_brands_names.exec(brand_name_file)

    outfile= open(outfilename, "w")
    outfile.truncate()
    outfile.write("ID, TITLE, URL\n")
    
    res=""

    base_url = "https://www.vinted.es/api/v2/brands?keyword="

    url_cookies = 'https://www.vinted.es/auth/token_refresh'

    sess = requests.Session()
    
    HEADERS = {
                "User-Agent": "PostmanRuntime/7.28.4",  # random.choice(USER_AGENTS),
                "Host": "www.vinted.es",
    }

    sess.headers.update(HEADERS)

    sess.post(url_cookies)
    final_brands = []
    for brand in brand_names:
        
        brand_url = brand.replace("&" , "%26")
    
        url = base_url+brand_url

        # Obtener los datos
        data = get_data(url, sess)

        if data is not None:
            # Serializar el JSON
            deserialized_data = deserialize_json(data)
            # Imprimir los resultados deserializados
            if len(deserialized_data.brands) > 0:
                brands_names : list = [b.title for b in deserialized_data.brands]
                
                to_get = list(set(brands_names) - set (final_brands))
                
                brands_to_add = filter(lambda b : b.title in to_get, deserialized_data.brands)
            
                for b in brands_to_add:
                    br = b.title.replace('\'' , '\'\'') # Escape ' character in SQL
                    br = br.replace('\"' , '\'\'')
                    res+=(f"{b.id} , \'{br}\' , \'{b.url}\'\n")
                    print(b.title + " ✔️")
                    final_brands.append(b.title)
        else:
            print("Error al obtener los datos.")

        #time.sleep(0.1)

    outfile.write(res)
    outfile.close()