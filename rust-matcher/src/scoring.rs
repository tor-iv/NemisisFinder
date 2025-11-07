//! Scoring strategies for calculating opposition between users
//!
//! This module implements different algorithms for measuring how "opposite"
//! two users' opinions are based on their questionnaire responses.

use crate::{ScoringStrategy, User};

// ============================================================================
// Simple Difference Scorer (Baseline)
// ============================================================================

/// Calculates opposition as the sum of absolute differences
///
/// This is the most straightforward scoring method:
/// For each question, subtract answers and take absolute value, then sum.
///
/// # Example
/// ```text
/// User A: [1, 7, 3]  (strongly disagree, strongly agree, neutral)
/// User B: [7, 1, 5]  (strongly agree, strongly disagree, somewhat agree)
///
/// Differences: |1-7| + |7-1| + |3-5| = 6 + 6 + 2 = 14
/// ```
///
/// ## Characteristics
/// - Linear: All differences weighted equally
/// - Range: 0 (identical) to 6N (maximum opposition, where N = num questions)
/// - For 25 questions: 0-150 range
pub struct SimpleDifferenceScorer;

impl ScoringStrategy for SimpleDifferenceScorer {
    fn calculate_score(&self, user1: &User, user2: &User) -> f64 {
        // Ensure both users answered the same number of questions
        assert_eq!(
            user1.responses.len(),
            user2.responses.len(),
            "Users must have same number of responses"
        );

        // Calculate sum of absolute differences
        user1
            .responses
            .iter()                    // Create iterator over user1's responses
            .zip(&user2.responses)     // Pair with user2's responses
            .map(|(r1, r2)| (r1 - r2).abs())  // Calculate absolute difference
            .sum::<i32>() as f64       // Sum all differences, convert to f64
    }

    fn name(&self) -> &str {
        "Simple Difference"
    }
}

// ============================================================================
// Euclidean Distance Scorer
// ============================================================================

/// Calculates opposition using Euclidean distance (L2 norm)
///
/// This method squares differences before summing, which emphasizes
/// large differences over many small ones.
///
/// # Formula
/// ```text
/// score = sqrt(sum((r1[i] - r2[i])^2))
/// ```
///
/// # Example
/// ```text
/// User A: [1, 4, 4]
/// User B: [4, 4, 4]
///
/// Simple Difference: |1-4| + |4-4| + |4-4| = 3
/// Euclidean: sqrt((1-4)^2 + (4-4)^2 + (4-4)^2) = sqrt(9) = 3
///
/// User C: [1, 7, 4]
/// User D: [4, 4, 4]
///
/// Simple Difference: |1-4| + |7-4| + |4-4| = 6
/// Euclidean: sqrt((1-4)^2 + (7-4)^2 + (4-4)^2) = sqrt(18) = 4.24
/// ```
///
/// ## Characteristics
/// - Non-linear: Large differences weighted more heavily
/// - Range: 0 to 6√N (where N = num questions)
/// - For 25 questions: 0 to 30 range
/// - Useful when you want to prioritize "extreme" opposition over "consistent" opposition
pub struct EuclideanDistanceScorer;

impl ScoringStrategy for EuclideanDistanceScorer {
    fn calculate_score(&self, user1: &User, user2: &User) -> f64 {
        assert_eq!(
            user1.responses.len(),
            user2.responses.len(),
            "Users must have same number of responses"
        );

        // Calculate Euclidean distance: sqrt(sum of squared differences)
        let sum_of_squares: i32 = user1
            .responses
            .iter()
            .zip(&user2.responses)
            .map(|(r1, r2)| {
                let diff = r1 - r2;
                diff * diff  // Square the difference
            })
            .sum();

        // Take square root and convert to f64
        (sum_of_squares as f64).sqrt()
    }

    fn name(&self) -> &str {
        "Euclidean Distance"
    }
}

// ============================================================================
// Weighted Scorer
// ============================================================================

