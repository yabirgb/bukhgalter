# Bukhgalter

## Descripción del proyecto

Este proyecto pretende proporcionar una API para solventar el problema de
particionar pagos entre varias personas y además proveer de herramientas para
planificar pagos en actividades conjuntas. Un ejemplo sería cuando varias
personas se van juntas de viaje y una paga el hotel, otra la comida y al final
quieren ajustar la cuenta de cuanto debe cada persona. También sería interesante
en el viaje conocer cuanto se va gastando y llevar una planificación del
presupuesto disponible.

## Despliegue

Toda la información relativa al despliegue se encuentra en [PaaS](docs/paas.md)

## Microservicio

La información relativa a la REST API que se expone se encuentra en la
documentación sobre el [Microservicio](docs/microservicio.md) y en la de las
[urls](docs/URLS.md).

Tambien hay un apartado en [Microservicios/diseño](docs/microservicio.md#diseño)
que explica como se ha esrtructurado el microservicio y como se han desacoplado
en la medida de lo posible los distintos componentes del mismo. También se
documenta como se maneja actualmente el almacenamiento de datos.

[Microservicios/testing](docs/microservicio.md#testing) informa de la estructura
de los tests.

## Logging

Para obtener información sobre las peticiones que se procesan en el
microservicio se pone a disposición un sistema de logging con capacidad de
enviar los logs a otro servicio mediante syslog. Para saber como funciona 
y se integra en el proyecto consultar la documentación [logs](docs/logs.md).

## Configuración

El proyecto cuenta con distintos parámetros para ajustar el comportamiento del
microservicio. Se utilizan tanto variables de entorno como `etcd` a modo de
gestor de configuración distribuida. En el apartado de
[configuración](docs/configuracion_distribuida.md) se especifica la política
para encontrar la configuración y las variables que se buscan al arrancar el
microservicio.

## Ejecutar el servicio

Se ha creado un contenedor docker para ejecutar el microservicio. El archivo
[Dockerfile](https://github.com/yabirgb/bukhgalter/blob/master/dockerfiles/debian/Dockerfile)
se ha adaptado a partir del creado en versiones anterior. Para ejecutar el
contenedor hay que proporcionar las variables de entorno. Un ejemplo sería

    docker run -p 8000:8000 -e log_host="logs.papertrailapp.com" -e log_port=$PORT_LOG -e host="0.0.0.0" -e port=8000 -e RUST_LOG=Info image:latest

Para ejecutar el contenedor sin docker hay que:

1. Clonar el repositorio
2. Ejecutar `make release`
3. Crear las variables de entorno necesarias
4. Ejecutar `./target/release/bukhgalter`

## Serverless

Se ha documentado la parte [serverless](docs/serverless.md) de la aplicación en
la documentación  del proyecto. Se han creado un función en rust que devuelve un
`json` con información. Por motivos técnicos ha sido alojada en otro repositorio
[https://github.com/yabirgb/bukhgalter-collaborators](https://github.com/yabirgb/bukhgalter-collaborators).

También se ha creado un bot de telegram desplegado en netlify
[@bukhgalterbot](http://t.me/bukhgalterbot) y una interfaz web usando javascript y
el framework svelte. Esta interfaz se encuentra en la carpeta
[ui](https://github.com/yabirgb/bukhgalter/tree/master/ui) y está 
[desplegada en netlify](https://bukhgalter.netlify.app/).

Para más detalles remitirse a la documentación.

## Herramientas

- Como **lenguaje de programación para el proyecto** se ha elegido
  [Rust](https://www.rust-lang.org/).
- Como **gestor de tareas** se ha decidido utilizar `make`. [Justificación](/docs/herramientas.md)
- `Cargo` con su utilidad de `testing` (`cargo test`) sera la **herramienta de tests** que se use. [Justificación](/docs/herramientas.md)

## Integración continua

Respecto a los sistemas de integración continua, se han descrito cual ha sido su propósito
de uso [en el capítulo de CI de la documentación](docs/CI.md).

Se han configurado como sistemas de CI:

- [Circle-CI](https://app.circleci.com/pipelines/github/yabirgb/bukhgalter) (hay que loguearse para ver las builds. Se puede ver una [foto aquí](docs/images/circle_ci.png). Con la UI antigua [sí se pueden ver](https://circleci.com/gh/yabirgb/bukhgalter)).

- [Travis](https://travis-ci.com/github/yabirgb/bukhgalter/builds/)

## Docker

En el archivo de [contenerizacion](docs/contenerizacion.md) hago una explicación
del proceso que he seguido para crear los dos contenedores que he creado para el
proyecto.

- [Un contenedor para ejecutar los tests](dockerfiles/testing/slim/Dockerfile)
- [Un contenedor para ejecutar la aplicación](dockerfiles/alpine/Dockerfile)

En el archivo de [CI](docs/CI.md) explico como se ha integrado docker en los
tests y como se despliega el contenedor de tests en dockerhub. A raiz de un comentario 
en el grupo de Telegram he cambiado y he configurado también la acción desde dockerhub
para que se genere el contenedor. Lo documento en el archivo [dockerhub](docs/dockerhub.md).

Finalmente en el archivo de [criterios](docs/criterios.md) explico las
precauciones que he tenido al crear los contenedores.

Todas las fuentes de las que he sacado código no propio aparecen documentadas en los
distintos archivos en el apartado de `fuentes`.

El contenedor de tests se encuentra publicado en:

- [Dockerhub](https://hub.docker.com/repository/docker/yabirgb/bukhgalter)
- [Github Container Repository](https://github.com/users/yabirgb/packages/container/package/bukhgalter)

## Organización del proyecto

La estructura del proyecto y donde encontrar **archivos relevantes** se encuentra
disponible en la [documentación](docs/organizacion.md).

Algunos sitios claves del proyecto son:

- [Documentación](docs/)
- [Código implementado](src/)
- [Carpeta de tests](tests/)
- [Archivo de descripción para la asignatura](iv.yaml)

## Algunas funcionalidades

Es necesario tener `Rust` instalado junto a `Cargo` y `make`.

Para comprobar que el proyecto no tiene ningún problema de sintaxis

    make check

Para instalar las dependencias y compilar el proyecto usar

    make build 

Para ejecutar los tests

    make test

Para ejecutar el servicio usar

    make run

Para instalar las dependencias y compilar en modo de producción

    make release

Si se desea la documentación generada por rust para el proyecto

    make docs

## Documentación

A continuación se lista información de que se puede encontrar en la [documentación](docs).
Una versión web de la documentación se encuentra en la 
[página web del proyecto](https://yabirgb.github.io/bukhgalter/) en GitHub-Pages.

- En [el archivo de herramientas](docs/herramientas.md) se puede leer una
  argumentación de por qué se ha elegido cada herramienta que se ha listado con anterioridad.

- En [el arhivo de setup](docs/setup.md) se encuentran detalles sobre como se ha
  configurado git.

- [Historias de Usuario](docs/HU.md) que describen los requisitos del proyecto.

- [Variables de entorno usadas](docs/env.md)

- [Información sobre la contenerización del proyecto](docs/contenerizacion.md)


## Autor

- [Yábir García](https://github.com/yabirgb)
