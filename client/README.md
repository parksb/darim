# Darim Client

[![Client CI](https://github.com/ParkSB/darim/workflows/Client%20CI/badge.svg)](https://github.com/ParkSB/darim/actions?query=workflow%3A%22Client+CI%22)

```
+----------------------------------+
|  Client (index.html)             |
+----------------------------------+
              |
+--------+--------+--------+-------+
|  Pages                           |
+--------+--------+--------+-------+
|  auth  |  post  |  user  |  ...  |
+--------+--------+--------+-------+
     |        |        |
+----------------------------------+
|  Components                      |
+----------------------------------+
```

* `index.html` - An entry point of the application. It is built by parcel.
* Pages - Pages represented by URL. Each page can have its own components and API fetchers.
* Components - Reusable components used on multiple pages.
