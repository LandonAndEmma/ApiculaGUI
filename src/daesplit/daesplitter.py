import sys
import xml.etree.ElementTree as ET
import copy

# Registering namespace
ET.register_namespace('', "https://www.collada.org/2008/03/COLLADASchema/")

# Fetching file from command-line argument
my_file = sys.argv[1]

# Parsing XML file
tree = ET.parse(my_file)
root = tree.getroot()

# Namespace dictionary
ns = {'d': 'https://www.collada.org/2008/03/COLLADASchema/'}

# Finding elements
library_animation_clips = root.find('d:library_animation_clips', ns)
library_animation_clips_copy = copy.deepcopy(library_animation_clips)
library_animations = root.find('d:library_animations', ns)
library_animations_copy = copy.deepcopy(library_animations)

# Function to clear library elements
def libraryClear():
    for child in library_animation_clips.findall('d:animation_clip', ns):
        for child in library_animation_clips.findall('d:animation_clip', ns):
            library_animation_clips.remove(child)
        for child in library_animations.findall('d:animation', ns):
            library_animations.remove(child)

# Clearing library elements
libraryClear()

# Counter initialization
i = 0

# Looping through animation clips
for child in library_animation_clips_copy:
    library_animation_clips.append(child)
    
    # Fetching attributes
    att_id    = child.get('id')
    att_name  = child.get('name')
    file_name = att_name + ".dae"
    
    # Looping through joints
    for joint in child:
        library_animations.append(library_animations_copy[i])
        i += 1
    
    # Writing to file
    tree.write(file_name, encoding="utf-8", xml_declaration=True)
    libraryClear()