import requests
import json
"""
Simple script for obtaining all weapons data from www.dnd5eapi.co/api/

Requirements:
    Requests (https://docs.python-requests.org/en/latest/)

Usage (unix):
    python3 weapon_getter.py > output_file.json

"""
base_url = "https://www.dnd5eapi.co/api/"
base_weapon_url = base_url+"equipment/"
weapon_stats = {} # output

all_weapons = json.loads(requests.get(base_url+"equipment-categories/weapon").text)

for weapon in all_weapons['equipment']:
    weap_index = weapon['index']
    resp = requests.get(base_weapon_url+weap_index)
    all_weapon_stats = json.loads(resp.text)
    
    try:
        name = all_weapon_stats['name']
    except:
        name=None
        #print(weap_index + "has no " + "name")
        continue
    try:
        damage = all_weapon_stats['damage']['damage_dice']
    except:
        damage = None
        #print(weap_index + "has no " + "damage")
        continue
    try:
        props = all_weapon_stats['properties']
    except:
        props = None
        # print(weap_index + "has no " + "properties")

    weapon_out = {
        'name':name,
        'damage':damage,
        'props':props
    }
    weapon_stats[weap_index] = weapon_out
    
print(json.dumps(weapon_stats))

