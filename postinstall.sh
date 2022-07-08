#!/bin/sh -e
cp -v /usr/local/packages/acap_rust_http_example/reverseproxy.conf /etc/apache2/conf.d/acap_rust_http_example_reverseproxy.conf
systemctl reload httpd
