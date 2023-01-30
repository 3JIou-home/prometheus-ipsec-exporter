# Сервис для экспорта основных метрик vpn в prometheus.

## Принцип работы.
Выполняет по запросу 2 команды:
```shell
ipsec status | awk {'print $1'} | grep -v 'Security' | cut -d \"{\" -f1 | cut -d \"[\" -f1 | uniq  | wc -l
ip xfrm state | grep 'src' |wc -l
```

## Ключи.
Принимается только один аргумент - адрес биндинга для метрик.
```shell
--bind 0.0.0.0:9090
```
по этому сокету будут доступны метрики.
```shell
common-vpn1.alakt.kz.prod.bash.kz:/home/batman# curl common-vpn1.alakt.kz.prod.bash.kz:9090/metrics
ipsec.vpn.active_tunnels: 2
ipsec.vpn.active_connection: 4
```