# Diseño de la API

Para interactura con la API se utilizarán llamadas `HTTP`
y la información se codificará como `JSON`. 

En esta documentación se especifica el proposito de cada `endpoint` y, de ser
necesario, los campos que acepta en la petición y el tipo de los mismos.

## Eventos


1. Se necesitan crear eventos. 

        POST /api/v1/events/create


    - `name` (required)(`String`): Nombre del evento que se crea
    - `debtors` (required)(`List[String]`): Lista de deudores. Al menos el usuario que la crea debe aparecer en ella.
    - `total_debt` (`Float`)(`default=0.0`): Deuda total asociada al evento.

1. Se necesita listar la información de un evento

        GET /api/v1/events/{event_id}

1. Debe poder añadirse un deudor a una lista

        PUT /api/v1/events/{event_id}

    - `name` (`String`): Nombre del evento que se crea
    - `debtors` (`List[String]`): Lista de deudores. Al menos el usuario que la crea debe aparecer en ella.
    - `total_debt` (`Float`): Deuda total asociada al evento.
    - `debt_proportions` (`List[Float]`): Proporciones de deuda asignada a cada usuario.
    Condiciones: Las proporciones TIENEN que sumar 1.

1. Hacer un pago

        POST /api/v1/events/{event_id}/pay

    - `name` (required)(`String`): Nombre de la persona que realiza el pago
    - `amount` (required)(`Float`): Cantidad que se paga.

1. Conocer estadísticas sobre un evento:

        GET /api/v1/events/{event_id}/stats

## Organizaciones

