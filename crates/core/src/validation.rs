use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::dto::{
    CreateStoreRequest, CreateUserRequest, LoginRequest, UpdateStoreRequest, UpdateUserRequest,
};

/// Violación individual de validación asociada a un campo.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldViolation {
    pub field: String,
    pub message: String,
}

/// Error agregado de validación para payloads de entrada.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("Error de validación: {message}")]
pub struct ValidationErrors {
    pub message: String,
    pub violations: Vec<FieldViolation>,
}

impl ValidationErrors {
    pub fn new(violations: Vec<FieldViolation>) -> Self {
        let message = violations
            .iter()
            .map(|v| format!("{}: {}", v.field, v.message))
            .collect::<Vec<_>>()
            .join("; ");
        Self {
            message,
            violations,
        }
    }

    pub fn from_single(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(vec![FieldViolation {
            field: field.into(),
            message: message.into(),
        }])
    }
}

/// Verifica que un texto no sea vacío tras eliminar espacios.
pub fn validate_non_empty(field: &str, value: &str) -> Result<(), FieldViolation> {
    if value.trim().is_empty() {
        return Err(FieldViolation {
            field: field.to_string(),
            message: "no puede estar vacío".to_string(),
        });
    }
    Ok(())
}

/// Valida formato básico de correo electrónico.
pub fn validate_email(email: &str) -> Result<(), FieldViolation> {
    let trimmed = email.trim();
    if trimmed.is_empty() {
        return Err(FieldViolation {
            field: "email".to_string(),
            message: "el correo electrónico es obligatorio".to_string(),
        });
    }
    if trimmed.contains(char::is_whitespace) {
        return Err(FieldViolation {
            field: "email".to_string(),
            message: "el correo electrónico no puede contener espacios".to_string(),
        });
    }
    let parts: Vec<&str> = trimmed.split('@').collect();
    if parts.len() != 2 || parts[0].is_empty() || !parts[1].contains('.') {
        return Err(FieldViolation {
            field: "email".to_string(),
            message: "formato de correo electrónico inválido".to_string(),
        });
    }
    let domain = parts[1];
    if domain.starts_with('.') || domain.ends_with('.') || domain.contains("..") {
        return Err(FieldViolation {
            field: "email".to_string(),
            message: "dominio de correo electrónico inválido".to_string(),
        });
    }
    Ok(())
}

/// Valida formato básico de teléfono (no vacío, mínimo 6 dígitos/caracteres válidos).
pub fn validate_phone(phone: &str) -> Result<(), FieldViolation> {
    let trimmed = phone.trim();
    if trimmed.is_empty() {
        return Err(FieldViolation {
            field: "phone".to_string(),
            message: "el teléfono es obligatorio".to_string(),
        });
    }
    let digit_count = trimmed.chars().filter(|c| c.is_ascii_digit()).count();
    let valid_chars = trimmed
        .chars()
        .all(|c| c.is_ascii_digit() || matches!(c, '+' | '-' | ' ' | '(' | ')'));
    if !valid_chars || digit_count < 6 {
        return Err(FieldViolation {
            field: "phone".to_string(),
            message: "formato de teléfono inválido (mínimo 6 dígitos)".to_string(),
        });
    }
    Ok(())
}

/// Valida longitud mínima de contraseña.
pub fn validate_password(password: &str) -> Result<(), FieldViolation> {
    if password.len() < 8 {
        return Err(FieldViolation {
            field: "password".to_string(),
            message: "la contraseña debe tener al menos 8 caracteres".to_string(),
        });
    }
    Ok(())
}

/// Valida coordenadas geográficas WGS84 (`latitude` en [-90, 90], `longitude` en [-180, 180]).
pub fn validate_coordinates(lat: f64, lng: f64) -> Result<(), FieldViolation> {
    if !lat.is_finite() || !(-90.0..=90.0).contains(&lat) {
        return Err(FieldViolation {
            field: "latitude".to_string(),
            message: "la latitud debe estar entre -90 y 90".to_string(),
        });
    }
    if !lng.is_finite() || !(-180.0..=180.0).contains(&lng) {
        return Err(FieldViolation {
            field: "longitude".to_string(),
            message: "la longitud debe estar entre -180 y 180".to_string(),
        });
    }
    Ok(())
}

