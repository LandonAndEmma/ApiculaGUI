import sys
import xml.etree.ElementTree as ET
import copy

ET.register_namespace('', "http://www.collada.org/2005/11/COLLADASchema")

my_file = sys.argv[1]

tree = ET.parse(my_file)
root = tree.getroot()

ns = {'d': 'http://www.collada.org/2005/11/COLLADASchema'}

library_animation_clips = root.find('d:library_animation_clips', ns)
library_animation_clips_copy = copy.deepcopy(library_animation_clips)
library_animations = root.find('d:library_animations', ns)
library_animations_copy = copy.deepcopy(library_animations)

def libraryClear():
    for child in library_animation_clips.findall('d:animation_clip', ns):
        for child in library_animation_clips.findall('d:animation_clip', ns):
            library_animation_clips.remove(child)
        for child in library_animations.findall('d:animation', ns):
            library_animations.remove(child)
    return

libraryClear()

i = 0

for child in library_animation_clips_copy:
    
    library_animation_clips.append(child)
    
    att_id    = child.get('id')
    att_name  = child.get('name')
    file_name = att_name + ".dae"
    
    for joint in child:
        library_animations.append(library_animations_copy[i])
        i = i + 1
    
    tree.write(file_name, encoding="utf-8", xml_declaration=True)# <?xml version="1.0" encoding="utf-8"?>
    libraryClear()
