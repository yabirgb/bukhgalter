# Bukhgalter

## Descripción del proyecto

Este proyecto pretende proporcionar una API para solventar el problema de
particionar pagos entre varias personas y además proveer de herramientas para
planificar pagos en actividades conjuntas. Un ejemplo sería cuando varias
personas se van juntas de viaje y una paga el hotel, otra la comida y al final
quieren ajustar la cuenta de cuanto debe cada persona. También sería interesante
en el viaje conocer cuanto se va gastando y llevar una planificación del
presupuesto disponible.

## Cambios realizados

La lista completa de cambios en cada versión ha sido documentada en un archivo
[changelog](docs/changelog.md). En dicho archivo se encuentran los cambios de cada 
versión y que issues se han cerrado.

## Herramientas

- Como **lenguaje de programación para el proyecto** se ha elegido
  [Rust](https://www.rust-lang.org/).
- Como **gestor de tareas** se ha decidido utilizar `make`. [Justificación](/docs/herramientas.md)
- `Cargo` con su utilidad de `testing` (`cargo test`) sera la **herramienta de tests** que se use. [Justificación](/docs/herramientas.md)

## Docker

En el archivo de [contenerizacion](docs/contenerizacion.md) hago una explicación
del proceso que he seguido para crear los dos contenedores que he creado para el
proyecto.

- [Un contenedor para ejecutar los tests](dockerfiles/testing/slim/Dockerfile)
- [Un contenedor para ejecutar la aplicación](dockerfiles/alpine/Dockerfile)

En el archivo de [CI](docs/CI.md) explico como se ha integrado docker en los
tests y como se despliega el contenedor de tests en dockerhub.

Finalmente en el archivo de [criterios](docs/criterios.md) explico las
precauciones que he tenido al crear los contenedores.

Todas las fuentes de las que he sacado código no propio aparecen documentadas en los
distintos archivos en el apartado de `fuentes`.

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