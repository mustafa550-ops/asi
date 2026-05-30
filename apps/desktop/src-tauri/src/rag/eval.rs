pub struct RagEval {
    pub precision: f64,
    pub recall: f64,
    pub faithfulness: f64,
    pub context_relevance: f64,
    pub total_queries: u32,
}

pub struct EvalSample {
    pub query: String,
    pub retrieved_sources: Vec<String>,
    pub relevant_sources: Vec<String>,
    pub answer: String,
    pub ground_truth: Option<String>,
}

pub struct EvalFramework;

impl EvalFramework {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, samples: &[EvalSample]) -> RagEval {
        let count = samples.len() as u32;
        if count == 0 {
            return RagEval {
                precision: 0.0, recall: 0.0, faithfulness: 0.0,
                context_relevance: 0.0, total_queries: 0,
            };
        }

        let mut total_precision = 0.0;
        let mut total_recall = 0.0;
        let mut total_faithfulness = 0.0;
        let mut total_relevance = 0.0;

        for sample in samples {
            total_precision += self.calc_precision(&sample.retrieved_sources, &sample.relevant_sources);
            total_recall += self.calc_recall(&sample.retrieved_sources, &sample.relevant_sources);
            total_faithfulness += self.calc_faithfulness(&sample.answer, &sample.retrieved_sources);
            total_relevance += self.calc_context_relevance(&sample.query, &sample.retrieved_sources);
        }

        RagEval {
            precision: total_precision / count as f64,
            recall: total_recall / count as f64,
            faithfulness: total_faithfulness / count as f64,
            context_relevance: total_relevance / count as f64,
            total_queries: count,
        }
    }

    pub fn calc_precision(&self, retrieved: &[String], relevant: &[String]) -> f64 {
        if retrieved.is_empty() { return 0.0; }
        let relevant_set: std::collections::HashSet<&str> = relevant.iter().map(|s| s.as_str()).collect();
        let true_positives = retrieved.iter().filter(|r| relevant_set.contains(r.as_str())).count();
        true_positives as f64 / retrieved.len() as f64
    }

    pub fn calc_recall(&self, retrieved: &[String], relevant: &[String]) -> f64 {
        if relevant.is_empty() { return 0.0; }
        let retrieved_set: std::collections::HashSet<&str> = retrieved.iter().map(|s| s.as_str()).collect();
        let true_positives = relevant.iter().filter(|r| retrieved_set.contains(r.as_str())).count();
        true_positives as f64 / relevant.len() as f64
    }

    pub fn calc_faithfulness(&self, answer: &str, sources: &[String]) -> f64 {
        if sources.is_empty() || answer.is_empty() { return 0.0; }
        let answer_lower = answer.to_lowercase();
        let mut match_count = 0;

        for source in sources {
            let source_lower = source.to_lowercase();
            if answer_lower.contains(&source_lower) || source_lower.contains(&answer_lower) {
                match_count += 1;
            }
        }

        match_count as f64 / sources.len() as f64
    }

    pub fn calc_context_relevance(&self, query: &str, sources: &[String]) -> f64 {
        if sources.is_empty() || query.is_empty() { return 0.0; }
        let query_terms: std::collections::HashSet<&str> = query.split_whitespace().collect();
        if query_terms.is_empty() { return 0.0; }

        let mut total_overlap = 0.0;
        for source in sources {
            let source_terms: std::collections::HashSet<&str> = source.split_whitespace().collect();
            let intersection = query_terms.intersection(&source_terms).count();
            total_overlap += intersection as f64 / query_terms.len() as f64;
        }

        (total_overlap / sources.len() as f64).min(1.0)
    }

    pub fn report(&self, eval: &RagEval) -> String {
        format!(
            "RAG Değerlendirme Raporu\n\
             Toplam Sorgu: {n}\n\
             Precision: {p:.2}\n\
             Recall: {r:.2}\n\
             Faithfulness: {f:.2}\n\
             Context Relevance: {c:.2}",
            n = eval.total_queries,
            p = eval.precision * 100.0,
            r = eval.recall * 100.0,
            f = eval.faithfulness * 100.0,
            c = eval.context_relevance * 100.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(retrieved: Vec<&str>, relevant: Vec<&str>) -> EvalSample {
        EvalSample {
            query: "test".into(),
            retrieved_sources: retrieved.into_iter().map(String::from).collect(),
            relevant_sources: relevant.into_iter().map(String::from).collect(),
            answer: "cevap metni".into(),
            ground_truth: None,
        }
    }

    #[test]
    fn precision_perfect_match() {
        let eval = EvalFramework::new();
        let p = eval.calc_precision(&["a.md", "b.md"].map(String::from), &["a.md", "b.md"].map(String::from));
        assert!((p - 1.0).abs() < 1e-6);
    }

    #[test]
    fn precision_no_match() {
        let eval = EvalFramework::new();
        let p = eval.calc_precision(&["a.md"].map(String::from), &["b.md"].map(String::from));
        assert_eq!(p, 0.0);
    }

    #[test]
    fn recall_perfect_match() {
        let eval = EvalFramework::new();
        let r = eval.calc_recall(&["a.md", "b.md"].map(String::from), &["a.md", "b.md"].map(String::from));
        assert!((r - 1.0).abs() < 1e-6);
    }

    #[test]
    fn recall_partial() {
        let eval = EvalFramework::new();
        let r = eval.calc_recall(&["a.md"].map(String::from), &["a.md", "b.md"].map(String::from));
        assert!((r - 0.5).abs() < 1e-6);
    }

    #[test]
    fn faithfulness_basic() {
        let eval = EvalFramework::new();
        let f = eval.calc_faithfulness("cevap metni", &["cevap".into()]);
        assert!(f > 0.0);
    }

    #[test]
    fn context_relevance_basic() {
        let eval = EvalFramework::new();
        let r = eval.calc_context_relevance("kripto analiz", &["kripto piyasa raporu".into()]);
        assert!(r > 0.0);
    }

    #[test]
    fn evaluate_empty_samples() {
        let eval = EvalFramework::new();
        let result = eval.evaluate(&[]);
        assert_eq!(result.total_queries, 0);
        assert_eq!(result.precision, 0.0);
    }

    #[test]
    fn report_format() {
        let eval = EvalFramework::new();
        let result = RagEval {
            precision: 0.85, recall: 0.75, faithfulness: 0.9,
            context_relevance: 0.8, total_queries: 10,
        };
        let report = eval.report(&result);
        assert!(report.contains("85"));
        assert!(report.contains("75"));
        assert!(report.contains("10"));
    }
}
