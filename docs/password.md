# Documentación del Generador de Contraseñas Personalizado

## Introducción

Este generador de contraseñas está diseñado para usuarios técnicos que desean crear contraseñas seguras y personalizadas basadas en perfiles definidos a través de archivos de configuración. La flexibilidad del sistema permite adaptar los requisitos de generación de contraseñas a las necesidades de seguridad de diversas plataformas, proporcionando control total sobre los tipos de caracteres, patrones y entropía.

## Estructura del Archivo de Configuración

El archivo de configuración utiliza el formato TOML y está compuesto por varias secciones opcionales, lo que permite que los usuarios definan solo los aspectos que consideran relevantes:

- `[properties]`: Define las propiedades generales del perfil.
- `[requirements]`: Establece los requisitos mínimos y máximos para tipos de caracteres y longitud.
- `[allowed]` y [not_allowed]: Conjuntos de caracteres permitidos y excluidos.
- `[rules]`: Especifica reglas adicionales, como patrones de generación y entropía mínima (opcional).
- `[idiomas]`: Definición de conjuntos de caracteres personalizados basados en el idioma especificado en  `[properties].[lang]`

## Ejemplo Básico de Configuración:

```toml
[properties]
version = "0.1.0"                  # Versión del formato del archivo
lang = ["es"]                       # Idiomas soportados (ISO 639-1)
name = "Perfil Seguro Corporativo"  # Nombre descriptivo del perfil
type = "password"                   # Tipo de generación (password, passphrase)

[requirements]
digits = {min = 2, max = 6}         # Entre 2 y 6 dígitos
lowercase = {min = 2, max = inf}    # Al menos 2 letras minúsculas, sin límite máximo
uppercase = {min = 2, max = inf}    # Al menos 2 letras mayúsculas, sin límite máximo
symbols = {min = 1, max = 4}        # Entre 1 y 4 símbolos
length = {min = 10, max = 64}       # Longitud total entre 10 y 64 caracteres

[allowed]
include = ["U+30A2"]                # Caracteres permitidos (ejemplo: Katakana Letter A)

[not_allowed]
exclude = ["U+1F600-U+1F64F"]       # Caracteres excluidos (ejemplo: Emoticonos)

[es]
uppercase = ["U+0041-U+005A", "U+00D1"]  # A-Z y Ñ
lowercase = ["U+0061-U+007A", "U+00F1"]  # a-z y ñ
digits = ["U+0030-U+0039"]               # 0-9
symbols = ["U+0021-U+002F", "U+00A1"]    # !"#$%&'()*+,-./ y ¡

[rules]
max-consecutive = 2            # Máximo de caracteres consecutivos iguales
min-entropy-bits = 24          # Entropía mínima requerida (bits)
pattern = "(lowercase){3}*"    # Patrón opcional
```

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

Especifica los requisitos mínimos y máximos para los tipos de caracteres y la longitud total de la contraseña. Los nombres como `lowercase`, `uppercase`, `digits`, etc., son ejemplos de conjuntos que el usuario define en la sección correspondiente y no deben interpretarse de manera literal.

El campo `length` es obligatorio, ya que define la longitud mínima, máxima o literal de la contraseña.

Además de los rangos `{min, max},` también puedes usar números literales (enteros positivos) para definir cantidades fijas.

```toml
[requirements]
length = {min = 10, max = 64}        # Longitud total entre 10 y 64 caracteres (obligatorio)
custom_set_1 = {min = 2, max = inf}  # Definido por el usuario (ejemplo: letras minúsculas)
custom_set_2 = {min = 2, max = inf}  # Definido por el usuario (ejemplo: letras mayúsculas)
custom_set_3 = 3                     # Definido por el usuario, número literal (ejemplo: exactamente 3 símbolos)
```

**Restricciones:**
- No se puede usar `length` como nombre de conjunto. Esto se debe a que length tiene un propósito específico en el sistema y es necesario para definir la longitud total de la contraseña.
- No se pueden repetir nombres de conjuntos. Cada conjunto debe tener un nombre único para evitar conflictos en la generación de contraseñas.

## Secciones `[allowed]` y `[not_allowed]`

Define caracteres adicionales permitidos o excluidos utilizando códigos Unicode en formato U+XXXX. Se permiten rangos de caracteres.

```toml
[allowed]
include = ["U+30A2", "U+02DC"]    # Caracteres permitidos (ejemplo: Katakana Letter A y virgulilla)

[not_allowed]
exclude = ["U+1F600-U+1F64F"]     # Caracteres excluidos (ejemplo: Emoticonos)
```

