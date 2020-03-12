extern crate plotly;

use actix_files::NamedFile;
use actix_web::{web, HttpResponse};
#[allow(unused_imports)]
use plotly::common::{Line, LineShape, Marker, Mode};
#[allow(unused_imports)]
use plotly::{Histogram, NamedColor, Plot, Scatter};
use serde::Deserialize;
use serde_json::json;
use slog::info;
use std::path::Path;

use crate::api_error::ApiError;
use crate::AppState;

use std::collections::HashMap;

#[derive(Deserialize)]
struct Coordinates {
  x: Vec<f64>,
  y: Vec<f64>,
}

type Data = HashMap<String, Coordinates>;

fn line_and_scatter_plot(
  context: web::Data<AppState>,
  data: web::Json<Data>,
) -> Result<String, ApiError> {
  let mut plot = Plot::new();

  for (h, coordinates) in data.iter() {
    info!(context.log, "generating plot for {}", h);
    let x = coordinates.x.clone();
    let y = coordinates.y.clone();
    
    let trace = Scatter::new(x, y)
      .mode(Mode::LinesMarkers)
      .name(h)
      .show_legend(true)
      .line(Line::new().shape(LineShape::Hv));
    plot.add_trace(trace);
  }

  let filename = String::from("testfile.html");
  let res = filename.clone();
  plot.to_html(filename);
  Ok(res)
}

async fn generate_plot(
  context: web::Data<AppState>,
  data: web::Json<Data>,
) -> Result<NamedFile, ApiError> {
  info!(context.log, "generating plot...");
  let filename = line_and_scatter_plot(context, data).unwrap();
  let path = Path::new(&filename);
  Ok(NamedFile::open(path).unwrap())
}

async fn index() -> Result<HttpResponse, ApiError> {
  Ok(
    HttpResponse::Ok().json(json!({ "message": "Welcome to bitgodine_plotter v 0.1".to_string() })),
  )
}

pub fn routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/plot")
      .route("", web::post().to(generate_plot))
      .route("", web::get().to(index)),
  );
}
