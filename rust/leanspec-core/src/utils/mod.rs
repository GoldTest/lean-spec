//! Backward-compatible re-exports from domain modules.
//!
//! Implementations have moved to `spec_ops`, `io`, and `compute` modules.
//! This module re-exports everything for backward compatibility.

pub use crate::compute::{
    global_token_counter, Insights, SpecStats, TokenCount, TokenCounter, TokenStatus,
};
pub use crate::io::{
    hash_content, DiscoveredProject, DiscoveryError, ProjectDiscovery, TemplateError,
    TemplateLoader,
};
pub use crate::spec_ops::{
    apply_checklist_toggles, apply_replacements, apply_section_updates, preserve_title_heading,
    rebuild_content, split_frontmatter, ArchiveError, ChecklistToggle, ChecklistToggleResult,
    CompleteDependencyGraph, DependencyGraph, ImpactRadius, LoadError, MatchMode, MetadataUpdate,
    Replacement, ReplacementResult, SectionMode, SectionUpdate, SpecArchiver, SpecHierarchyNode,
    SpecLoader, SpecRelationshipIndex, SpecWriter, WriteError,
};
