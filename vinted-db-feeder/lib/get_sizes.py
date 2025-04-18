from typing import List
from bs4 import BeautifulSoup, PageElement
import csv, os

category_to_id = {
    "HOGAR":"1918",
    "HOMBRE":"5",
    "MASCOTAS":"2093",
    "MUJER":"1904",
    "NIÑOS":"1193"
}

class Size:
    def __init__(self, id, title, size_type):
        self.id = id
        self.title = title
        self.category = size_type
    
    def __str__(self):
        return f"{self.id}, '{self.title}', '{self.category}'"


def parse_to_file(path):

    with open(path, 'r') as file:
        contenido_html = file.read()
    
    soup = BeautifulSoup(contenido_html, 'html.parser')

    p = file.name.split('/')
    category_name= p[len(p)-1].split('.')[0].upper()
    #category_id = category_to_id[category_name]

    pile_elements:List[PageElement] = soup.find_all('li', class_='pile__element')

    # Initialize a list to store the size objects
    sizes_list:List[Size] = list()

    # Iterate through the pile__element elements
    for element in pile_elements:
        
        # Get ID
        pile_element_div = element.find('div', class_='web_ui__Cell__cell web_ui__Cell__default web_ui__Cell__navigating')
        pile_element_id = pile_element_div.get('id')
        id_parts = pile_element_id.split('-')
        id = id_parts[-1]

        # Get size name
        size_name = element.find('h2', class_='web_ui__Text__text web_ui__Text__title web_ui__Text__left').text.strip()

        # Get category
        pile = element.parent.fetchPreviousSiblings()[0]

        # Extract the category from the h3 tag
        size_type = pile.find('h3', class_='web_ui__Text__text web_ui__Text__subtitle web_ui__Text__left').text.strip()

        size_obj = Size(id, size_name, size_type)

        sizes_list.append(size_obj)

    # Agregar las etiquetas encontradas a la lista

    return (sizes_list, category_name)

def print_result(lista, category_name) -> str:
    result=""
    for size in lista:
        result+=f"{size}, {category_name}\n"
    return result

def merge_csvs(csv_folder, output_file):
    csv_files = [file for file in os.listdir(csv_folder) if file.endswith('.csv')]
    
    # Create a dictionary to store the merged data
    merged_data = {}
    h = []
    category_id:str

    # Iterate over each CSV file
    for csv_file in csv_files:
        csv_path = os.path.join(csv_folder, csv_file)
        
        # Get the CSV file name without the extension
        csv_name: str = os.path.splitext(csv_file)[0]
        csv_name= csv_name.split(".")[0]
        h.append(csv_name)
        
        # Read the CSV file
        with open(csv_path, 'r', newline='') as file:
            reader = csv.reader(file)
            next(reader)  # Skip the header row
            
            # Iterate over each row in the CSV file
            for row in reader:
                id = row[0]
                title = row[1]
                size_type = row[2]
                category_name = row[3].strip()
                category_id = category_to_id.get(category_name)
                
                # Add the TITLE_CSV_NAME entry to the merged data dictionary
                merged_data.setdefault(id, {})[f'TITLE_{csv_name}'] = title
                merged_data.setdefault(id,{})[f'SIZE_TYPE_{csv_name}'] = size_type
                if category_id is not None:
                    merged_data.setdefault(id,{})["CATEGORY_ID"]=category_id

        
    
    # Write the merged data to the output file
    with open(output_file, 'w', newline='') as file:
        writer = csv.writer(file)
        
        # Write the header row
        h2 = []
        for csv_name in h:
            h2.append(f'TITLE_{csv_name}')
            h2.append(f'SIZE_TYPE_{csv_name}')
        h2.append("CATEGORY_ID")

        header = ['ID'] + h2
        writer.writerow(header)
        
        # Write the data rows
        for id, size_data in merged_data.items():
            row = [id] + [size_data.get(f'TITLE_{csv_name}', '') for csv_name in h] + [size_data.get(f'SIZE_TYPE_{csv_name}', '') for csv_name in h] + [size_data.get("CATEGORY_ID", '')]
            writer.writerow(row)

def exec(path, outfolder, outfilename):
    if(os.path.exists(outfolder)==False):
        os.mkdir(outfolder)

    for fol in os.listdir(path):
        csv_name = fol.upper()+".csv"
        outfile= open(outfolder+csv_name, "w")
        outfile.truncate()
        outfile.write("ID, TITLE, SIZE_TYPE, CATEGORY_NAME\n")

        for html_file in os.listdir(os.path.join(path, fol)):
            lista, category_id = parse_to_file(os.path.join(path, fol, html_file))
            outfile.write(print_result(lista, category_id))

        outfile.close()
    
    merge_csvs(outfolder, outfilename)