Las estructuras de datos que me van a permitir cumplir con las tareas de las
historias de usuario están definidas en la carpeta [src/db](src/db/). Más concretamiente

- En el archivo [models](/src/db/models.rs) se definen las estructuras que van a
  representar de manera interna las estructuras que van a permitir cumplir con las HU. Además en este archivo se encuentran las implementaciones de las funciones que van a ser necesarias.

- En el archivo [db](/src/db/db.rs) se encuentra la descripción para las
  abstracciones necesarias y recomendadas por `mongodb` para conectarse a la
  base de datos.