> **NOTA:** Los caracteres en la lista de `not_allowed` tienen prioridad y serán excluidos, incluso si también están presentes en la lista `allowed`.

## Sección `[idioma]`

El identificador de idioma, corresponde a los códigos de idioma ISO 639-1 que se especifican en la sección `[properties].[lang]`. Esto permite que el generador de contraseñas maneje múltiples configuraciones de caracteres para diferentes idiomas, lo que es especialmente útil en entornos multilingües.

Por ejemplo, si en la sección `[properties]` defines `lang = ["es", "en"]`, podrías tener configuraciones específicas de caracteres para el español `[es]` y para el inglés `[en]`. Esto te permite aplicar reglas y caracteres únicos para cada idioma.

También puedes definir los conjuntos de caracteres personalizados que el usuario utilizará en los patrones o requisitos definidos en `[requirements]`. Los nombres de los conjuntos son completamente personalizables y seleccionados por el usuario, de manera que pueden reflejar las necesidades específicas de cada perfil.

Por ejemplo, puedes crear conjuntos como `letras_mayusculas`, `numeros`, o cualquier otro nombre que tenga sentido para tu configuración.

### **Ejemplo:**

```toml
[es]
letras_mayusculas = [
  "U+0041-U+005A",  # A-Z
  "U+00D1",         # Ñ
  "U+00C1",         # Á
  "U+00C9",         # É
  "U+00CD",         # Í
  "U+00D3",         # Ó
  "U+00DA",         # Ú
  "U+00DC"          # Ü
]

[en]
uppercase_letters = [
  "U+0041-U+005A"   # A-Z
]

lowercase_letters = [
  "U+0061-U+007A"   # a-z
]
```

> **NOTA:** Los nombres de los conjuntos (`letras_mayusculas`, `uppercase_letters`) son personalizados y seleccionados por el usuario. Estos se usarán posteriormente en las secciones `[requirements]` o `[rules]` para referirse a los tipos de caracteres que se utilizarán en la generación de contraseñas según el idioma y por ahora no deben repetirse entre idiomas.

## Sección [rules]

Aquí es donde se especifican las reglas y patrones para la generación de contraseñas. Estas reglas son opcionales, y puedes usarlas para definir patrones estructurados de contraseñas, limitar caracteres consecutivos o establecer una entropía mínima.


### Sintaxis de Patrones

La sintaxis de los patrones te permite definir la estructura exacta de la contraseña usando bloques, cantidades y control sobre los caracteres que no deberían estar presentes.

- **Bloques**: Se definen entre paréntesis `()`.
- **Conjuntos**: El nombre del conjunto de caracteres que será utilizado dentro del bloque. Este conjunto debe haber sido definido previamente en la sección de idioma (por ejemplo, `[es]`).
- **Cantidad**: Especificada entre llaves `{}` después del bloque.
  - `{min,max}`: Mínimo y máximo de caracteres consecutivos del conjunto.
  - `{min,}`: Mínimo de caracteres consecutivos sin límite máximo.
  - `{num}`: Número exacto de caracteres consecutivos.
- **Negación**: Usar `^` antes del nombre del conjunto para negar (excluir) ese conjunto.
- **Carácter de control**: Usar `*` para indicar que el resto de la contraseña puede ser cualquier carácter permitido, sin restricciones adicionales.

Ejemplos de Patrones
Contraseña que comienza con una mayúscula y el resto es aleatorio:

Ejemplos de Patrones

1. **Contraseña que comienza con una mayúscula y el resto es aleatorio:**

    ```toml
    [rules]
    pattern = "(uppercase){1}*"
    ```
2. **Contraseña que debe tener 3 minúsculas seguidas, luego cualquier carácter excepto dígitos:**
    ```toml
    [rules]
    pattern = "(lowercase){3}(^digits)*"
    ```
3. **Contraseña que empieza con 2 dígitos, seguido de caracteres que no sean símbolos en las siguientes posiciones:**
    ```toml
    [rules]
    pattern = "(digits){2}(^symbols){3}*"
    ```

Reglas Adicionales

- Negación de conjuntos: `(^{conjunto}){n}` indica que en ese bloque no deben aparecer caracteres del conjunto especificado.
- Carácter de control `*`: Indica que el resto de la contraseña puede contener cualquier carácter permitido según los conjuntos definidos y las restricciones generales.

Reglas de Seguridad Adicionales

```toml
[rules]
max-consecutive = 2       # Máximo de caracteres consecutivos iguales
min-entropy-bits = 24     # Entropía mínima requerida
```

> **NOTA:** El campo pattern es opcional, y si no se especifica, la contraseña se generará de manera aleatoria respetando los requisitos definidos en [requirements].
