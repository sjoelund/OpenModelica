//! Translation of Util/ErrorTypes.mo
//!
//! This module defines types used by the error handling system.
//! It contains severity levels, message types, and message structures.

use im::Vector;
use std::fmt;

// ============================================================================
// Persistent list type (mapped to im::Vector since im 15.x has no List)
// ============================================================================

/// Persistent list type (mapped to im::Vector since im 15.x has no List)
type List<T> = Vector<T>;

// ============================================================================
// SourceInfo - file information (defined locally, normally from absyn)
// ============================================================================

/// SourceInfo (Info) - file information (filename, read-only flag, line/column positions)
/// This is a built-in compiler type, defined outside of ErrorTypes.mo.
#[derive(Debug, Clone, PartialEq)]
pub struct SourceInfo {
    pub file_name: String,
    pub is_read_only: bool,
    pub start_line: i32,
    pub start_column: i32,
    pub end_line: i32,
    pub end_column: i32,
}

// ============================================================================
// Severity - severity of message
// ============================================================================

/// Severity levels for compiler messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Severity {
    /// Error because of a failure in the tool
    INTERNAL,
    /// Error when tool can not succeed in translation because of a user error
    ERROR,
    /// Warning when tool succeeds but with warning
    WARNING,
    /// Additional information to user, e.g. what actions tool has taken
    NOTIFICATION,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::INTERNAL => write!(f, "INTERNAL"),
            Severity::ERROR => write!(f, "ERROR"),
            Severity::WARNING => write!(f, "WARNING"),
            Severity::NOTIFICATION => write!(f, "NOTIFICATION"),
        }
    }
}

// ============================================================================
// MessageType - runtime scripting / interpretation error
// ============================================================================

/// Types of messages categorizing where an error occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum MessageType {
    /// Syntax errors
    SYNTAX,
    /// Grammar errors
    GRAMMAR,
    /// Instantiation errors: up to flat modelica
    TRANSLATION,
    /// Symbolic manipulation error, simcodegen, up to .exe file
    SYMBOLIC,
    /// Runtime simulation error
    SIMULATION,
    /// Runtime scripting / interpretation error
    SCRIPTING,
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::SYNTAX => write!(f, "SYNTAX"),
            MessageType::GRAMMAR => write!(f, "GRAMMAR"),
            MessageType::TRANSLATION => write!(f, "TRANSLATION"),
            MessageType::SYMBOLIC => write!(f, "SYMBOLIC"),
            MessageType::SIMULATION => write!(f, "SIMULATION"),
            MessageType::SCRIPTING => write!(f, "SCRIPTING"),
        }
    }
}

// ============================================================================
// Simple type aliases
// ============================================================================

/// Unique error id. Used to look up message string and type and severity.
pub type ErrorID = i32;

/// Tokens to insert into message at positions identified by - %s for string, %n for string number n.
pub type MessageTokens = List<String>;

// ============================================================================
// Gettext.TranslatableContent - translation from Gettext package
// ============================================================================

/// A translatable message content (from Gettext.TranslatableContent).
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TranslatableContent {
    /// A translatable message with a msgid
    GETTEXT { msgid: String },
    /// A non-translatable string
    NOTRANS { str: String },
}

impl fmt::Display for TranslatableContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TranslatableContent::GETTEXT { msgid } => write!(f, "gettext({msgid})"),
            TranslatableContent::NOTRANS { str: s } => write!(f, "{s}"),
        }
    }
}

// ============================================================================
// Message - a message with id, type, severity, and content
// ============================================================================

/// A compiler message containing id, type, severity, and content.
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Message {
    MESSAGE {
        id: ErrorID,
        ty: MessageType,
        severity: Severity,
        message: TranslatableContent,
    },
}

// ============================================================================
// TotalMessage - a message with source information
// ============================================================================

/// A message with attached source location information.
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TotalMessage {
    TOTALMESSAGE {
        msg: Message,
        info: SourceInfo,
    },
}
