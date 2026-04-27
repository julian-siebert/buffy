use license::License;
use serde::Serialize;

use crate::error::{Error, Result};

#[derive(Debug, Clone, Serialize)]
pub struct ResolvedLicense {
    pub id: String,
    pub name: String,
    pub url: String,
    pub text: String,
}

pub fn resolve_licenses(spdx_expr: &str) -> Result<Vec<ResolvedLicense>> {
    let expr = spdx::Expression::parse(spdx_expr).map_err(|_| Error::SpdxParse {
        expr: spdx_expr.into(),
    })?;

    let mut out = Vec::new();

    for req in expr.requirements() {
        let id = match &req.req.license {
            spdx::LicenseItem::Spdx { id, .. } => id.name, // z.B. "MIT"
            spdx::LicenseItem::Other(lic_ref) => {
                return Err(Error::Internal(format!(
                    "Custom LicenseRef '{lic_ref}' is not supported – \
                             use a standard SPDX identifier"
                )));
            }
        };

        let lic: &dyn License = id
            .parse()
            .map_err(|_| Error::SpdxParse { expr: id.into() })?;

        out.push(ResolvedLicense {
            id: id.to_string(),
            name: lic.name().to_string(),
            url: format!("https://spdx.org/licenses/{id}.html"),
            text: lic.text().to_string(),
        });
    }

    Ok(out)
}
