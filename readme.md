# EPA de Stack con Kani

Este proyecto implementa una stack en Rust, y luego con funciones que evalúan si una stack en particular puede
transicionar entre dos estados, utiliza la librería [kani-verifier](https://github.com/model-checking/kani) para
evaluar sobre (casi) cualquier stack aprovechando la técnica de model checking.

## Funciones a evaluar

Las funciones que muestran si hay transición o no son del tipo:

```
[kani::proof]
fn puedo_ir_de_A_a_B_con_accion() {
    let s = non_deterministic_stack();
    kani::assume(estoy_en_estado_A(s));
    s.accion(...);
    assert!(!estoy_en_estado_B(s)); // Falla si se llegó al estado esperado.
}
```

Si una aserción falla, podemos decir que se puede ir desde algun estado que pertenezca al grupo A, a algún estado que 
pertenezca al grupo B, mediante la 'acción', que puede ser cualquier operación del objeto estudiado.

Nota: si el grupo de estados permite una única operación, se omite el 'con_acción' dado que esta implícito en el estado.

## Ejecución

Si se ejecuta

```
cargo kani
```

luego de unos minutos salen las verificaciones fallidas. 

Las verificaciones fallidas simbolizan las transiciones posibles entre estados.

En este caso las verificaciones exitosas, es decir, las que muestran que no hay transición, son:

- `puedo_ir_push_a_push`
- `puedo_ir_pop_a_pop`
- `puedo_ir_pushpop_a_pop_con_pop`
- `puedo_ir_pushpop_a_push_con_push`

## Debugging

La herramienta kani provee una función *experimental* donde podemos obtener los valores concretos que hicieron fallar la aserción.

Para utilizarla, correr:
```
cargo kani -Z concrete-playback --concrete-playback=[print|inplace]
```

Esto escribe un test con un vector que contiene cada valor generado por `kani::any()` en un vector de 8 bytes, con un comentario
arriba de cada vector mostrando el valor efectivo.

La opción `inplace` escribirá el test en el archivo y la opción `print` lo hará en consola después de cada proof.

Más información sobre esta feature [aquí](https://model-checking.github.io/kani/reference/experimental/concrete-playback.html).