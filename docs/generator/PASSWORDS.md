# Documentación del Generador de Contraseñas de KryptoPass

## Introducción

El generador de contraseñas está diseñado para proporcionar flexibilidad y seguridad en la generación de contraseñas, a la vez que simplifica la administración interna de los requisitos, validaciones de rangos, y patrones. Esta documentación explica el diseño del sistema y cómo abordar las validaciones de longitud, rangos y patrones de forma cohesiva. También funciona como una especificación del uso y funcionamiento del generador.

## Diseño General

El generador maneja los siguientes aspectos:

1. **Rangos y longitud de la contraseña**: Los valores mínimos y máximos para cada tipo de carácter deben ser consistentes con la longitud total de la contraseña.
2. **Integración de patrones**: Los patrones deben respetar los requisitos de rangos y proveer una estructura opcional para la generación de contraseñas.
3. **Validación y generación**: Se valida que los requisitos sean coherentes antes de proceder con la generación de la contraseña.

Se decidió utilizar [TOML](https://toml.io/en/) como formato de archivo de configuración para el generador de contraseñas; ya que es fácil de leer y escribir, y permite una estructura clara y concisa. 

El archivo de configuración está compuesto por varias secciones (algunas opcionales), lo que permite que los usuarios definan solo los aspectos que consideran relevantes:

- `[properties]`: Define las propiedades generales del perfil de generación.
- `[requirements]`: Establece los requisitos mínimos y máximos para tipos de caracteres y longitud.
- `[allowed]` y [not_allowed]: Conjuntos de caracteres permitidos y excluidos.
- `[rules]`: Especifica reglas adicionales y/o avanzadas diseñadas para brindar inteligencia y seguridad a la generación de contraseñas.
- `[custom]`: Definición de conjuntos de caracteres personalizados basados en los idiomas especificados en `[properties].[lang]`.

## Flujo del Generador

### Paso 1: Leer y procesar la configuración

El archivo de configuración define los requisitos de tipos de caracteres, rangos mínimos y máximos, así como un patrón opcional para la generación de contraseñas. El patrón, si está presente, debe ser compatible con los requisitos.

### Paso 2: Validar los requisitos y el patrón

Antes de generar una contraseña, se valida lo siguiente:

- La suma de los valores mínimos de cada tipo de carácter debe ser menor o igual a la longitud máxima de la contraseña.
- La suma de los valores máximos de cada tipo de carácter debe ser mayor o igual a la longitud mínima de la contraseña.
- Si hay un patrón, se asegura que no contradiga los rangos definidos.

### Paso 3: Generación de la contraseña

Dependiendo de si existe un patrón, el proceso de generación sigue dos rutas:

1. **Generación con patrón**: La contraseña sigue la estructura definida por el patrón, completando cualquier carácter restante de manera aleatoria si es necesario.
2. **Generación aleatoria**: Si no hay un patrón, la contraseña se genera de forma aleatoria respetando los rangos definidos.

### Paso 4: Validar la contraseña generada

Una vez generada, la contraseña es validada para asegurar que cumple con los requisitos de longitud, cantidad de tipos de caracteres, y reglas adicionales (por ejemplo, entropía mínima).

## Sección `[properties]`
Define las propiedades generales del perfil, incluyendo el nombre, idioma y tipo de generación. Todos los campos en esta sección son obligatorios.

```toml
[properties]
version = "0.1.0"                   # Versión del archivo de configuración
lang = ["es"]                       # Idiomas soportados (ISO 639-1)
name = "Perfil Seguro Corporativo"  # Nombre descriptivo del perfil
type = "password"                   # Tipo de generación (password, passphrase)
```

## Sección `[requirements]`

Define los requisitos de tipos de caracteres, rangos mínimos y máximos, así como un las reglas que el patrón opcional debe seguir para la generación de contraseñas. El campo `length` es obligatorio, ya que define la longitud mínima, máxima o literal de la contraseña. Además de los rangos `{min, max},` también puedes usar números literales (enteros positivos) para definir cantidades fijas.

```toml
[requirements]
length = {min = 10, max = 64}     # Longitud total entre 10 y 64 caracteres (obligatorio)
lowercase = {min = 2, max = inf}  # Definido por el usuario (ejemplo: letras minúsculas)
uppercase = {min = 2, max = inf}  # Definido por el usuario (ejemplo: letras mayúsculas)
custom_set_3 = 3                  # Definido por el usuario, número literal (ejemplo: exactamente 3 caracteres del conjunto)
```

> **Nota**: En los ejemplos puede llevar a ver los nombres como *lowercase, uppercase, digits*, etc., Estos son ejemplos de conjuntos que el usuario define en la sección correspondiente y **no deben interpretarse de manera literal**.

### Validación de Rangos y Longitud

La validación de rangos y longitud se realiza asegurando que:

1. La suma de los valores mínimos de cada tipo de carácter no exceda la longitud máxima.
2. La suma de los valores máximos de cada tipo de carácter no sea menor a la longitud mínima.

### Ejemplo de validación:

```toml
[requirements]
digits = {min = 2, max = 6}
lowercase = {min = 2, max = inf}
uppercase = {min = 2, max = inf}
symbols = {min = 1, max = 4}
length = {min = 10, max = 64}
```

Validación:

- **Suma de mínimos**: 2 (digits) + 2 (lowercase) + 2 (uppercase) + 1 (symbols) = 7
- **Suma de máximos**: 6 (digits) + inf (lowercase) + inf (uppercase) + 4 (symbols) = inf

La suma de mínimos (7) es menor o igual a la longitud máxima (64), y la suma de máximos (inf) es mayor o igual a la longitud mínima (10), por lo tanto los requisitos son válidos.

### Reglas para los rangos infinitos:

- Si el valor `max` es `inf`, se considera que el límite real es la longitud máxima de la contraseña.
- Si `max` excede la longitud máxima de la contraseña, se ajusta automáticamente al valor de la longitud máxima.

---

**Restricciones:**
- No se puede usar `length` como nombre de conjunto. Esto se debe a que length tiene un propósito específico en el sistema y es necesario para definir la longitud total de la contraseña.
- No se pueden repetir nombres de conjuntos. Cada conjunto debe tener un nombre único para evitar conflictos en la generación de contraseñas.