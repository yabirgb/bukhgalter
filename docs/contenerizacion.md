# Contenerización de la aplicación

## Objetivos

Las aplicaciones escritas en `Rust` tienden a crecer en tamaño rápidamente
debido a la compilación de dependencias y el tiempo de compilación de las mismas
es muy grande. 

Por esto uno de los pasos claves cuando queremos contenerizar una aplicación
escrita en `Rust` es evitar la compilación de dependencias con frecuencia.

Otro problema es que si se opta por una imagen de `alpine` como imagen base
puede haber problemas a la hora de compilar si se usan ciertas librerías de 
`C`. 

## Construcción del contenedor

### Elección de la imagen base

Para elegir la imagen base me he decantado entre dos posibilidades:

- `Debian`: Ya que incluye una buena base de software y de librerías para `C` que serán necesarias para compilar las dependencias del proyecto.
- `Alpine`: Ya que es una versión de `linux` famosa por el pequeño tamaño de los
  contenedores que se generan aunque sí se tienen más problemas a la hora de
  trabajar código de `Rust`.

Ya que `alpine` use `musl`, a la hora de compilar las dependencias será
necesario usar `musl` también para compilar las librerías necesarias. Es por eso
que se ha elegido como imagen para la compilación de las mismas `ekidd/rust-musl-builder:stable` que usa `ubuntu` pero compila con `musl`.

### Resultado de la construcción

Tras construir los dos archivos `Dockerfile` con las distintas versiones los
contenedores resultantes tienen los siguientes tamaños:

        REPOSITORY                TAG                 IMAGE ID            CREATED             SIZE
        bukhgalter-alpine         latest              56737fdf344f        31 seconds ago      15MB
        bukhgalter-debian         latest              584ac87e2e5b        31 minutes ago      83.2MB

### Elección del contenedor

A la hora de ejecutar ambos contenedores no he tenido ningún problema que me
haga decantar por uno u otro. A la vista de que el tamaño para la imagen de
`alpine` es mucho mayor y, puesto que no hay ningún requisito técnico que me haga
decantarme con uno frente a otro, he decidido usar el contenedor basado en `alpine`.

## Fuentes 

- [https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/](https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/)