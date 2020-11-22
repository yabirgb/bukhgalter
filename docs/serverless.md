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
por la comunidad. He descartado plataformas como _azure_ por no contar con un
soporte bien documentado para _Rust_ o _aws_ por requerir una configuración 
que no es la convencional y exigir el uso de herramientas muy concretas para 
_Rust_.

## Código desarrollado

He desarrollado una función que lee una lista en formato string pero con la
estructura de un documento `json`, y la devuelve marcando el tipo de contenido
como `application/json`.


    fn handler(_: Request) -> Result<impl IntoResponse, NowError> {
        let response = Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .body(collaborators.to_string())
            .expect("Internal Server Error");

            Ok(response)
    }

El código desarrollado con la configuración se encuentra disponible en [bukhgalter-collaborators](https://github.com/yabirgb/bukhgalter-collaborators).

En esta pieza de código lo que estamos haciendo es crear una respuesta http en
la que especificamos el código de la misma, que es un documento `json`, que
cualquiera puede hacer una petición al endpoint (he decidido hacerlo así para
permitir que otra gente pueda acceder ya que no es contenido crítico).
Finalmente le pasamos al cuerpo la cadena json y si algo falla devolvemos un
código de error.

Como precaución he seguido el consejo proporcionado en clase y, en lugar de leer
un archivo cada vez que se hace una petición, he leido una variable que se lee
en tiempo de compilación y está un archivo separado del resto del código. De
esta manera se puede generar de manera rápida y automática sin necesidad y no
se pierde tiempo en accesos al posible archivo. 

Además se ha reducido el tamaño de la respuesta lo máximo posible para consumir
únicamente lo necesario en ancho de banda y que la respuesta sea lo más rápida
posible.

Se puede hacer una petición en
[http://bukhgalter-collaborators.vercel.app/](http://bukhgalter-collaborators.vercel.app/).
El repositorio ha sido [enlazado con vercel](vercel.md)

## Desplegado de la UI

Para completar mi objetivo con la HU he creado una pequeña interfaz de usuario
que por ahora muestra los colaboradores del proyecto en el landing page. Mi objetivo
es mover esta sección en un futuro cuando avance más el proyecto. El desplegado 
lo he realizado en netlify y está disponible en [https://bukhgalter.netlify.app/](https://bukhgalter.netlify.app/).

![](images/deploy_ui.png)

### Criterios que he seguido

En primer lugar me he decantado por netlify porque creo que cuenta con muchas herramientas
interesantes para desplegar páginas como la que quería crear. Además la interfaz es muy completa
y me permite ver claramente lo que está pasando con el sitio web. El rendimiento que he experimentado
es muy bueno y no he notado que me haga falta nada que no tenga con netlify.

Respecto al lenguaje utilizado me he decantado por usar `node` por los siguientes motivos:

- Los SPA suelen usar todos js y con node puedo trabajar en ellos.
- Svelte es una utilidad hecha en js que proporciona herramientas para
  desarrollar usando node y después compilar el sitio web en un archivo `.js`.
- Netlify soporta de manera nativa `js`.

El desplegado lo he documentado en un archivo sobre [netlify](netlify.md)

He obtado usar este repositorio como un `monorepo` con la excepción de la
función serverless de `rust`. La interfaz se encuentra en la carpeta [ui](https://github.com/yabirgb/bukhgalter/tree/master/ui).

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

3. No he encontrado la manera correcta de evitar que se publique la ui en cada
   commit que hago. Finalmente me he decantado por usar un [github actions](https://github.com/yabirgb/bukhgalter/blob/master/.github/workflows/build_netlify.yml)
   que realiza una llamada a netlify e inicializa la construcción del sitio. De esta manera solo 
   se construye cuando detecta que hay cambios en la UI. 