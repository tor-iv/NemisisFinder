//! # NemisisFinder Matching Engine
//!
//! This library implements maximum weight perfect matching algorithms
//! to pair users with the most opposite opinions.
//!
//! ## Core Concepts
//! - **User**: Someone who completed the questionnaire
//! - **Match**: A pairing of two users with their opposition score
//! - **ScoringStrategy**: Different ways to calculate "opposite-ness"
//! - **Matcher**: Algorithms to find optimal pairings

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// ============================================================================
// Modules
// ============================================================================

pub mod scoring;
pub mod matching;

// Re-export scoring strategies for convenient access
pub use scoring::{
    EuclideanDistanceScorer, PolarizationScorer, SimpleDifferenceScorer, WeightedScorer,
};

// Re-export matching algorithms
pub use matching::GreedyMatcher;

// ============================================================================
// Core Data Structures
// ============================================================================

/// Represents a user and their questionnaire responses
///
/// Each user has:
/// - A unique identifier (from Firebase)
/// - A vector of responses (1-7 scale for 25 questions)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier (Firebase UID)
    pub id: String,

    /// Questionnaire responses (each value between 1-7)
    /// Length should match the number of questions (25 in your case)
    pub responses: Vec<i32>,
}

impl User {
    /// Create a new user with validated responses
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the user
    /// * `responses` - Vector of questionnaire answers (1-7 scale)
    ///
    /// # Returns
    /// * `Ok(User)` if responses are valid
    /// * `Err(String)` if responses contain invalid values
    pub fn new(id: String, responses: Vec<i32>) -> Result<User, String> {
        // Validate that all responses are in the 1-7 range
        if responses.iter().any(|&r| r < 1 || r > 7) {
            return Err("All responses must be between 1 and 7".to_string());
        }

        Ok(User { id, responses })
    }

    /// Get the number of questions this user answered
    pub fn num_questions(&self) -> usize {
        self.responses.len()
    }
}

/// Represents a match between two users
///
/// A match contains:
/// - References to both users (by ID)
/// - The calculated opposition score (higher = more opposite)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    /// First user's ID
    pub user1_id: String,

    /// Second user's ID
    pub user2_id: String,

    /// Opposition score (meaning depends on scoring strategy)
    /// Higher score = more opposite opinions
    pub score: f64,
}

impl Match {
    /// Create a new match between two users
    pub fn new(user1_id: String, user2_id: String, score: f64) -> Self {
        Match {
            user1_id,
            user2_id,
            score,
        }
    }
}

// ============================================================================
// Scoring Strategy Trait
// ============================================================================

/// Trait defining how to calculate opposition between two users
///
/// Different strategies can emphasize different aspects:
/// - Simple subtraction: Sum of absolute differences
/// - Euclidean: Emphasizes large differences
/// - Weighted: Some questions matter more
/// - Polarization: Extreme positions weighted higher
pub trait ScoringStrategy {
    /// Calculate the opposition score between two users
    ///
    /// # Arguments
    /// * `user1` - First user
    /// * `user2` - Second user
    ///
    /// # Returns
    /// * Opposition score (higher = more opposite)
    ///
    /// # Panics
    /// * If users have different numbers of responses
    fn calculate_score(&self, user1: &User, user2: &User) -> f64;

    /// Get a human-readable name for this strategy
    fn name(&self) -> &str;
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation_valid() {
        let responses = vec![1, 2, 3, 4, 5, 6, 7];
        let user = User::new("test_id".to_string(), responses.clone());

        assert!(user.is_ok());
        let user = user.unwrap();
        assert_eq!(user.id, "test_id");
        assert_eq!(user.responses, responses);
        assert_eq!(user.num_questions(), 7);
    }

    #[test]
    fn test_user_creation_invalid_low() {
        let responses = vec![0, 2, 3, 4, 5, 6, 7]; // 0 is invalid
        let user = User::new("test_id".to_string(), responses);

        assert!(user.is_err());
    }

    #[test]
    fn test_user_creation_invalid_high() {
        let responses = vec![1, 2, 3, 4, 5, 6, 8]; // 8 is invalid
        let user = User::new("test_id".to_string(), responses);

        assert!(user.is_err());
    }

    #[test]
    fn test_match_creation() {
        let match_obj = Match::new(
            "user1".to_string(),
            "user2".to_string(),
            42.5,
        );

        assert_eq!(match_obj.user1_id, "user1");
        assert_eq!(match_obj.user2_id, "user2");
        assert_eq!(match_obj.score, 42.5);
    }
}
