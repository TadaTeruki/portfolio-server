use axum::response::IntoResponse;
use hyper::StatusCode;

pub async fn check_health() -> impl IntoResponse {
    (
        StatusCode::OK,
        "
                                         Peruki@future-university-hakodate
   .                                     ---------------------------------
  ##########==  =#                       Name: Teruki TADA
   ##===####==###==#==.                  Shell: Japanese
   ##==####==##        ===               Location: Hakodate, Japan
    ########=             #=             Affiliation: Future University Hakodate
     ####=            ##   .#.           Twitter: @PerukiFUN
      #         ##    ##     #           GitHub: TadaTeruki
      #         ##         .#=           Main Language: Go
      =#               ====              Supported Languages: C, C++, Go, Rust, JS/TS
        =====.    ====                   Fields: Landscape Evolution Model
       =#=#########.                           : Backend Web Development
     #######=######=                           : Community Management
        ===  #     =                     Website: https://portfolio.peruki.dev      
",
    )
}
