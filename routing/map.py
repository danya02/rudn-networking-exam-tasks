# Module for managing the geographical routing maps.

from typing import List
from enum import Enum
import json

@lru_cache
def get_cities() -> :
    with open('cities.json/cities.json') as o:
        return list(map(City.from_json, json.load(o)))

class City:
    """
    Represents a geographical city loaded from a file.
    """
    def __init__(self, country, name, lat, lng):
        self.country = country
        self.name = name
        self.lat = lat
        self.lng = lng
    
    @classmethod
    def from_json(cls, data):
        return cls(
            data['country'],
            data['name'],
            float(data['lat']),
            float(data['lng']),
        )


class GeoMap:
    """
    Represents a geographical map of routers and computers.
    """

    def __init__(self):
        self.nodes: List[GeoNode] = []

class GeoNodeRole(Enum):
    ROUTER = 1
    COMPUTER = 2

class GeoNode:
    """
    Represents a geographical location (such as a city)
    and its network role.
    """
    def __init__(self, name, country, lat, lng, role):
        self.role: GeoNodeRole = role
        self.name: str = name
        self.country: str = country
        self.lat: float = lat
        self.lng: float = lng
    
    @classmethod
    def from_city_by_role(cls, role: GeoNodeRole):
        """
        Construct a GeoNode from a random city 
        """