# FindSmiley API

## `/restaurant`

Description: Fetches the restaurants in the database  
Type: `GET`  
Return value: Sparse locations with ID's
```json
[
  {
    "id": 1,
    "lat": 55.1321,
    "lng": 8.321321
  },
  {
    "id": 2,
    "lat": 55.321,
    "lng": 8.23145
  }
]
```

## `/restaurant/<id>`
Description: Fetches a single restaurants information  
Type: `GET`  
Return value: Detailed info
```json
{
  "id": 1,
  "lat": 55.321,
  "lng": 8.321321,
  "name": "McDonalds Aalborg",
  "zipcode": 9000
}
```

## `/restaurant/search`
Description: Fetches a single restaurants information  
Type: `GET`  
Parameters:  
* `name`: The name of the restaurant
* `northEast`: NorthEast point of the boundary to fetch restaurants in
* `southWest`: SouthEast point of the boundary to fetch restaurants in  

Valid parameters: 
* Only `name`
* `northEast` & `southWest`

Return value: Array of detailed info
```json
[
  {
    "id": 1,
    "lat": 55.321,
    "lng": 8.321321,
    "name": "McDonalds Aalborg",
    "zipcode": 9000
  }
]
```


## `/subscribe`
Description: Subscribes the user to push notifications for a given restaurant  
Type: `POST`  
Input value(body):
```json
{
  "token": "EXPObababa",
  "restaurant": 2
}
```

## `/unsubscribe`
Description: Unsubscribes the user to push notifications for a given restaurant  
Type: `POST`  
Input value(body):
```json
{
  "token": "EXPObababa",
  "restaurant": 2
}
```
