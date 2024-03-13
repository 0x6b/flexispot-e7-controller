# web

Web server for Raspberry Pi.

See [`e7server.toml.sample`](e7server.toml.sample) for configuration sample, and [`e7server.service.sample`](e7server.service.sample) for systemd configuration sample.

```console
$ cp e7server.toml.sample /etc/e7server.toml
$ cp e7server.service.sample /etc/systemd/system/e7server.service
$ $EDITOR /etc/systemd/system/e7server.service
$ sudo systemctl enable e7server
$ sudo systemctl start e7server
$ sudo journalctl -u e7server.service
```
