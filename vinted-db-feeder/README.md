# Vinted data scrapper

A series of data scrappers in Python that extract static information from the Vinted source code, being the perfect complement for a Vinted API wrapper.

## Install & Run

### Requirements

- Pip
- Python

1. Create a virtual environment

   ```bash
   python3 -m venv pyvenv/
   source venv/bin/activate
   ```

2. Run `requirements.txt`

   ```bash
    pip3 install -r requirements.txt
   ```

3. Run `main.py`

## Extracted data categories

| Element             | Fields Returned                                     |
| ------------------- | --------------------------------------------------- |
| Brands              | Names, Ids                                          |
| Materials           | Id, Name                                            |
| Colors              | Id, Color, Hex Code                                 |
| Sizes               | Id, Title, Size_Type, Category_id                   |
| Categories          | Id, Title, Code, Parent Id, URL, URL EN, Item Count |
| Categories Children | Category Id, Child Id                               |
| Countries           | Id, French_name, local_name, ISO_code, flag_emoji   |

### Materials and sizes

- Available languages: 🇪🇸 🇫🇷 🇺🇸
  - [More languages can be added if html file included in `data/raw/materials` or `data/raw/sizes`]

### Categories - Catalogs

- **Debug mode:** Builds the full decision tree

- **Exec mode:** Returns 2 CSVs:

  - `categories.csv`: Table of all the available categories and their attributes

  - `categories_children.csv`: Dictionary that models the Category->Children list relationship

## Performance

- Without brands search:

```bash
real    0m1,941s
user    0m1,225s
sys     0m0,037s
```

- Brands validation process:

```bash
real    14m14,211s
user    0m19,753s
sys     0m1,229s
```

## Authors

[Álvaro Cabo](https://github.com/alvarocabo)

[Pepe Márquez](https://github.com/pxp9)
