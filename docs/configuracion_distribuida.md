# Configuración

## Flujo para conseguir parámetros

Para obtener las variables que parametrizan el microservicio he seguido la
siguiente estrategia:

1. En primer lugar se comprueba si existen las variables de entorno `ETCD_SERVER`
  y `ETCD_PORT`. Si no existe se asignan por defecto `localhost` y `2739` respectivamente.
1. Se intenta conectar al servidor de `etcd` especificado en el paso anterior. Si
  no existe, se avanza al paso 4
1. Si existe se intenta buscar la variable que se ha solicitado en `etcd`. Si la
   busqueda no devuelve ningún valor se avanza al siguiente paso. En caso contrario se devuelve el valor obtenido.
1. Si no existe la variable en `etcd` se opta por buscar en el entorno. Si no
   existe la variable en el entorno se devuelve  un error `VariableNotFound`, en
   caso contrario se devuelve  el valor de la variable.

## Variables de entorno

- `mode`: Indica el modo del servidor, `prod` (producción) o `dev` (desarrollo).
- `ETCD_SERVER`: host para `etcd`.
- `ETCD_PORT`: puerto de `etcd`.
- `log_host`: host para el sistema de logging. Requerido en `mode=prod`.
- `log_port`: puerto para el sistema de logging. Requerido en `mode=prod`.
- `host`: Ip address del servidor (`localhost` por defecto).
- `port`: Puerto para levantar el servidor (`8000` por defecto).