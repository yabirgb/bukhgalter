# Bukhgalter

## Descripción del proyecto

Este proyecto pretende proporcionar una API para solventar el problema de
particionar pagos entre varias personas y además proveer de herramientas para
planificar pagos en actividades conjuntas. Un ejemplo sería cuando varias
personas se van juntas de viaje y una paga el hotel, otra la comida y al final
quieren ajustar la cuenta de cuanto debe cada persona. También sería interesante
en el viaje conocer cuanto se va gastando y llevar una planificación del
presupuesto disponible.

## Herramientas

- Como **lenguaje de programación para el proyecto** se ha elegido
  [Rust](https://www.rust-lang.org/).

- Como **base de datos** se ha elegido `mongodb`.

## Organización del proyecto

La estructura del proyecto y donde encontrar **archivos relevantes** se encuentra
disponible en la [documentación](docs/organizacion.md).

Algunos sitios claves del proyecto son:

- [Documentación](docs/)
- [Código implementado](src/)
- [Carpeta de tests](tests/)
- [Archivo de descripción para la asignatura](iv.yaml)

## Iniciar el proyecto

Es necesario tener `Rust` instalado junto a `Cargo` y `make`.

Para instalar las dependencias y compilar el proyecto usar

    make build 

Para ejecutar los tests

    make test

Para ejecutar el servicio usar

    make run

Para instalar las dependencias y compilar en modo de producción

    make release

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


## Autor

- [Yábir García](https://github.com/yabirgb)