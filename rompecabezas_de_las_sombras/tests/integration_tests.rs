use rompecabezas_de_las_sombras::program::{
    calcular_intervalos, calcular_longitud_cubierta, extraer_datos_flatlander,
    extraer_datos_iniciales,
};

#[test]
fn caso_1() {
    let datos_iniciales = extraer_datos_iniciales("45 2").unwrap();
    let flatlanders = vec![
        extraer_datos_flatlander("0 10").unwrap(),
        extraer_datos_flatlander("5 10").unwrap(),
    ];
    let intervalos = calcular_intervalos(&flatlanders, datos_iniciales.0);
    let longitud_cubierta = calcular_longitud_cubierta(&intervalos);

    assert!((longitud_cubierta - 15.0000000000000).abs() < 1e-9);
}

#[test]
fn caso_2() {
    let datos_iniciales = extraer_datos_iniciales("45 3").unwrap();
    let flatlanders = vec![
        extraer_datos_flatlander("50 150").unwrap(),
        extraer_datos_flatlander("0 100").unwrap(),
        extraer_datos_flatlander("100 200").unwrap(),
    ];
    let intervalos = calcular_intervalos(&flatlanders, datos_iniciales.0);
    let longitud_cubierta = calcular_longitud_cubierta(&intervalos);

    assert!((longitud_cubierta - 300.00000000000006).abs() < 1e-9);
}

#[test]
fn caso_3() {
    let datos_iniciales = extraer_datos_iniciales("30 3").unwrap();
    let flatlanders = vec![
        extraer_datos_flatlander("50 150").unwrap(),
        extraer_datos_flatlander("0 100").unwrap(),
        extraer_datos_flatlander("100 200").unwrap(),
    ];
    let intervalos = calcular_intervalos(&flatlanders, datos_iniciales.0);
    let longitud_cubierta = calcular_longitud_cubierta(&intervalos);

    assert!((longitud_cubierta - 446.4101615137755).abs() < 1e-9);
}

#[test]
fn caso_minimo_un_flatlander() {
    let datos_iniciales = extraer_datos_iniciales("60 1").unwrap();
    let flatlanders = vec![extraer_datos_flatlander("0 100").unwrap()];
    let intervalos = calcular_intervalos(&flatlanders, datos_iniciales.0);
    let longitud_cubierta = calcular_longitud_cubierta(&intervalos);

    assert!(longitud_cubierta > 0.0);
}

#[test]
fn caso_intervalos_solapados_completamente() {
    let datos_iniciales = extraer_datos_iniciales("45 2").unwrap();
    let flatlanders = vec![
        extraer_datos_flatlander("0 100").unwrap(),
        extraer_datos_flatlander("0 50").unwrap(),
    ];
    let intervalos = calcular_intervalos(&flatlanders, datos_iniciales.0);
    let longitud_cubierta = calcular_longitud_cubierta(&intervalos);

    assert!((longitud_cubierta - 100.0).abs() < 1e-9);
}

#[test]
fn caso_angulo_bajo() {
    let datos_iniciales = extraer_datos_iniciales("10 2").unwrap();
    let flatlanders = vec![
        extraer_datos_flatlander("0 50").unwrap(),
        extraer_datos_flatlander("100 100").unwrap(),
    ];
    let intervalos = calcular_intervalos(&flatlanders, datos_iniciales.0);
    let longitud_cubierta = calcular_longitud_cubierta(&intervalos);

    assert!(longitud_cubierta > 500.0);
}

#[test]
fn caso_angulo_alto() {
    let datos_iniciales = extraer_datos_iniciales("80 2").unwrap();
    let flatlanders = vec![
        extraer_datos_flatlander("0 50").unwrap(),
        extraer_datos_flatlander("10 100").unwrap(),
    ];
    let intervalos = calcular_intervalos(&flatlanders, datos_iniciales.0);
    let longitud_cubierta = calcular_longitud_cubierta(&intervalos);

    assert!(longitud_cubierta < 200.0);
}
