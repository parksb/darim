# Patic Server

[![Server CI](https://github.com/ParkSB/patic/workflows/Server%20CI/badge.svg)](https://github.com/ParkSB/patic/actions?query=workflow%3A%22Server+CI%22)

* [flosse/rust-web-framework-comparison](https://github.com/flosse/rust-web-framework-comparison)
* [thecloudmaker/actix_tutorials](https://github.com/thecloudmaker/actix_tutorials)
* [actix/examples](https://github.com/actix/examples)
* [diesel.rs](http://diesel.rs/)
* [Yoshua Wuyts, "Error Handling Survey", 2019](https://blog.yoshuawuyts.com/error-handling-survey/)

## List posts

```
GET /posts
```

### Response

```json
{
    "data": [
        {
            "id": 3,
            "user_id": 1,
            "user_name": "park",
            "user_avatar_url": "image.jpg",
            "content": "Lorem ipsum dolor sit amet",
            "created_at": "2020-04-11T16:31:09",
            "updated_at": null 
        },
        {
            "id": 2,
            "user_id": 2,
            "user_name": "lee",
            "user_avatar_url": null,
            "content": "Lorem ipsum dolor sit amet",
            "created_at": "2020-04-10T07:43:03",
            "updated_at": "2020-04-11T16:07:41"
        },
        {
            "id": 1,
            "user_id": 3,
            "user_name": "kim",
            "user_avatar_url": null,
            "content": "Lorem ipsum dolor sit amet",
            "created_at": "2020-04-07T13:16:06",
            "updated_at": null
        }
    ]
}
```

## Create a post

```
POST /posts
```

### Parameters

```json
{
    "user_id": 1,
    "content": "Lorem ipsum dolor sit amet"
}
```

### Response

```json
{
    "data": true
}
```

## Update a post

```
PATCH /posts/:id
```

### Parameters

```json
{
    "content": "Lorem ipsum dolor sit amet"
}
```

### Response

```json
{
    "data": true
}
```

## Delete a post

```
DELETE /posts/:id
```

### Response

```json
{
    "data": true
}
```
