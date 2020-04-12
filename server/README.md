# Patic Server

* [flosse/rust-web-framework-comparison"](https://github.com/flosse/rust-web-framework-comparison)
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
            "author": "park",
            "content": "Lorem ipsum dolor sit amet",
            "created_at": "2020-04-11T16:31:09",
            "updated_at": null 
        },
        {
            "id": 2,
            "author": "lee",
            "content": "Lorem ipsum dolor sit amet",
            "created_at": "2020-04-10T07:43:03",
            "updated_at": "2020-04-11T16:07:41"
        },
        {
            "id": 1,
            "author": "kim",
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
    "author": "park",
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
    "author": "park"
}
```

```json
{
    "content": "Lorem ipsum dolor sit amet"
}
```

```json
{
    "author": "park",
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