/// Calculates opposition with custom weights per question
///
/// This strategy allows you to emphasize certain questions over others.
/// For example, disagreement on political questions might be weighted
/// higher than disagreement on lifestyle preferences.
///
/// # Example
/// ```text
/// Questions: [Politics, Religion, Pizza topping]
/// Weights:   [3.0,      2.5,      0.5]
///
/// User A: [1, 7, 3]
/// User B: [7, 1, 5]
///
/// Score = |1-7|×3.0 + |7-1|×2.5 + |3-5|×0.5
///       = 6×3.0 + 6×2.5 + 2×0.5
///       = 18 + 15 + 1 = 34
/// ```
///
/// ## Characteristics
/// - Flexible: Emphasize questions that matter most
/// - Range: 0 to (6 × sum of weights)
/// - Requires: Pre-defined weight configuration
#[derive(Debug, Clone)]
pub struct WeightedScorer {
    /// Weight for each question (must match number of questions)
    weights: Vec<f64>,
}

impl WeightedScorer {
    /// Create a new weighted scorer with custom weights
    ///
    /// # Arguments
    /// * `weights` - Vector of weights (one per question)
    ///
    /// # Returns
    /// * `Ok(WeightedScorer)` if weights are valid
    /// * `Err(String)` if weights are invalid (empty, negative, or zero)
    ///
    /// # Example
    /// ```
    /// use rust_matcher::scoring::WeightedScorer;
    ///
    /// // Politics and religion weighted higher than hobbies
    /// let weights = vec![3.0, 2.5, 1.0, 1.0, 0.5];
    /// let scorer = WeightedScorer::new(weights).unwrap();
    /// ```
    pub fn new(weights: Vec<f64>) -> Result<Self, String> {
        // Validate weights
        if weights.is_empty() {
            return Err("Weights vector cannot be empty".to_string());
        }

        if weights.iter().any(|&w| w <= 0.0) {
            return Err("All weights must be positive".to_string());
        }

        Ok(WeightedScorer { weights })
    }

    /// Create a weighted scorer with equal weights (equivalent to SimpleDifference)
    ///
    /// # Arguments
    /// * `num_questions` - Number of questions in the questionnaire
    ///
    /// # Example
    /// ```
    /// use rust_matcher::scoring::WeightedScorer;
    ///
    /// let scorer = WeightedScorer::equal_weights(25);
    /// ```
    pub fn equal_weights(num_questions: usize) -> Self {
        WeightedScorer {
            weights: vec![1.0; num_questions],
        }
    }

    /// Get the number of questions this scorer expects
    pub fn num_questions(&self) -> usize {
        self.weights.len()
    }

    /// Get a reference to the weights
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }
}

impl ScoringStrategy for WeightedScorer {
    fn calculate_score(&self, user1: &User, user2: &User) -> f64 {
        assert_eq!(
            user1.responses.len(),
            user2.responses.len(),
            "Users must have same number of responses"
        );

        assert_eq!(
            user1.responses.len(),
            self.weights.len(),
            "Number of responses must match number of weights"
        );

        // Calculate weighted sum of absolute differences
        user1
            .responses
            .iter()
            .zip(&user2.responses)
            .zip(&self.weights)
            .map(|((r1, r2), weight)| {
                let diff = (r1 - r2).abs() as f64;
                diff * weight
            })
            .sum()
    }

    fn name(&self) -> &str {
        "Weighted"
    }
}

// ============================================================================
// Polarization Scorer
// ============================================================================

/// Calculates opposition with emphasis on passionate/extreme positions
///
/// This scorer recognizes that strongly-held opinions (1 or 7) represent
/// more commitment than moderate positions (3-5). It multiplies difference
/// scores by the "conviction" of both users.
///
/// # Philosophy
/// Two people who both feel *strongly* (even if opposite) make better
/// debate partners than pairing someone passionate with someone indifferent.
///
/// # Example
/// ```text
/// User A answers 1 (strongly disagree) - conviction weight: 1.5x
/// User B answers 7 (strongly agree) - conviction weight: 1.5x
/// Difference: |1-7| = 6
/// Final score: 6 × 1.5 × 1.5 = 13.5
///
/// vs.
///
/// User C answers 3 (somewhat disagree) - conviction weight: 1.0x
/// User D answers 5 (somewhat agree) - conviction weight: 1.0x
/// Difference: |3-5| = 2
/// Final score: 2 × 1.0 × 1.0 = 2.0
/// ```
///
/// ## Characteristics
/// - Emphasizes passion over moderation
/// - Rewards strong disagreements between committed users
/// - De-emphasizes disagreements where one/both users are neutral
/// - Range: 0 to ~225 (for 25 questions with default multipliers)
#[derive(Debug, Clone)]
pub struct PolarizationScorer {
    /// Multiplier for extreme positions (1 or 7)
    extreme_multiplier: f64,

