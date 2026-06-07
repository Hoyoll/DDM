# DDM
Music player for ddd

# How to use!
- Setup a repo.json file under $USER/.config/ddm/
- Fill it with json with this schema
```
[
    {
    "name": <string>, 
    "url": <string> // https://api.github.com/repos/<user>/<repo>/contents,
    "token": <string> // your github token so you can authenticate the request, it's preferable this way
    }
]
