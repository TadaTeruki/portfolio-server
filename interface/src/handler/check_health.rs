use axum::response::IntoResponse;
use hyper::StatusCode;

pub async fn check_health() -> impl IntoResponse {
    (
        StatusCode::OK,
        "
    .                                     Peruki@future-university-hakodate
   ##########==  =#                       ---------------------------------
    ##===####==###==#==.                  Name: Teruki TADA
    ##==####==##        ===               Local Name: 多田 瑛貴
     ########=             #=             Location: Hakodate, Japan
      ####=            ##   .#.           Affiliation: School of System Information Science,
       #         ##    ##     #                        Future University Hakodate
       #         ##         .#=           Twitter: @PerukiFUN
       =#               ====              GitHub: TadaTeruki
         =====.    ====                   Major Programming Language: Go
        =#=#########.                     Programming Languages: C, C++, Go, (Rust)
      #######=######=                     Major Fields: Procedural Generation (CG)
         ===  #     =                                   Backend Web Development
                                          Server: OK
",
    )
}
