extern crate openssl;
extern crate postgres;

use postgres::{Connection, TlsMode};

use openssl::ssl::{SslConnectorBuilder, SslMethod, SslVerifyMode};
use openssl::x509;

fn main() {
    let mut connector = SslConnectorBuilder::new(SslMethod::tls()).unwrap();
    connector.set_ca_file("root.crt").unwrap();
    connector
        .set_certificate_file("postgresql.crt", x509::X509_FILETYPE_PEM)
        .unwrap();

    connector
        .set_private_key_file("postgresql.key", x509::X509_FILETYPE_PEM)
        .unwrap();

    // openssl::ssl::SslVerfifyMode constant in not defined yet in openssl 0.9.23 which is rust-postgres dependency
    // disable certificate hostname check
    let mode = SslVerifyMode::empty();
    connector.set_verify(mode);

    let negotiator = postgres::tls::openssl::OpenSsl::from(connector.build());

    let conn = Connection::connect(
        "postgres://firefox@localhost:5432",
        TlsMode::Prefer(&negotiator),
    ).unwrap();
    let res = conn.query("SELECT 1+1 as foo", &[]).unwrap();
    for row in &res {
        let foo: i32 = row.get(0);
        println!("{}", foo);
    }
}
