# Criterios seguidos a la hora de crear los contenedores

Me he basado en las buenas prácticas que se listan en [esta guía](https://jsitech1.gitbooks.io/meet-docker/content/mejores_practicas_dockerfiles.html) y que me han parecido adecuadas
y correctas. 

- He utilizado un archivo `.dockerignore` para evitar copias de archivos autogenerados. 
[https://github.com/yabirgb/bukhgalter/blob/master/.dockerignore](https://github.com/yabirgb/bukhgalter/blob/master/.dockerignore)
- En los dos contenedores que he creado solo se ejecuta un único proceso.
- He utilizado imágenes oficiales de rust y además he especificado la versión de rust 
que quiero para evitar posibles incompatibilidades entre versiones.
- En la imágenes para contenerizar la aplicación he usado una construcción en dos etapas
para reducir el tamaño del contenedor que se genera.
- He especificado un [directorio de trabajo al construir el contenedor](https://github.com/yabirgb/bukhgalter/blob/master/dockerfiles/testing/slim/Dockerfile)
- También hago uso de un usuario distinto al usuario de root cuando [trabajo con mi aplicación](https://github.com/yabirgb/bukhgalter/blob/master/dockerfiles/alpine/Dockerfile)