/// Valida que un importe o cantidad sea finito y no negativo (`>= 0`).
pub fn validate_non_negative(field: &str, value: f64) -> Result<(), FieldViolation> {
    if !value.is_finite() || value < 0.0 {
        return Err(FieldViolation {
            field: field.to_string(),
            message: "debe ser un número mayor o igual que 0".to_string(),
        });
    }
    Ok(())
}

/// Valida que un importe o cantidad sea finito y estrictamente positivo (`> 0`).
pub fn validate_positive(field: &str, value: f64) -> Result<(), FieldViolation> {
    if !value.is_finite() || value <= 0.0 {
        return Err(FieldViolation {
            field: field.to_string(),
            message: "debe ser un número mayor que 0".to_string(),
        });
    }
    Ok(())
}

/// Valida un par opcional `(latitude, longitude)` exigiendo ambos si se envía uno.
pub fn validate_optional_coordinates(
    latitude: Option<f64>,
    longitude: Option<f64>,
) -> Result<(), FieldViolation> {
    match (latitude, longitude) {
        (Some(lat), Some(lng)) => validate_coordinates(lat, lng),
        (None, None) => Ok(()),
        (Some(_), None) => Err(FieldViolation {
            field: "longitude".to_string(),
            message: "si se indica latitude debe indicarse también longitude".to_string(),
        }),
        (None, Some(_)) => Err(FieldViolation {
            field: "latitude".to_string(),
            message: "si se indica longitude debe indicarse también latitude".to_string(),
        }),
    }
}

fn collect_result(violations: &mut Vec<FieldViolation>, res: Result<(), FieldViolation>) {
    if let Err(err) = res {
        violations.push(err);
    }
}

impl LoginRequest {
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        let mut violations = Vec::new();
        collect_result(&mut violations, validate_email(&self.email));
        collect_result(&mut violations, validate_non_empty("password", &self.password));
        if violations.is_empty() {
            Ok(())
        } else {
            Err(ValidationErrors::new(violations))
        }
    }
}

impl CreateUserRequest {
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        let mut violations = Vec::new();
        collect_result(&mut violations, validate_non_empty("name", &self.name));
        collect_result(&mut violations, validate_email(&self.email));
        collect_result(&mut violations, validate_phone(&self.phone));
        collect_result(&mut violations, validate_password(&self.password));
        if let Some(shift) = &self.shift {
            collect_result(&mut violations, validate_non_empty("shift", shift));
        }
        if violations.is_empty() {
            Ok(())
        } else {
            Err(ValidationErrors::new(violations))
        }
    }
}

impl UpdateUserRequest {
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        let mut violations = Vec::new();
        if let Some(name) = &self.name {
            collect_result(&mut violations, validate_non_empty("name", name));
        }
        if let Some(email) = &self.email {
            collect_result(&mut violations, validate_email(email));
        }
        if let Some(phone) = &self.phone {
            collect_result(&mut violations, validate_phone(phone));
        }
        if let Some(password) = &self.password {
            collect_result(&mut violations, validate_password(password));
        }
        if let Some(shift) = &self.shift {
            collect_result(&mut violations, validate_non_empty("shift", shift));
        }
        if violations.is_empty() {
            Ok(())
        } else {
            Err(ValidationErrors::new(violations))
        }
    }
}

impl CreateStoreRequest {
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        let mut violations = Vec::new();
        collect_result(&mut violations, validate_non_empty("name", &self.name));
        collect_result(&mut violations, validate_non_empty("zone", &self.zone));
        collect_result(&mut violations, validate_non_empty("address", &self.address));
        collect_result(
            &mut violations,
            validate_non_empty("contact_name", &self.contact_name),
        );
        collect_result(&mut violations, validate_phone(&self.phone));
        collect_result(
            &mut violations,
            validate_non_empty("delivery_window", &self.delivery_window),
        );
        if let Some(limit) = self.credit_limit {
            collect_result(&mut violations, validate_non_negative("credit_limit", limit));
        }
        collect_result(
            &mut violations,
            validate_optional_coordinates(self.latitude, self.longitude),
        );
        if violations.is_empty() {
            Ok(())
        } else {
            Err(ValidationErrors::new(violations))
        }
    }
}