    /// Multiplier for leaning positions (2 or 6)
    lean_multiplier: f64,

    /// Multiplier for moderate positions (3, 4, or 5)
    moderate_multiplier: f64,
}

impl PolarizationScorer {
    /// Create a new polarization scorer with custom multipliers
    ///
    /// # Arguments
    /// * `extreme_multiplier` - Weight for answers 1 or 7 (default: 1.5)
    /// * `lean_multiplier` - Weight for answers 2 or 6 (default: 1.2)
    /// * `moderate_multiplier` - Weight for answers 3-5 (default: 1.0)
    ///
    /// # Example
    /// ```
    /// use rust_matcher::scoring::PolarizationScorer;
    ///
    /// // Heavy emphasis on extreme positions
    /// let scorer = PolarizationScorer::new(2.0, 1.5, 1.0);
    /// ```
    pub fn new(extreme_multiplier: f64, lean_multiplier: f64, moderate_multiplier: f64) -> Self {
        PolarizationScorer {
            extreme_multiplier,
            lean_multiplier,
            moderate_multiplier,
        }
    }

    /// Create a polarization scorer with default multipliers
    ///
    /// Defaults:
    /// - Extreme (1, 7): 1.5x
    /// - Lean (2, 6): 1.2x
    /// - Moderate (3-5): 1.0x
    ///
    /// # Example
    /// ```
    /// use rust_matcher::scoring::PolarizationScorer;
    ///
    /// let scorer = PolarizationScorer::default();
    /// ```
    pub fn default() -> Self {
        PolarizationScorer {
            extreme_multiplier: 1.5,
            lean_multiplier: 1.2,
            moderate_multiplier: 1.0,
        }
    }

    /// Calculate the polarization weight for a given answer
    ///
    /// This is a private helper method that determines how "passionate"
    /// or "committed" a particular answer represents.
    ///
    /// # Pattern Matching on Ranges
    /// This uses Rust's powerful pattern matching to categorize answers:
    /// - `1 | 7` matches either 1 OR 7
    /// - `3..=5` matches range from 3 to 5 inclusive
    fn polarization_weight(&self, answer: i32) -> f64 {
        match answer {
            1 | 7 => self.extreme_multiplier,   // Strongly disagree/agree
            2 | 6 => self.lean_multiplier,      // Lean disagree/agree
            3..=5 => self.moderate_multiplier,  // Neutral to somewhat
            _ => 1.0,                           // Fallback (shouldn't happen with validation)
        }
    }
}

impl ScoringStrategy for PolarizationScorer {
    fn calculate_score(&self, user1: &User, user2: &User) -> f64 {
        assert_eq!(
            user1.responses.len(),
            user2.responses.len(),
            "Users must have same number of responses"
        );

        // Calculate polarization-weighted score
        user1
            .responses
            .iter()
            .zip(&user2.responses)
            .map(|(&r1, &r2)| {
                let diff = (r1 - r2).abs() as f64;
                let weight1 = self.polarization_weight(r1);
                let weight2 = self.polarization_weight(r2);

                // Multiply difference by BOTH users' conviction levels
                diff * weight1 * weight2
            })
            .sum()
    }

