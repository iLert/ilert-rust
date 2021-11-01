# ilert-rust CHANGELOG

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