impl UpdateStoreRequest {
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        let mut violations = Vec::new();
        if let Some(name) = &self.name {
            collect_result(&mut violations, validate_non_empty("name", name));
        }
        if let Some(zone) = &self.zone {
            collect_result(&mut violations, validate_non_empty("zone", zone));
        }
        if let Some(address) = &self.address {
            collect_result(&mut violations, validate_non_empty("address", address));
        }
        if let Some(contact_name) = &self.contact_name {
            collect_result(
                &mut violations,
                validate_non_empty("contact_name", contact_name),
            );
        }
        if let Some(phone) = &self.phone {
            collect_result(&mut violations, validate_phone(phone));
        }
        if let Some(window) = &self.delivery_window {
            collect_result(
                &mut violations,
                validate_non_empty("delivery_window", window),
            );
        }
        if let Some(balance) = self.pending_balance {
            collect_result(
                &mut violations,
                validate_non_negative("pending_balance", balance),
            );
        }
        if let Some(limit) = self.credit_limit {
            collect_result(&mut violations, validate_non_negative("credit_limit", limit));
        }
        collect_result(
            &mut violations,
            validate_optional_coordinates(self.latitude, self.longitude),
        );
        if violations.is_empty() {
            Ok(())
        } else {
            Err(ValidationErrors::new(violations))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Role;

    #[test]
    fn email_validation_cases() {
        assert!(validate_email("jefe@panaderia.es").is_ok());
        assert!(validate_email("  ").is_err());
        assert!(validate_email("sin-arroba.es").is_err());
        assert!(validate_email("a@b").is_err());
        assert!(validate_email("a b@panaderia.es").is_err());
    }

    #[test]
    fn phone_validation_cases() {
        assert!(validate_phone("+34 600 123 456").is_ok());
        assert!(validate_phone("612345").is_ok());
        assert!(validate_phone("123").is_err());
        assert!(validate_phone("abc-def").is_err());
        assert!(validate_phone("").is_err());
    }

    #[test]
    fn coordinate_validation_cases() {
        assert!(validate_coordinates(40.4168, -3.7038).is_ok());
        assert!(validate_coordinates(-90.0, 180.0).is_ok());
        assert!(validate_coordinates(91.0, 0.0).is_err());
        assert!(validate_coordinates(0.0, -181.0).is_err());
        assert!(validate_coordinates(f64::NAN, 0.0).is_err());
        assert!(validate_optional_coordinates(Some(40.4), None).is_err());
        assert!(validate_optional_coordinates(None, None).is_ok());
    }

    #[test]
    fn non_negative_and_positive_validation_cases() {
        assert!(validate_non_negative("amount", 0.0).is_ok());
        assert!(validate_non_negative("amount", 12.5).is_ok());
        assert!(validate_non_negative("amount", -0.01).is_err());
        assert!(validate_non_negative("amount", f64::INFINITY).is_err());

        assert!(validate_positive("quantity", 0.5).is_ok());
        assert!(validate_positive("quantity", 0.0).is_err());
    }

    #[test]
    fn create_user_and_store_payload_validation() {
        let valid_user = CreateUserRequest {
            name: "Carlos Ruiz".to_string(),
            email: "carlos@panaderia.es".to_string(),
            phone: "+34 600 111 222".to_string(),
            password: "Password123".to_string(),
            role: Role::Repartidor,
            assigned_vehicle: Some("Furgoneta 01".to_string()),
            shift: Some("Mañana".to_string()),
            status: None,
        };
        assert!(valid_user.validate().is_ok());

        let invalid_user = CreateUserRequest {
            password: "123".to_string(),
            email: "mal".to_string(),
            ..valid_user
        };
        let err = invalid_user.validate().unwrap_err();
        assert_eq!(err.violations.len(), 2);

        let valid_store = CreateStoreRequest {
            name: "La Espiga Centro".to_string(),
            zone: "Centro".to_string(),
            address: "Calle Mayor 12".to_string(),
            contact_name: "Marta".to_string(),
            phone: "910123456".to_string(),
            delivery_window: "06:30 - 08:00".to_string(),
            credit_limit: Some(500.0),
            delivery_notes: None,
            latitude: Some(40.4168),
            longitude: Some(-3.7038),
            active: Some(true),
        };
        assert!(valid_store.validate().is_ok());

        let invalid_store = CreateStoreRequest {
            credit_limit: Some(-10.0),
            longitude: None,
            ..valid_store
        };
        let err = invalid_store.validate().unwrap_err();
        assert_eq!(err.violations.len(), 2);
    }
}
