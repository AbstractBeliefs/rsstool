import requests
from bs4 import BeautifulSoup

soup = BeautifulSoup(requests.get("https://avherald.com/").text, "html.parser")
soup = soup.find("td", id="ad1cell")

headers = soup.find_all("td", align="center")
for header in headers:
    print "[" + header.find("img")["alt"] + "]",
    print header.next_sibling.find("span", "headline_avherald").text + ":",
    print "https://avherald.com" + header.next_sibling.find("a")["href"]
