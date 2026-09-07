//! The body catalogue: real elements and physical properties loaded from `data/bodies.json`.

use crate::kepler::{Elements, Jd};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Index of a body within a [`Catalogue`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BodyId(pub usize);

/// Orbit class as reported by SBDB, or `Planet` for the two planets we add by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Kind {
    /// Main-belt asteroid.
    MainBelt,
    /// Near-Earth asteroid of any sub-class.
    NearEarth,
    /// Jupiter trojan.
    Trojan,
    /// Outer main belt / Hilda / Cybele region.
    Outer,
    /// A planet.
    Planet,
    /// Anything SBDB labels that we do not model specially.
    Other(String),
}

impl From<&str> for Kind {
    fn from(s: &str) -> Self {
        match s {
            "MBA" => Self::MainBelt,
            "AMO" | "APO" | "ATE" | "IEO" => Self::NearEarth,
            "TJN" => Self::Trojan,
            "OMB" | "HIL" | "CEN" => Self::Outer,
            "PLANET" => Self::Planet,
            other => Self::Other(other.to_owned()),
        }
    }
}

#[derive(Debug, Deserialize)]
struct RawBody {
    name: String,
    designation: String,
    kind: String,
    a_au: f64,
    e: f64,
    i_deg: f64,
    om_deg: f64,
    w_deg: f64,
    ma_deg: f64,
    epoch_jd: f64,
    diameter_km: Option<f64>,
    spec: Option<String>,
    rot_per_h: Option<f64>,
    gm_km3s2: Option<f64>,
    albedo: Option<f64>,
    h_mag: Option<f64>,
}

/// One body: its orbit and what is known of its physical nature.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body {
    /// Short name, e.g. `Ceres`.
    pub name: String,
    /// Full designation, e.g. `1 Ceres (A801 AA)`.
    pub designation: String,
    /// Orbit class.
    pub kind: Kind,
    /// Osculating elements.
    pub elements: Elements,
    /// Diameter in kilometres, when measured.
    pub diameter_km: Option<f64>,
    /// Spectral class (Bus-DeMeo or Tholen), when measured.
    pub spec: Option<String>,
    /// Rotation period in hours, when measured.
    pub rot_per_h: Option<f64>,
    /// Gravitational parameter in km³/s², when measured.
    pub gm_km3s2: Option<f64>,
    /// Geometric albedo, when measured.
    pub albedo: Option<f64>,
    /// Absolute magnitude.
    pub h_mag: Option<f64>,
}

impl Body {
    /// Mean radius in kilometres, if the diameter is known.
    #[must_use]
    pub fn radius_km(&self) -> Option<f64> {
        self.diameter_km.map(|d| d / 2.0)
    }
}

/// Errors from loading a catalogue.
#[derive(Debug, thiserror::Error)]
pub enum CatalogueError {
    /// The JSON could not be parsed.
    #[error("catalogue JSON: {0}")]
    Json(#[from] serde_json::Error),
    /// Two bodies share a name.
    #[error("duplicate body name {0}")]
    Duplicate(String),
}

/// The set of bodies the map knows about.
#[derive(Debug, Clone, Default)]
pub struct Catalogue {
    bodies: Vec<Body>,
    by_name: HashMap<String, BodyId>,
}

impl Catalogue {
    /// Parses the catalogue from the JSON text of `data/bodies.json`.
    pub fn from_json(text: &str) -> Result<Self, CatalogueError> {
        let raw: Vec<RawBody> = serde_json::from_str(text)?;
        let mut cat = Self::default();
        for RawBody {
            name,
            designation,
            kind,
            a_au,
            e,
            i_deg,
            om_deg,
            w_deg,
            ma_deg,
            epoch_jd,
            diameter_km,
            spec,
            rot_per_h,
            gm_km3s2,
            albedo,
            h_mag,
        } in raw
        {
            let elements =
                Elements::from_degrees(a_au, e, i_deg, om_deg, w_deg, ma_deg, Jd(epoch_jd));
            let body = Body {
                name,
                designation,
                kind: Kind::from(kind.as_str()),
                elements,
                diameter_km,
                spec,
                rot_per_h,
                gm_km3s2,
                albedo,
                h_mag,
            };
            cat.push(body)?;
        }
        Ok(cat)
    }

    /// Adds a body, returning its id.
    pub fn push(&mut self, body: Body) -> Result<BodyId, CatalogueError> {
        if self.by_name.contains_key(&body.name) {
            return Err(CatalogueError::Duplicate(body.name));
        }
        let id = BodyId(self.bodies.len());
        self.by_name.insert(body.name.clone(), id);
        self.bodies.push(body);
        Ok(id)
    }

    /// Looks a body up by short name.
    #[must_use]
    pub fn id(&self, name: &str) -> Option<BodyId> {
        self.by_name.get(name).copied()
    }

    /// The body with the given id.
    #[must_use]
    pub fn get(&self, id: BodyId) -> &Body {
        &self.bodies[id.0]
    }

    /// Iterates all bodies with their ids.
    pub fn iter(&self) -> impl Iterator<Item = (BodyId, &Body)> {
        self.bodies.iter().enumerate().map(|(i, b)| (BodyId(i), b))
    }

    /// Number of bodies.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.bodies.len()
    }

    /// Whether the catalogue is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.bodies.is_empty()
    }
}
