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

![main transaction flow](https://user-images.githubusercontent.com/6410412/93713894-dc21eb80-fb99-11ea-8b3c-f689bbf05876.png)

### [Client](client)

* `index.html` - An entry point of the application. It is built by parcel.
* Pages - Pages represented by URL. Each page can use general components, API fetchers, and models.
* Components - Reusable components used on multiple pages.

### [Server](server)

* `main.rs` - An entry point of the application. It runs a http server.
* Routes - A presentation layer that makes API public and passes request/response data to other layers.
* Services - A business layer that processes the transaction.
* Models - A data layer that can access the database and define data structures.

## Client-side Encryption

* Darim supports client-side encryption to protect the user's secrect from others including server.

### Generate keys

![key generation flow](https://user-images.githubusercontent.com/6410412/91041309-c37dee80-e64a-11ea-9ac0-75dc0d810aa8.png)

1. When a user finishes the sign-up process, the secret key and public key are generated on the client-side.
1. The client encrypts the secret key by public key and saevs the encrypted secret key in local storage.
1. The public key is sent to the server, and the server stores it.

### Read & Write

![read and write flow](https://user-images.githubusercontent.com/6410412/91042440-b530d200-e64c-11ea-86f5-dfbcf025bdf4.png)

1. When a user creates the plaintext post, the client requests the public key to the server.
1. The client decrypts the encrypted secret key in the local storage using the public key from the server.
1. The plaintext post is encrypted by the secret key decrypted by the public key.
1. The encrypted post is sent to the server, and the server stores it.

> * At this point, the server can only know encrypted post.
> * When the client requests the server to read the post, whole flows are reversed.

## License

This project is distributed under the AGPL-3.0 License - see the [LICENSE](LICENSE) file for details.
