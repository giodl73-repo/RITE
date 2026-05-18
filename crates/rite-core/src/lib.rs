use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiteDocument {
    pub document_id: String,
    #[serde(default)]
    pub rites: Vec<RiteRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiteRecord {
    pub rite_id: String,
    pub label: String,
    pub kind: RiteKind,
    #[serde(default)]
    pub obligations: Vec<Obligation>,
    #[serde(default)]
    pub taboos: Vec<Taboo>,
    #[serde(default)]
    pub sacred_places: Vec<SacredPlace>,
    #[serde(default)]
    pub calendar_events: Vec<CalendarEvent>,
    #[serde(default)]
    pub evidence_refs: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiteKind {
    Funeral,
    Festival,
    Oath,
    Pilgrimage,
    Mourning,
    Harvest,
    Legitimacy,
    Protection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Obligation {
    pub obligation_id: String,
    pub category: ObligationCategory,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObligationCategory {
    RequiredAction,
    ForbiddenAction,
    CostlyOffering,
    MourningDuty,
    AccessRule,
    StatusChange,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Taboo {
    pub taboo_id: String,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SacredPlace {
    pub place_id: String,
    pub label: String,
    #[serde(default)]
    pub access_note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub event_id: String,
    pub timing: String,
    pub activates: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceRef {
    pub source_id: String,
    pub path: String,
    #[serde(default)]
    pub anchor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    pub severity: FindingSeverity,
    pub code: String,
    pub location: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    Error,
    Warning,
}

pub fn validate_document(document: &RiteDocument) -> Vec<ValidationFinding> {
    let mut findings = Vec::new();
    require_non_empty(
        &mut findings,
        "document_id",
        "document.document_id",
        &document.document_id,
    );

    let mut seen_rite_ids = HashSet::new();
    for rite in &document.rites {
        require_non_empty(&mut findings, "rite_id", "rite.rite_id", &rite.rite_id);
        require_non_empty(&mut findings, "rite_label", &rite.rite_id, &rite.label);
        if !seen_rite_ids.insert(rite.rite_id.as_str()) {
            findings.push(error(
                "duplicate_rite_id",
                &rite.rite_id,
                "rite id appears more than once",
            ));
        }
        if rite.obligations.is_empty() {
            findings.push(warning(
                "rite_without_obligation",
                &rite.rite_id,
                "rite has no obligation",
            ));
        }
        if rite.evidence_refs.is_empty() {
            findings.push(warning(
                "rite_without_evidence",
                &rite.rite_id,
                "rite has no evidence reference",
            ));
        }
        for obligation in &rite.obligations {
            require_non_empty(
                &mut findings,
                "obligation_id",
                &rite.rite_id,
                &obligation.obligation_id,
            );
            require_non_empty(
                &mut findings,
                "obligation_description",
                &obligation.obligation_id,
                &obligation.description,
            );
        }
        for taboo in &rite.taboos {
            require_non_empty(&mut findings, "taboo_id", &rite.rite_id, &taboo.taboo_id);
            require_non_empty(
                &mut findings,
                "taboo_boundary",
                &taboo.taboo_id,
                &taboo.boundary,
            );
        }
        for place in &rite.sacred_places {
            require_non_empty(&mut findings, "place_id", &rite.rite_id, &place.place_id);
            require_non_empty(&mut findings, "place_label", &place.place_id, &place.label);
        }
        for event in &rite.calendar_events {
            require_non_empty(
                &mut findings,
                "calendar_event_id",
                &rite.rite_id,
                &event.event_id,
            );
            require_non_empty(
                &mut findings,
                "calendar_timing",
                &event.event_id,
                &event.timing,
            );
            require_non_empty(
                &mut findings,
                "calendar_activates",
                &event.event_id,
                &event.activates,
            );
        }
    }

    findings
}

pub fn has_errors(findings: &[ValidationFinding]) -> bool {
    findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Error)
}

fn require_non_empty(
    findings: &mut Vec<ValidationFinding>,
    code: &str,
    location: &str,
    value: &str,
) {
    if value.trim().is_empty() {
        findings.push(error(code, location, "required field is empty"));
    }
}

fn error(code: &str, location: &str, message: &str) -> ValidationFinding {
    ValidationFinding {
        severity: FindingSeverity::Error,
        code: code.to_string(),
        location: location.to_string(),
        message: message.to_string(),
    }
}

fn warning(code: &str, location: &str, message: &str) -> ValidationFinding {
    ValidationFinding {
        severity: FindingSeverity::Warning,
        code: code.to_string(),
        location: location.to_string(),
        message: message.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_rite_ids_are_errors() {
        let document = RiteDocument {
            document_id: "rite:test".to_string(),
            rites: vec![rite("rite:winter-oath"), rite("rite:winter-oath")],
        };
        let findings = validate_document(&document);
        assert!(findings
            .iter()
            .any(|finding| finding.code == "duplicate_rite_id"));
        assert!(has_errors(&findings));
    }

    #[test]
    fn complete_rite_record_has_no_errors() {
        let document = RiteDocument {
            document_id: "rite:test".to_string(),
            rites: vec![rite("rite:winter-oath")],
        };
        assert!(!has_errors(&validate_document(&document)));
    }

    fn rite(rite_id: &str) -> RiteRecord {
        RiteRecord {
            rite_id: rite_id.to_string(),
            label: "Winter hearth oath".to_string(),
            kind: RiteKind::Oath,
            obligations: vec![Obligation {
                obligation_id: "obligation:share-last-fuel".to_string(),
                category: ObligationCategory::RequiredAction,
                description: "Households must pool emergency fuel before the first hard freeze.".to_string(),
            }],
            taboos: vec![Taboo {
                taboo_id: "taboo:cold-hearth".to_string(),
                boundary: "Letting a neighbor's hearth die during oath week marks a household as oath-broken.".to_string(),
            }],
            sacred_places: vec![SacredPlace {
                place_id: "place:common-hearth".to_string(),
                label: "Common hearth".to_string(),
                access_note: Some("Open during oath week, even to rivals.".to_string()),
            }],
            calendar_events: vec![CalendarEvent {
                event_id: "calendar:first-hard-freeze".to_string(),
                timing: "first hard freeze".to_string(),
                activates: "Emergency fuel obligations begin.".to_string(),
            }],
            evidence_refs: vec![EvidenceRef {
                source_id: "seed".to_string(),
                path: "fixtures\\seed-rite.json".to_string(),
                anchor: Some(rite_id.to_string()),
            }],
        }
    }
}
