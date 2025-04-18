from bs4 import BeautifulSoup
import re, os, csv

def extraer_textos(html_file):
    with open(html_file, 'r') as file:
        text_html = file.read()
        
    soup = BeautifulSoup(text_html, 'html.parser')
    pattern = re.compile(r'material_ids-list-item-(\d+)')
    
    divs = soup.find_all('div', attrs={'id' : pattern})
    
    textos_etiqueta_a = []
    for div in divs:
        match = re.match(pattern, div['id'])
        if match:
            valor_h2 = div.find('h2').text
            numero = match.group(1)
            textos_etiqueta_a.append((numero , valor_h2))
    return textos_etiqueta_a

def print_texts(textos):
    res=""
    for num , texto in textos:
        res+=(str(num) + ', \'' +texto+"\'\n")
    return res

def merge_csvs(csv_folder, output_file):
    csv_files = [file for file in os.listdir(csv_folder) if file.endswith('.csv')]
    
    # Create a dictionary to store the merged data
    merged_data = {}
    h = []

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
                material = row[1]
                
                # Add the MATERIAL_CSVNAME entry to the merged data dictionary
                merged_data.setdefault(id, {})[f'MATERIAL_{csv_name}'] = material
    
    # Write the merged data to the output file
    with open(output_file, 'w', newline='') as file:
        writer = csv.writer(file)
        
        # Write the header row
        h2 = []
        for csv_name in h:
            h2.append(f'MATERIAL_{csv_name}')


        header = ['ID'] + h2
        writer.writerow(header)
        
        # Write the data rows
        for id, material_data in merged_data.items():
            row = [id] + [material_data.get(f'MATERIAL_{csv_name}', '') for csv_name in h]
            writer.writerow(row)


def exec(folder, outfolder, outfilename):
    
    if(os.path.exists(outfolder)==False):
        os.mkdir(outfolder)

    for filename in os.listdir(folder):
        file_name = filename.split(".")[0]+".csv"
        html_file = os.path.join(folder, filename)
        if os.path.isfile(html_file):
            outfile= open(outfolder+file_name, "w")
            outfile.truncate()
            outfile.write("ID, MATERIAL\n")
            textos = extraer_textos(html_file)
            outfile.write(print_texts(textos))
            outfile.close()
    
    f = open(outfilename, "w")
    f.close()

    merge_csvs(outfolder, outfilename)