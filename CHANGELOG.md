# ilert-rust CHANGELOG

## 2023-05-13, Version 3.1.0

* added alert accept and resolve operations

## 2023-05-12, Version 3.0.0

* updated dependencies
* fixed a few occurrences of `iLert` -> `ilert` due to rebranding, however decided to keep core structs on `ILert..` as `Ilert` is not much better
* added endpoints to handle `/api/alerts`
* added endpoints to handle `/api/incidents`
* added endpoints to handle `/api/services`
* added traits to support **PUT** and **DELETE** operations in builder style
* added skip() and limit() options to .get() builders
* created client.create() in favor of client.post(); `post()` is now deprecated
* set default timeout to 25 seconds, was 10 seconds
* **BREAKING** renamed `ScheduleApiResource` to `ScheduleGetApiResource`
* **BREAKING** renamed `UserApiResource` to `UserGetApiResource`

## 2021-11-01, Version 2.0.0

* **BREAKING** adjusted client to the new iLert API version `/api/v1` -> `/api`
* **BREAKING** incident_key has been renamed to alert_key
* **BREAKING** event property summary moved from &str -> Option<String>
* added new COMMENT event type
* added new event properties comments and routing_key

## 2020-07-13, Version 1.0.0

* **BREAKING** event split into event, event_with_details
* event_with_details allows for full event including priority, links, images and custom details

## 2020-07-09, Version 0.3.0

* moved to new domain
* added heartbeat
* upgraded deps
* added debug log to urls

## 2020-05-13, Version 0.2.3

* starting the changelog