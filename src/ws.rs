use std::collections::HashMap;
use minreq::Response;
use serde::{Deserialize, Serialize};
use crate::Resultado;

pub const MSG_ERROR_CNX: &str = "Error de conexión (Es)";

#[derive(Deserialize)]
pub struct WSRespuesta<R>  {
	ok: bool,
	error: String,
	contenido: R
}

pub fn peticion_post_json<C: Serialize, R: for<'a> Deserialize<'a>>(url: &str, metodo: &str, cabeza: HashMap<String, String>, cuerpo: C, timeout: u64) -> Resultado<R> {

	//let ttt = Local::now();

	let url = format!("{}/{}", url, metodo);

	//#[cfg(debug_assertions)]
	//println!("Petición: POST {}", url);
	let cuerpo_bin = serde_json::to_vec(&cuerpo).map_err(|e|format!("{}", e))?;

	let mut ws = minreq::post(url)
		.with_timeout(timeout)
		.with_header("Content-Type", "application/json; charset=UTF-8");

	for (k, v) in cabeza {
		//#[cfg(debug_assertions)]
		//println!("          {k}: {v}");
		ws = ws.with_header(k, v);
	}

	//#[cfg(debug_assertions)]
	//println!("          {}", String::from_utf8(cuerpo_bin.clone()).unwrap());

	let resp = ws
		.with_body(cuerpo_bin)
		.send()
		.map_err(|e|
			if let minreq::Error::IoError(_) = e {
				MSG_ERROR_CNX.to_string()
			} else {
				format!(r#"Error en "{}": {}"#, metodo, e)
			}
		)?;

	let raw_resp = resp.as_str()
		.map_err(|e| format!(r#"Respuesta "{}" no esta en UTF-8, {}"#, metodo, e))?;

	//#[cfg(debug_assertions)]
	//println!("Respuesta HTTP {}: {} bytes : {}", resp.status_code, raw_resp.len(), raw_resp);
	//println!("Respuesta Tiempo de Descarga: {}", (Local::now() - ttt).s_float());

	if resp.status_code < 200 || resp.status_code > 299 {

		let resp: WSRespuesta<String> = serde_json::from_str(raw_resp)
			.map_err( |e|
				format!(r#"Respuesta HTTP {} {} en "{}""#, resp.status_code, resp.reason_phrase, metodo)
			)?;

		Err(resp.error)?;
	}

	//let ttt = Local::now();

	let resp: WSRespuesta<R> = serde_json::from_str(raw_resp)
		.map_err( |e| {

			//#[cfg(debug_assertions)]
			//escribir_archivo_carpeta_local_forte(NOM_CARP_ERROR, &format!("ws_raw_{}.json", Local::now().format("%Y%m%d_%H%M%S_%3f")), raw_resp);

			format!(r#"Formato del cuerpo WS recibido incorrecto en "{}", {}"#, metodo, e)
		})?;

	//println!("Respuesta Tiempo de Deserialización: {}", (Local::now() - ttt).s_float());

	Ok(resp.contenido)
}

pub fn peticion_raw_get(url: &str, timeout: u64) -> Resultado<Response> {
	let resp = minreq::get(url)
		.with_timeout(timeout)
		.send()
		.map_err(|e| format!(r#"Error en "{}": {}"#, url, e))?;

	if resp.status_code < 200 || resp.status_code > 299 {
		Err(format!(r#"Respuesta HTTP {} {} en "{}""#, resp.status_code, resp.reason_phrase, url))?;
	}

	Ok(resp)
}