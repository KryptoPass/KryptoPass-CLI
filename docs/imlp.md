## 2. Validación de Compatibilidad entre Secciones

**Objetivo:**  
Asegurar que las configuraciones son coherentes y no hay conflictos entre ellas.

### Acciones:

1. **Validar que `pattern` es compatible con `requirements` y `length`:**
   - [x] Analizar el `pattern`: Parsear el patrón para extraer los bloques, conjuntos y cantidades.
   - [x] Verificar que los conjuntos utilizados en el `pattern` existen en las secciones de idiomas correspondientes.
   - Calcular la longitud mínima y máxima posible del `pattern`:  
     - Sumar las cantidades mínimas y máximas de cada bloque.
   - Comparar con `requirements.length`:  
     - Asegurar que la longitud del `pattern` está dentro de los límites definidos.

2. **Validar que `requirements` es compatible con `length`:**
   - Sumar los mínimos de cada requisito y asegurarse de que no exceden `length.max`.
   - Sumar los máximos de cada requisito y asegurarse de que al menos alcanzan `length.min`.

3. **Verificar que no hay conflictos en los nombres de los conjuntos:**
   - Asegurar que los nombres de conjuntos no se repiten y que no se utilizan palabras reservadas como `length`.

4. **Aplicar `not_allowed` a los conjuntos:**
   - Remover de los conjuntos de caracteres cualquier carácter o rango especificado en `[not_allowed]`.
   - Asegurar que los conjuntos no quedan vacíos:  
     - Después de aplicar `not_allowed`, verificar que cada conjunto aún contiene caracteres.

5. **Acciones en caso de error:**  
   Si se detecta algún conflicto o error de validación, abortar el proceso y notificar al usuario.
