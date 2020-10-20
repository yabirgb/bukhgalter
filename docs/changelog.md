# Cambios realizados entre hitos

## Hito 2 (REENVIO)

- Se [documenta](herramientas.md#herramienta-de-tests) por que se ha utilizado `cargo tests`, otras herramientas de test
  que se consideraron, el conjunto de librerías de aserciones que se usa...

- Se han documentado en los mismos tests lo que se hace y la HU de usuario con la que se relaciona. 
  También se ha hecho una revisión de algunos y todo se ha desarrollado en relación con los issues
  
  - [#39](https://github.com/yabirgb/bukhgalter/issues/39)
  - [#41](https://github.com/yabirgb/bukhgalter/issues/41)
  - [#43](https://github.com/yabirgb/bukhgalter/issues/43)
  - [#40](https://github.com/yabirgb/bukhgalter/issues/40)
  - [#42](https://github.com/yabirgb/bukhgalter/issues/42)

  Y finalmente se ha cerrado un `issue` que originó esta separación por HU [#37]((https://github.com/yabirgb/bukhgalter/issues/37))

- Se ha refactorizado el código de tests para eliminar las constantes que
  aparecían sueltas y para hacer uso de `fixtures`. Se desarrolla tras la creación de [#36](https://github.com/yabirgb/bukhgalter/issues/36)

- Se ha eliminado el código relacionado con `mongodb` y que no pertenecía a estos hitos. [#38](https://github.com/yabirgb/bukhgalter/issues/38)

## Hito 2

### Historias de usuario creadas

- [HU 4](https://github.com/yabirgb/bukhgalter/issues/15) Actualización de las proporciones de deudas
- [HU 5](https://github.com/yabirgb/bukhgalter/issues/28) Conocer datos de una lista sobre el estado de las deudas

### Descripción de cambios realizados

Para trabajar en este hito se han agrupado [varios issues en un milestone de github](https://github.com/yabirgb/bukhgalter/milestone/4?closed=1).

- [PR #17](https://github.com/yabirgb/bukhgalter/pull/17/files#diff-dfa4a9583bac7bc8fce6f734c62c90119f100176ea007dd898820618a274f02c) Se ha introducido una nueva [clase de error](https://github.com/yabirgb/bukhgalter/blob/master/src/db/errors.rs#L16) 
para capturar los errores que se pueden producir en las implementaciones para las estructuras del proyecto. 

- [PR #17](https://github.com/yabirgb/bukhgalter/pull/17), [#11](https://github.com/yabirgb/bukhgalter/issues/11) Se ha añadido un [fichero de tests](https://github.com/yabirgb/bukhgalter/blob/master/tests/test_models.rs) donde se testean las implementaciones para
  las tres estructuras asociadas al proyecto. Entre otros [también se han testeado](https://github.com/yabirgb/bukhgalter/blob/master/tests/test_models.rs#L292) los posibles errores que se han descrito en el fichero anterior. Los tests comprueban que los datos para las cuentas se crean de manera correcta y que las reglas que se especifican en las historias de usuario [HU4](https://github.com/yabirgb/bukhgalter/issues/15) y [HU2](https://github.com/yabirgb/bukhgalter/issues/9) se verifican. Se han creado tests que introducen datos correctos y tests con datos erróneos para detectar que la implementación del código tiene en cuenta los diferentes casos posibles. 

- [PR #19](https://github.com/yabirgb/bukhgalter/pull/19), [#16](https://github.com/yabirgb/bukhgalter/issues/16) Ahora los tests se
  ejecutan de manera automática usando 
  [github actions](https://github.com/yabirgb/bukhgalter/blob/master/.github/workflows/testing.yml).
  En esta `action` se realizan dos tareas distintas. La primera de ellas utiliza
  la utilidad `check` de cargo para comprobar la corrección sintáctica del
  código así como que este compila correctamente. Además se comprueba que las
  dependencias que se definen para el proyecto no tienen ningún problema de
  incompatibilidad. La segunda tarea se dedica a ejecutar los tests que se han
  definido.

  En ambos casos, y tras haber experimentado con la ejecución de estas `actions`,
  he decido utilizar `actions/cache@v2` que permite cachear algunos pasos de la
  compilación de Rust. He seguido las indicaciones que se presentan en este post
  [https://levans.fr/rust_travis_cache.html](https://levans.fr/rust_travis_cache.html)
  y que recomienda solo cachear ciertos directorios de cargo para evitar _builds_ lentas.

  **Me he basado en los ejemplos que se proporcionan en el repositorio de estas `actions`
  para construir las mías** [https://github.com/actions-rs/cargo](https://github.com/actions-rs/cargo)

- [#12](https://github.com/yabirgb/bukhgalter/issues/12) `make` será el gestor de tareas que se va a utilizar en el proyecto. Una
  justificación del motivo del mismo se encuentra en [la documentación](https://github.com/yabirgb/bukhgalter/blob/master/docs/herramientas.md). Las reglas que se han añadido al `Makefile` se encuentras explicadas en el archivo [README](https://github.com/yabirgb/bukhgalter#iniciar-el-proyecto)

- [PR #18](https://github.com/yabirgb/bukhgalter/pull/18), [#14](https://github.com/yabirgb/bukhgalter/issues/14) Se ha finalizado con
  la implementación de los métodos básicos a nivel de estructura de datos que
  van a ser necesarios para cubrir las historias de usuario HU1 HU2 HU3 HU4. Más
  concretamente el grueso de la lógica se encuentra en la implementación de la
  estructura
  [Account](https://github.com/yabirgb/bukhgalter/blob/master/src/db/models.rs#L83)

- [#27](https://github.com/yabirgb/bukhgalter/issues/27) Se ha añadido este
  archivo de cambios atendiendo a los datos que es necesario mostrar de manera organizada.

- [#25](https://github.com/yabirgb/bukhgalter/issues/25) La documentación de las [estructuras creadas para le proyecto](https://github.com/yabirgb/bukhgalter/blob/master/src/db/models.rs)
  ha sido actualizada. Ahora se incluye información más detallada de cada estructura y los métodos más complejos han sido documentados de manera correcta. Además esta documentación se puede 
  `compilar` de manera que se genera documentación a partir de los comentarios del código en formato _html_.

### Mejoras introducidas sobre aspectos anteriores

- [#26](https://github.com/yabirgb/bukhgalter/issues/26) No se documentó en el
  hito anterior la corrección del código implementado. Para que quede más claro
  se ha añadido una entrada en el `README` y dichos tests también se ejecutan en 
  `github actions`.
