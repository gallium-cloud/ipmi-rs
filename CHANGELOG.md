# Unreleased
* Support for sending bridged IPMB messages for sensors that are not available on the system
  interface for File-based connections. `GetSensorReading::new()` and `GetSensorReading::for_sensor()`
  have been replaced with`GetSensorReading::for_sensor_key()` which now takes a `&Sensorkey`. ([#6])
* Fix parsing ID String modifier in `CompactSensorRecord` ([#7])


[#6]: https://github.com/datdenkikniet/ipmi-rs/pull/6
[#7]: https://github.com/datdenkikniet/ipmi-rs/pull/7

# [0.2.1](https://github.com/datdenkikniet/ipmi-rs/tree/v0.2.1)

* Use correct bit for detecting signedness of full-record sensor scaling field `B`. ([#4])

[#4]: https://github.com/datdenkikniet/ipmi-rs/pull/4

# [0.2.0](https://github.com/datdenkikniet/ipmi-rs/tree/v0.2.0)

Initial release.