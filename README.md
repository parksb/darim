# 🏕 Darim

[![Client CI](https://github.com/ParkSB/darim/workflows/Client%20CI/badge.svg)](https://github.com/ParkSB/darim/actions?query=workflow%3A%22Client+CI%22)
[![Server CI](https://github.com/ParkSB/darim/workflows/Server%20CI/badge.svg)](https://github.com/ParkSB/darim/actions?query=workflow%3A%22Server+CI%22)

* Darim: Diary Improved
* Darim is a personal diary service that supports encryption, calendar view, and markdown syntax.
* You can keep your diary a secret even from the developer through client-side encryption.

![Preview](https://user-images.githubusercontent.com/6410412/87238882-579d4900-c443-11ea-8e81-267b3243237c.png)

## Architecture

* Darim is following the layered architecture.
* Each layer cannot be cross-referenced. All references between layers can flow in a higher direction. In other words, only the upper layer can invoke the lower layer members.

```
+-----------------+----------------+
|  Components     |  API + Models  |
+--------+--------+--------+-------+
         |                 |
+--------+--------+--------+-------+
|  auth  |  post  |  user  |  ...  |
+--------+--------+--------+-------+
|  Pages                           |
+-----------------+----------------+
                  |
+-----------------+----------------+
|  Client (index.html)             |
+-----------------+----------------+
                  |
+-----------------+----------------+
|  Server (main.rs)                |
+-----------------+----------------+
                  |
+-----------------+----------------+
|  Routes                          |
+--------+--------+--------+-------+
|  auth  |  post  |  user  |  ...  |
+----+---+----+---+----+---+---+---+
     |        |        |       |
+----+--------+--------+-------+---+
|  Services                        |
+--------+--------+--------+-------+
|  auth  |  post  |  user  |  ...  |
+--------+--------+--------+-------+
                  |
+-----------------+----------------+
|  Models                          |
+--------+--------+--------+-------+
|  auth  |  post  |  user  |  ...  |
+----+---+----+---+----+---+---+---+
     |        |        |       |
+----+--------+--------+-------+---+
|  Database                        |
+----------------------------------+
```

### [Client](client)

* `index.html` - An entry point of the application. It is built by parcel.
* Pages - Pages represented by URL. Each page can use general components and API fetchers.
* Components - Reusable components used on multiple pages.

### [Server](server)

* `main.rs` - An entry point of the application. It runs a http server.
* Routes - A presentation layer that makes API public and passes request/response data to other layers.
* Services - A business layer that processes the transaction.
* Models - A data layer that can access the database and define data structures.

## License

This project is distributed under the AGPL-3.0 License - see the [LICENSE](LICENSE) file for details.
