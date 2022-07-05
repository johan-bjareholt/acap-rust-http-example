acap-rust-http-example
======================

Tested on an AXIS P1375 on
- 9.80 LTS
- 10.12 LTS
- 11.0 Active

Building is done with the crate "cargo-acap" as follows:

    cargo install cargo-acap
    cargo acap build

Once the ACAP is installed on the device, there is a single HTTP endpoint at http://$DEVICE_IP/acap_rust_http_example

Can also be run on host by running "cargo run", and accessed at http://localhost:8000

The eap takes up 1.4MB, and unpacked it takes up 3.1MB on armv7hf.
If you use a lighter HTTP framework than Rocket, the size could be significantly decreased.
