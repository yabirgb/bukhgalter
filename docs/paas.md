# Despliegue en PAAS

## Servicio elegido

Para desplegar mi aplicación me he decantado por usar [Heroku](https://heroku.com). Los motivos para esta
elección han sido los siguientes:

- Tenía que poder desplegar una aplicación en Rust o ofrecer despliegue de contenedores.
- Debe permitir especificar la infraestructura y requisitos del proyecto.
- Buena integración con los servicios de integración continua.
- Debe tener un plan gratuito.

### Alternativas consideradas

#### Dokku

- Es una aplicación para self-hosting y por lo tanto no tiene plan gratuito.

#### Back4App

- Es una plataforma más orientada a bases de datos.
- El plan gratuito se encuentra muy limitado.

#### Heroku

- Con el pack student de github educations podemos obtener un mejor plan.
- Herramienta de CLI muy completa.
- Existen buildpacks para rust.
- Buen soporte para docker.

De entre las anteriores la que cumple todos los requisitos y más ventajas me aportaba era Heroku.

## Métodos para el despliegue

He probado tres estrategias de despliegue distinta

### Despliegue mediante Procfile

He probado a usar el buildpack de rust para hacer deploy del proyecto usando un archivo [Procfile](https://github.com/yabirgb/bukhgalter/blob/4e9f301c1c37ffe2dd9e83afc3b1cb05c1b4d652/Procfile). El problema con 
este método de despligue es que no puedo especificar de manera sencilla otros aspectos del despliegue como
el uso de addons.

### Despliegue mediante webhook

La primera técnica de despliegue que he probado ha sido la de especificar en el archivo [heroku.yml](heroku.yml) (archivo donde puedo especificar las características del despliegue) y conectar 
heroku con el repositorio de github desde la configuración de la app de heroku.

Este método de despligue a tenido el contratiempo de que el contenedor se construye utilizando 
los dynos de mi cuenta, y debido a las limitaciones de los mismos, la construcción del proyecto 
es extremadamente lenta. Un despligue ha tardado 40 minutos en ejecutarse.

![](images/heroku_deploy.png)

### Despliegue utilizando CD


Mi tercera estrategia y con la que me he quedado ha sido utilizar los workflows de github para realizar 
el despligue. He comprobado que al utilizar la utilidad de CLI de heroku puedes compilar el cotenedor
que quieres utilizar y subirlo a los repositorios de heroku de manera que se puede utilizar ese contenedor
para poner a funcionar la aplicación.

De esta manera los contenedores no se compilan en el dyno de heroku, sino en la máquina local. Mi experiencia es que los servidores de github son rápidos para compilar y, por este motivo, me he 
decantado por utilizarlos para hacer el despliegue. 

En primer lugar [creamos la aplicación](https://devcenter.heroku.com/changelog-items/1441) a partir del `manifest` (el archivo `heroku.yml`) para que la aplicación se genere con la estructura y addons que necesitamos.

    heroku create --manifest

En el archivo [de configuración para los workflows de github](https://github.com/yabirgb/bukhgalter/blob/master/.github/workflows/testing.yml) he especificado que una vez los tests hayan pasado, se proceda
a hacer deploy de la aplicación utilizando la utilidad de CLI de heroku. Esto sucede en la rama master, que
será la que utilice como rama para hacer los deploys. Un ejemplo de deploy ha sido [este](https://github.com/yabirgb/bukhgalter/runs/1674792424?check_suite_focus=true) donde ha tardado 17 minutos en completar el compilado de la aplicación.

## Problemas encontrados

- Uno de los principales problemas que he tenido han sido los altos tiempos de compilación, que no permiten
un despliegue rápido del servidor. Esto es debido a que se compila utilizando el flag `--release` del compilador que realiza optimizaciones en todas las dependencias además de realizar las comprobaciones sobre
los tipos y la memoria que hace Rust.

- El addon oficial de mongo ha sido eliminado del marketplace de heroku.

## Bibliografía

- [Archivo de configuración de heroku](https://devcenter.heroku.com/articles/build-docker-images-heroku-yml)
- [Despliegue utilizando contenedores](https://devcenter.heroku.com/articles/container-registry-and-runtime)
- [Crear aplicaciones a partir de un manifiesto](https://devcenter.heroku.com/changelog-items/1441)