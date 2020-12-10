# Diseño de la API

Para interactura con la API se utilizarán llamadas `HTTP`
y la información se codificará como `JSON`. 

En esta documentación se especifica el proposito de cada `endpoint` y, de ser
necesario, los campos que acepta en la petición y el tipo de los mismos.

## Objetos comunes

A continuación se listan algunos objetos que se usan en las peticiones y que se
repiten a lo largo de la aplicación. 

### Debtor

Representa a un deudor en una cuenta

    {
        "name" : String (required),
        "paid_amount": Float (required),
        "fraction": Float (required),
        "paid": Bool (required),
    }

### Item

Objeto que se asigna a una cuenta

    {
        
        "date": u32 (required),
        "price": Float (required),
        "name": String (required)

    }

### Account

Cuenta que representa una asociación entre `items` y `debtors`

    {
        "name": String (required),
        "debtors": [Debtor] (required),
        "items": [Item] (required)
    }

## Endpoints


1. Se necesitan crear eventos. 

        POST /api/v1/events/create


   Toma:
        
   - Entrada JSON de `Account`

   Devuelve

   - `OK`: Devuelve los datos de la entrada creada
   - `BAD_REQUEST`: Si la entrada no es válida

1. Se necesita listar la información de un evento

        GET /api/v1/events/{event_id}

   Toma:
        
   - Argumento en url del evento del que se quiere solicitar información

   Devuelve

   - `OK`: Devuelve los datos del evento
   - `NOT_FOUND`: Si el evento no se encuentra en el sistema

1. Debe poder añadirse un deudor a una lista

        PUT /api/v1/events/{event_id}

   Toma:
        
   - Entrada JSON de `Account`

   Devuelve

   - `OK`: Devuelve los datos de la entrada actualizada
   - `BAD_REQUEST`: Si la entrada no es válida

1. Hacer un pago

        PATH /api/v1/events/pay

   Toma:
        
   Entrada JSON con
    - `name` (required)(`String`): Nombre de la persona que realiza el pago.
    - `amount` (required)(`Float`): Cantidad que se paga.
    - `id` (required)(`String`): Id de la cuenta en la que se hace el pago.

   Devuelve

   - `OK`: Devuelve los datos de la entrada actualizada
   - `NOT_FOUND`: Si la entrada no hay un evento con dicho id


1. Un usuario debe poder conseguir todos los eventos en los que participa

        GET /api/v1/users/{user_id}

   Toma:
        
   - Argumento en url del evento del usuario que se quiere consultar

   Devuelve

   - `OK`: Devuelve los datos del evento o una lista vacía si no tiene o no
     existe el usuario


Con las funcionalidades anteriormente descritas se cubren las historias de
usuario a nivel de API:

- HU1: Añadir y leer eventos asociados a un usuario (1, 5)
- HU2: Agregar y eliminar deudores (3)
- HU3: Realizar pagos (4)
- HU4: Actualizar proporciones de deudas (3)
- HU5: Conocer datos de una cuenta (2)