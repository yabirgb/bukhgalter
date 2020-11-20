# Serverless

## Objetivo

Se han creado dos historias de usuario
([HU6](https://github.com/yabirgb/bukhgalter/issues/56)
[HU7](https://github.com/yabirgb/bukhgalter/issues/57)). Para cubrir a nivel de
backend las necesidades de _HU7_ se ha decidido crear un servicio serverless 
para listar las organizaciones que colaboran con el proyecto.

## Elección de la plataforma

Al estar mi código desarrollado en _Rust_ he encontrado dificultades para
encontrar plataformas con buen soporte. Me he decantado por **Vercel** que,
aunque no lo soporta de manera oficial, sí cuenta con soporte para Rust hecho 
por la comunidad.

## Código desarrollado

He desarrollado una función que lee una lista en formato string pero con la
estructura de un documento `json`, y la devuelve marcando el tipo de contenido
como `application/json`.


    fn handler(_: Request) -> Result<impl IntoResponse, NowError> {
        let response = Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(collaborators.to_string())
            .expect("Internal Server Error");

            Ok(response)
    }

El código desarrollado con la configuración se encuentra disponible en [bukhgalter-collaborators](https://github.com/yabirgb/bukhgalter-collaborators).

Como precaución he seguido el consejo proporcionado en clase y, en lugar de leer
un archivo cada vez que se hace una petición, he leido una variable que se lee
en tiempo de compilación y está un archivo separado del resto del código. De
esta manera se puede generar de manera rápida y automática sin necesidad y no
se pierde tiempo en accesos al posible archivo. 

Además se ha reducido el tamaño de la respuesta lo máximo posible para consumir
únicamente lo necesario en ancho de banda y que la respuesta sea lo más rápida
posible.


## Problemas encontrados

1. El primer problema que he encontrado es que no se puede tener la última
   versión del microframework web que se usa. La razón es que la librería `lambda`
   que sirve para generar una estructura de petición compatible con `vercel` no es
   compatible con la nueva versión del microframework web y por tanto no se puede elevar 
   la versión del mismo.

2. El segundo problema es que la librería disponible para usar `vercel` con
   _Rust_ no sigue siendo activamente desarrollada y no están disponibles todas
   las funcionalidades actuales. La principal que he echado en falta es que no
   se puede enlazar un proyecto en una subcarpeta por lo que no he podido hacer
   autodeploy de mi función serverless. Me he decantado por mover lo necesario a
   un proyecto nuevo [bukhgalter-collaborators](https://github.com/yabirgb/bukhgalter-collaborators).