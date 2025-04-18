import json

class Category:
    def __init__(self, id: int, title, code: str, parent_id:int, url,
                 url_en, children_ids: list[int]):
        self.id = id
        self.title = title
        self.code = code
        self.parent_id = parent_id
        self.url = url
        self.url_en = url_en
        self.children_ids = children_ids
        self.children = []

    def __str__(self):
        return f"{self.id},'{self.title}','{self.code}',{self.parent_id},'{self.url}','{self.url_en}'"

    def to_node(self, level):
        indent = "  " * level
        tree_str = f"{indent}- {self.title} ({self.id})\n"
        for child in self.children:
            tree_str += child.__str__()
        return tree_str

def get_data(filename: str):
    file = open(filename, "r")
    r =json.load(file)
    return (r["catalogs"], r["catalog_children_tree"])

def deserialize_json(catalogs: dict, tree: dict):
    res = {}
    for id in catalogs:
        catalog=catalogs.get(id)
        children_ids = tree.get(str(catalog["id"]), [])
        cat = Category(catalog["id"], catalog["title"],
                            catalog["code"], catalog["parent_id"], 
                            catalog["url"], catalog["url_en"], 
                            children_ids)
        res[catalog["id"]] = cat
    return res

def build_tree(category_dict):
        root_categories = []
        category_map = {}

        # Iterate over the category dictionary to create Category objects and populate the category map
        for category_id, category_object in category_dict.items():
            category = Category(category_object.id, category_object.title, category_object.code,
                                category_object.parent_id, category_object.url, category_object.url_en,
                                category_object.children_ids)
            category_map[category_id] = category

        # Build the category tree using the children_ids property
        for category_id, category_object in category_dict.items():
            parent_id = category_object.parent_id
            if parent_id == 0:
                root_categories.append(category_map[category_id])
            else:
                try:
                    parent_category = category_map[parent_id]
                    parent_category.children.append(category_map[category_id])
                except KeyError:
                    root_categories.append(category_map[category_id])

        return root_categories

def generate_cat_tree(source):
    a,b = get_data(source)
    r= deserialize_json(a, b)
    return build_tree(r)

def tree_to_csv(tree: list[Category]):
    res = "ID, TITLE, CODE, PARENT_ID, URL, URL_EN\n"
    tree_str = "ID, CHILD\n"

    def build_csv_string(category):
        nonlocal res, tree_str
        res += str(category) + "\n"
        tree_key = str(category.id)
        for child in category.children:
            aux = build_csv_string(child)
            if(aux is not None):
                res += aux
            tree_str += f"{tree_key}, {str(child.id)}\n"

    for root in tree:
        build_csv_string(root)

    return res, tree_str

def debug(source, outfile):
    outfile = open(outfile, "w")
    res = ""
    for root_category in generate_cat_tree(source):
        res+=root_category.to_node(0)+"\n"
    
    outfile.write(res)
    outfile.close()

def exec(source, outfile1, outfile2):
    out_cat = open(outfile1, "w")
    out_tree = open(outfile2, "w")

    a,b = get_data(source)
    r= deserialize_json(a, b)
    t = build_tree(r)

    res, tree = tree_to_csv(t)

    out_cat.write(res)
    out_tree.write(tree)
    out_cat.close()
    out_tree.close()