    fn name(&self) -> &str {
        "Polarization"
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_difference_identical_users() {
        let user1 = User::new("user1".to_string(), vec![1, 2, 3, 4, 5]).unwrap();
        let user2 = User::new("user2".to_string(), vec![1, 2, 3, 4, 5]).unwrap();

        let scorer = SimpleDifferenceScorer;
        let score = scorer.calculate_score(&user1, &user2);

        assert_eq!(score, 0.0, "Identical users should have 0 opposition");
    }

    #[test]
    fn test_simple_difference_maximum_opposition() {
        // User1 answers all 1s (strongly disagree with everything)
        let user1 = User::new("user1".to_string(), vec![1, 1, 1, 1, 1]).unwrap();
        // User2 answers all 7s (strongly agree with everything)
        let user2 = User::new("user2".to_string(), vec![7, 7, 7, 7, 7]).unwrap();

        let scorer = SimpleDifferenceScorer;
        let score = scorer.calculate_score(&user1, &user2);

        // Each question: |1-7| = 6, five questions = 30
        assert_eq!(score, 30.0, "Maximum opposition should be 6 * num_questions");
    }

    #[test]
    fn test_simple_difference_mixed() {
        let user1 = User::new("user1".to_string(), vec![1, 7, 3, 5]).unwrap();
        let user2 = User::new("user2".to_string(), vec![7, 1, 5, 4]).unwrap();

        let scorer = SimpleDifferenceScorer;
        let score = scorer.calculate_score(&user1, &user2);

        // |1-7| + |7-1| + |3-5| + |5-4| = 6 + 6 + 2 + 1 = 15
        assert_eq!(score, 15.0);
    }

    #[test]
    fn test_simple_difference_is_symmetric() {
        let user1 = User::new("user1".to_string(), vec![2, 4, 6]).unwrap();
        let user2 = User::new("user2".to_string(), vec![5, 1, 7]).unwrap();

        let scorer = SimpleDifferenceScorer;
        let score1 = scorer.calculate_score(&user1, &user2);
        let score2 = scorer.calculate_score(&user2, &user1);

        assert_eq!(score1, score2, "Score should be symmetric (order doesn't matter)");
    }

    #[test]
    #[should_panic(expected = "Users must have same number of responses")]
    fn test_simple_difference_mismatched_lengths() {
        let user1 = User::new("user1".to_string(), vec![1, 2, 3]).unwrap();
        let user2 = User::new("user2".to_string(), vec![1, 2]).unwrap();

        let scorer = SimpleDifferenceScorer;
        scorer.calculate_score(&user1, &user2); // Should panic
    }

    // ========================================================================
    // Euclidean Distance Scorer Tests
    // ========================================================================

    #[test]
    fn test_euclidean_identical_users() {
        let user1 = User::new("user1".to_string(), vec![3, 5, 2]).unwrap();
        let user2 = User::new("user2".to_string(), vec![3, 5, 2]).unwrap();

        let scorer = EuclideanDistanceScorer;
        let score = scorer.calculate_score(&user1, &user2);

        assert_eq!(score, 0.0, "Identical users should have 0 distance");
    }

    #[test]
    fn test_euclidean_maximum_opposition() {
        let user1 = User::new("user1".to_string(), vec![1, 1, 1]).unwrap();
        let user2 = User::new("user2".to_string(), vec![7, 7, 7]).unwrap();

        let scorer = EuclideanDistanceScorer;
        let score = scorer.calculate_score(&user1, &user2);

        // sqrt((6)^2 + (6)^2 + (6)^2) = sqrt(108) ≈ 10.39
        assert!((score - 10.392).abs() < 0.01, "Expected ~10.39, got {}", score);
    }

    #[test]
    fn test_euclidean_vs_simple_difference() {
        // Case 1: One large difference
        let user1 = User::new("user1".to_string(), vec![1, 4, 4]).unwrap();
        let user2 = User::new("user2".to_string(), vec![4, 4, 4]).unwrap();

        let euclidean = EuclideanDistanceScorer;
        let simple = SimpleDifferenceScorer;

        let euclidean_score = euclidean.calculate_score(&user1, &user2);
        let simple_score = simple.calculate_score(&user1, &user2);

        // sqrt(9) = 3, simple = 3 (same in this case)
        assert_eq!(euclidean_score, 3.0);
        assert_eq!(simple_score, 3.0);

        // Case 2: Multiple large differences (Euclidean emphasizes more)
        let user3 = User::new("user3".to_string(), vec![1, 7, 1]).unwrap();
        let user4 = User::new("user4".to_string(), vec![4, 4, 4]).unwrap();

        let euclidean_score2 = euclidean.calculate_score(&user3, &user4);
        let simple_score2 = simple.calculate_score(&user3, &user4);

        // Euclidean: sqrt(9 + 9 + 9) = sqrt(27) ≈ 5.196
        // Simple: 3 + 3 + 3 = 9
        assert!((euclidean_score2 - 5.196).abs() < 0.01);
        assert_eq!(simple_score2, 9.0);

        // Euclidean score is lower because it de-emphasizes multiple moderate differences
        assert!(euclidean_score2 < simple_score2);
    }

    #[test]
    fn test_euclidean_is_symmetric() {
        let user1 = User::new("user1".to_string(), vec![2, 5, 1, 7]).unwrap();
        let user2 = User::new("user2".to_string(), vec![6, 3, 4, 2]).unwrap();

        let scorer = EuclideanDistanceScorer;
        let score1 = scorer.calculate_score(&user1, &user2);
        let score2 = scorer.calculate_score(&user2, &user1);

        assert_eq!(score1, score2, "Euclidean distance should be symmetric");
    }

    // ========================================================================
    // Weighted Scorer Tests
    // ========================================================================

    #[test]
    fn test_weighted_creation_valid() {
        let weights = vec![1.0, 2.0, 1.5, 0.5];
        let scorer = WeightedScorer::new(weights.clone());

        assert!(scorer.is_ok());
        let scorer = scorer.unwrap();
        assert_eq!(scorer.num_questions(), 4);
        assert_eq!(scorer.weights(), &[1.0, 2.0, 1.5, 0.5]);
    }

    #[test]
    fn test_weighted_creation_invalid_empty() {
        let weights = vec![];
        let scorer = WeightedScorer::new(weights);

        assert!(scorer.is_err());
        assert_eq!(scorer.unwrap_err(), "Weights vector cannot be empty");
    }

    #[test]
    fn test_weighted_creation_invalid_negative() {
        let weights = vec![1.0, -2.0, 1.5];
        let scorer = WeightedScorer::new(weights);

        assert!(scorer.is_err());
        assert_eq!(scorer.unwrap_err(), "All weights must be positive");
    }

    #[test]
    fn test_weighted_creation_invalid_zero() {
        let weights = vec![1.0, 0.0, 1.5];
        let scorer = WeightedScorer::new(weights);

        assert!(scorer.is_err());
        assert_eq!(scorer.unwrap_err(), "All weights must be positive");
    }

    #[test]
    fn test_weighted_equal_weights() {
        let scorer = WeightedScorer::equal_weights(5);

        assert_eq!(scorer.num_questions(), 5);
        assert_eq!(scorer.weights(), &[1.0, 1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_weighted_basic_calculation() {
        // Weights: [2.0, 1.0, 3.0]
        let weights = vec![2.0, 1.0, 3.0];
        let scorer = WeightedScorer::new(weights).unwrap();

        let user1 = User::new("user1".to_string(), vec![1, 4, 7]).unwrap();
        let user2 = User::new("user2".to_string(), vec![7, 2, 1]).unwrap();

        let score = scorer.calculate_score(&user1, &user2);

        // |1-7|×2.0 + |4-2|×1.0 + |7-1|×3.0 = 6×2 + 2×1 + 6×3 = 12 + 2 + 18 = 32
        assert_eq!(score, 32.0);
    }

    #[test]
    fn test_weighted_emphasizes_important_questions() {
        let user1 = User::new("user1".to_string(), vec![1, 4]).unwrap();
        let user2 = User::new("user2".to_string(), vec![7, 5]).unwrap();

        // Equal weights: both questions matter equally
        let equal_scorer = WeightedScorer::equal_weights(2);
        let equal_score = equal_scorer.calculate_score(&user1, &user2);
        // |1-7|×1 + |4-5|×1 = 6 + 1 = 7
        assert_eq!(equal_score, 7.0);

        // Heavy weight on first question
        let weighted_scorer = WeightedScorer::new(vec![10.0, 1.0]).unwrap();
        let weighted_score = weighted_scorer.calculate_score(&user1, &user2);
        // |1-7|×10 + |4-5|×1 = 60 + 1 = 61
        assert_eq!(weighted_score, 61.0);
    }

    #[test]
    fn test_weighted_vs_simple_difference() {
        let user1 = User::new("user1".to_string(), vec![1, 3, 5]).unwrap();
        let user2 = User::new("user2".to_string(), vec![4, 6, 2]).unwrap();

        // Equal weights should match SimpleDifferenceScorer
        let weighted = WeightedScorer::equal_weights(3);
        let simple = SimpleDifferenceScorer;

        let weighted_score = weighted.calculate_score(&user1, &user2);
        let simple_score = simple.calculate_score(&user1, &user2);

        assert_eq!(weighted_score, simple_score);
    }

    #[test]
    #[should_panic(expected = "Number of responses must match number of weights")]
    fn test_weighted_mismatched_num_questions() {
        let weights = vec![1.0, 2.0, 3.0];
        let scorer = WeightedScorer::new(weights).unwrap();

        let user1 = User::new("user1".to_string(), vec![1, 2]).unwrap(); // Only 2 questions
        let user2 = User::new("user2".to_string(), vec![3, 4]).unwrap();

        scorer.calculate_score(&user1, &user2); // Should panic
    }

    // ========================================================================
    // Polarization Scorer Tests
    // ========================================================================

    #[test]
    fn test_polarization_creation() {
        let scorer = PolarizationScorer::new(2.0, 1.5, 1.0);

        // Verify creation (can't directly access private fields, but check it works)
        assert_eq!(scorer.name(), "Polarization");
    }

    #[test]
    fn test_polarization_default() {
        let scorer = PolarizationScorer::default();

        assert_eq!(scorer.name(), "Polarization");
    }

    #[test]
    fn test_polarization_extreme_vs_extreme() {
        // Both users have extreme positions (1 vs 7)
        let user1 = User::new("user1".to_string(), vec![1]).unwrap();
        let user2 = User::new("user2".to_string(), vec![7]).unwrap();

        let scorer = PolarizationScorer::default(); // 1.5x for extremes

        let score = scorer.calculate_score(&user1, &user2);

        // |1-7| × 1.5 × 1.5 = 6 × 2.25 = 13.5
        assert_eq!(score, 13.5);
    }

    #[test]
    fn test_polarization_moderate_vs_moderate() {
        // Both users have moderate positions (3 vs 5)
        let user1 = User::new("user1".to_string(), vec![3]).unwrap();
        let user2 = User::new("user2".to_string(), vec![5]).unwrap();

        let scorer = PolarizationScorer::default(); // 1.0x for moderates

        let score = scorer.calculate_score(&user1, &user2);

        // |3-5| × 1.0 × 1.0 = 2.0
        assert_eq!(score, 2.0);
    }

    #[test]
    fn test_polarization_extreme_vs_moderate() {
        // One extreme, one moderate
        let user1 = User::new("user1".to_string(), vec![1]).unwrap();
        let user2 = User::new("user2".to_string(), vec![4]).unwrap();

        let scorer = PolarizationScorer::default(); // 1.5x for extreme, 1.0x for moderate

        let score = scorer.calculate_score(&user1, &user2);

        // |1-4| × 1.5 × 1.0 = 3 × 1.5 = 4.5
        assert_eq!(score, 4.5);
    }

    #[test]
    fn test_polarization_lean_positions() {
        // Both lean positions (2 vs 6)
        let user1 = User::new("user1".to_string(), vec![2]).unwrap();
        let user2 = User::new("user2".to_string(), vec![6]).unwrap();

        let scorer = PolarizationScorer::default(); // 1.2x for lean

        let score = scorer.calculate_score(&user1, &user2);

        // |2-6| × 1.2 × 1.2 = 4 × 1.44 = 5.76
        assert_eq!(score, 5.76);
    }

    #[test]
    fn test_polarization_emphasizes_passion() {
        // Scenario A: Large difference but both moderate (3 vs 5)
        let user_a1 = User::new("a1".to_string(), vec![3]).unwrap();
        let user_a2 = User::new("a2".to_string(), vec![5]).unwrap();

        // Scenario B: Same numeric difference but both extreme (1 vs 3)
        let user_b1 = User::new("b1".to_string(), vec![1]).unwrap();
        let user_b2 = User::new("b2".to_string(), vec![3]).unwrap();

        let scorer = PolarizationScorer::default();

        let score_a = scorer.calculate_score(&user_a1, &user_a2);
        let score_b = scorer.calculate_score(&user_b1, &user_b2);

        // Scenario A: |3-5| × 1.0 × 1.0 = 2.0
        // Scenario B: |1-3| × 1.5 × 1.0 = 3.0
        assert_eq!(score_a, 2.0);
        assert_eq!(score_b, 3.0);
        assert!(score_b > score_a, "Extreme position should score higher");
    }

    #[test]
    fn test_polarization_vs_simple_difference() {
        // Compare on same data
        let user1 = User::new("user1".to_string(), vec![1, 7, 4]).unwrap();
        let user2 = User::new("user2".to_string(), vec![7, 1, 4]).unwrap();

        let polar = PolarizationScorer::default();
        let simple = SimpleDifferenceScorer;

        let polar_score = polar.calculate_score(&user1, &user2);
        let simple_score = simple.calculate_score(&user1, &user2);

        // Q1: |1-7| = 6 → Simple: 6, Polar: 6×1.5×1.5 = 13.5
        // Q2: |7-1| = 6 → Simple: 6, Polar: 6×1.5×1.5 = 13.5
        // Q3: |4-4| = 0 → Simple: 0, Polar: 0
        // Simple total: 12
        // Polar total: 27.0

        assert_eq!(simple_score, 12.0);
        assert_eq!(polar_score, 27.0);
        assert!(
            polar_score > simple_score,
            "Polarization should emphasize extreme disagreements"
        );
    }

    #[test]
    fn test_polarization_custom_multipliers() {
        // Create scorer with heavy extreme emphasis
        let scorer = PolarizationScorer::new(3.0, 1.5, 1.0);

        let user1 = User::new("user1".to_string(), vec![1]).unwrap();
        let user2 = User::new("user2".to_string(), vec![7]).unwrap();

        let score = scorer.calculate_score(&user1, &user2);

        // |1-7| × 3.0 × 3.0 = 6 × 9 = 54
        assert_eq!(score, 54.0);
    }

    #[test]
    fn test_polarization_pattern_matching_coverage() {
        // Test all answer ranges
        let scorer = PolarizationScorer::default();

        // Test answer 1 (extreme)
        let u1 = User::new("1".to_string(), vec![1]).unwrap();
        let u7 = User::new("7".to_string(), vec![7]).unwrap();
        assert_eq!(scorer.calculate_score(&u1, &u7), 13.5); // 6×1.5×1.5

        // Test answer 2 (lean)
        let u2 = User::new("2".to_string(), vec![2]).unwrap();
        let u6 = User::new("6".to_string(), vec![6]).unwrap();
        assert_eq!(scorer.calculate_score(&u2, &u6), 5.76); // 4×1.2×1.2

        // Test answer 3,4,5 (moderate)
        let u3 = User::new("3".to_string(), vec![3]).unwrap();
        let u4 = User::new("4".to_string(), vec![4]).unwrap();
        let u5 = User::new("5".to_string(), vec![5]).unwrap();

        assert_eq!(scorer.calculate_score(&u3, &u5), 2.0); // 2×1.0×1.0
        assert_eq!(scorer.calculate_score(&u4, &u4), 0.0); // 0 (identical)
    }

    // ========================================================================
    // Comprehensive Strategy Comparison
    // ========================================================================

    #[test]
    fn test_all_strategies_comparison() {
        // Create test scenarios that highlight strategy differences

        println!("\n=== Scoring Strategy Comparison ===\n");

        // Scenario 1: Strong disagreement on everything
        println!("Scenario 1: Maximum opposition (user answers all 1s vs all 7s)");
        let user_max_1 = User::new("max1".to_string(), vec![1, 1, 1, 1, 1]).unwrap();
        let user_max_2 = User::new("max2".to_string(), vec![7, 7, 7, 7, 7]).unwrap();

        let simple = SimpleDifferenceScorer;
        let euclidean = EuclideanDistanceScorer;
        let weighted = WeightedScorer::equal_weights(5);
        let polar = PolarizationScorer::default();

        println!("  Simple:      {}", simple.calculate_score(&user_max_1, &user_max_2));
        println!("  Euclidean:   {:.2}", euclidean.calculate_score(&user_max_1, &user_max_2));
        println!("  Weighted:    {}", weighted.calculate_score(&user_max_1, &user_max_2));
        println!("  Polarization: {:.2}", polar.calculate_score(&user_max_1, &user_max_2));

        // Scenario 2: Moderate disagreement across the board
        println!("\nScenario 2: Moderate opposition (consistent difference of 2 points)");
        let user_mod_1 = User::new("mod1".to_string(), vec![2, 2, 2, 2, 2]).unwrap();
        let user_mod_2 = User::new("mod2".to_string(), vec![4, 4, 4, 4, 4]).unwrap();

        println!("  Simple:       {}", simple.calculate_score(&user_mod_1, &user_mod_2));
        println!("  Euclidean:    {:.2}", euclidean.calculate_score(&user_mod_1, &user_mod_2));
        println!("  Weighted:     {}", weighted.calculate_score(&user_mod_1, &user_mod_2));
        println!("  Polarization: {:.2}", polar.calculate_score(&user_mod_1, &user_mod_2));

        // Scenario 3: One big disagreement vs many small ones
        println!("\nScenario 3A: ONE large disagreement + 4 agreements");
        let user_one_big_1 = User::new("big1".to_string(), vec![1, 4, 4, 4, 4]).unwrap();
        let user_one_big_2 = User::new("big2".to_string(), vec![7, 4, 4, 4, 4]).unwrap();

        println!("  Simple:       {}", simple.calculate_score(&user_one_big_1, &user_one_big_2));
        println!("  Euclidean:    {:.2}", euclidean.calculate_score(&user_one_big_1, &user_one_big_2));
        println!("  Weighted:     {}", weighted.calculate_score(&user_one_big_1, &user_one_big_2));
        println!("  Polarization: {:.2}", polar.calculate_score(&user_one_big_1, &user_one_big_2));

        println!("\nScenario 3B: MANY small disagreements (diff=2 on all 5)");
        let user_many_small_1 = User::new("small1".to_string(), vec![3, 3, 3, 3, 3]).unwrap();
        let user_many_small_2 = User::new("small2".to_string(), vec![5, 5, 5, 5, 5]).unwrap();

        println!("  Simple:       {}", simple.calculate_score(&user_many_small_1, &user_many_small_2));
        println!("  Euclidean:    {:.2}", euclidean.calculate_score(&user_many_small_1, &user_many_small_2));
        println!("  Weighted:     {}", weighted.calculate_score(&user_many_small_1, &user_many_small_2));
        println!("  Polarization: {:.2}", polar.calculate_score(&user_many_small_1, &user_many_small_2));

        // Scenario 4: Extreme vs moderate (passion asymmetry)
        println!("\nScenario 4: Passionate person vs indifferent person");
        let user_passionate = User::new("passion".to_string(), vec![1, 1, 7, 7, 1]).unwrap();
        let user_moderate = User::new("moderate".to_string(), vec![4, 4, 4, 4, 4]).unwrap();

        println!("  Simple:       {}", simple.calculate_score(&user_passionate, &user_moderate));
        println!("  Euclidean:    {:.2}", euclidean.calculate_score(&user_passionate, &user_moderate));
        println!("  Weighted:     {}", weighted.calculate_score(&user_passionate, &user_moderate));
        println!("  Polarization: {:.2}", polar.calculate_score(&user_passionate, &user_moderate));

        println!("\n=== Key Insights ===");
        println!("Simple:       Treats all differences equally");
        println!("Euclidean:    Emphasizes large differences over many small ones");
        println!("Weighted:     Allows custom importance per question");
        println!("Polarization: Rewards passionate disagreement, penalizes apathy");
        println!();

        // Assertions to verify behavior
        let max_simple = simple.calculate_score(&user_max_1, &user_max_2);
        let max_polar = polar.calculate_score(&user_max_1, &user_max_2);
        assert!(max_polar > max_simple, "Polarization should amplify extreme disagreements");

        // For truly moderate disagreements, use 3-5 range
        let truly_mod_1 = User::new("m1".to_string(), vec![3, 3, 3, 3, 3]).unwrap();
        let truly_mod_2 = User::new("m2".to_string(), vec![5, 5, 5, 5, 5]).unwrap();
        let truly_mod_simple = simple.calculate_score(&truly_mod_1, &truly_mod_2);
        let truly_mod_polar = polar.calculate_score(&truly_mod_1, &truly_mod_2);
        assert_eq!(
            truly_mod_polar, truly_mod_simple,
            "Polarization should equal simple for moderate-only disagreements (1.0x multiplier)"
        );
    }
}
