use dashmap::{DashMap, DashSet};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::grade::Grade, routes::academics::Sections};

/// Struct to store the subjects data in cache
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SubjectCache {
    pub uuid: Uuid,
    pub name: String,
    pub evaluation_type: EvaluationType,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum EvaluationType {
    Marks,
    Grades,
}

impl SubjectCache {
    pub fn new(uuid: Uuid, name: &str, evaluation_type: &str) -> Self {
        Self {
            uuid,
            name: name.to_string(),
            evaluation_type: evaluation_type
                .try_into()
                .expect("Failed to create evaluation type"),
        }
    }
}

impl TryFrom<&str> for EvaluationType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "MARKS" => Ok(Self::Marks),
            "GRADE" => Ok(Self::Grades),
            _ => Err(()),
        }
    }
}

/// Struct to store the exam_type data in cache
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ExamTypesCache {
    pub uuid: Uuid,
    pub name: String,
}

impl ExamTypesCache {
    pub fn new(uuid: Uuid, name: &str) -> Self {
        Self {
            uuid,
            name: name.to_string(),
        }
    }
}

/// It holds all the cache at the start of the application from db that won't change while
/// application is running
#[derive(Debug)]
pub struct AppCache {
    pub grades: GradesCache,
    pub terms: DashMap<String, Uuid>,
    pub sections: DashMap<String, Uuid>,
    pub exam_types: DashMap<String, ExamTypesCache>,
    pub subjects: DashMap<String, SubjectCache>,
    pub grade_subjects: DashMap<Uuid, Vec<Uuid>>,
    pub grade_terms: DashSet<(Uuid, Uuid, Uuid)>,
    pub max_marks: DashMap<(Uuid, Uuid), i32>,
}

/// It holds the Grades cache to search both way by id of grade to get name and by name to get id
#[derive(Debug)]
pub struct GradesCache {
    pub by_name: DashMap<String, Uuid>,
    pub by_id: DashMap<Uuid, String>,
}

impl GradesCache {
    pub async fn new(pool: &PgPool) -> Result<Self, Box<dyn std::error::Error>> {
        let grades = sqlx::query!(
            r#"
            SELECT name, id 
            FROM grades
            ORDER BY sort_order
            "#
        )
        .fetch_all(pool)
        .await?;

        let grades_by_name: DashMap<String, Uuid> = grades
            .iter()
            .map(|val| (val.name.clone(), val.id))
            .collect();

        let grades_by_id: DashMap<Uuid, String> = grades
            .iter()
            .map(|val| (val.id, val.name.clone()))
            .collect();

        Ok(Self {
            by_name: grades_by_name,
            by_id: grades_by_id,
        })
    }
}

impl AppCache {
    pub async fn new(pool: &PgPool) -> Result<Self, Box<dyn std::error::Error>> {
        let grades = GradesCache::new(&pool).await?;

        let terms = sqlx::query!(
            r#"
            SELECT name, id 
            FROM terms
            ORDER BY sort_order
            "#
        )
        .fetch_all(pool)
        .await?;

        let terms: DashMap<String, Uuid> =
            terms.into_iter().map(|term| (term.name, term.id)).collect();

        let sections = sqlx::query!(
            r#"
            SELECT name, id 
            FROM sections 
            ORDER BY sort_order
            "#
        )
        .fetch_all(pool)
        .await?;

        let sections: DashMap<String, Uuid> =
            sections.into_iter().map(|val| (val.name, val.id)).collect();

        let subjects = sqlx::query!(
            r#"
            SELECT name, id, code, evaluation_type 
            FROM subjects 
            "#
        )
        .fetch_all(pool)
        .await?;

        let subjects: DashMap<String, SubjectCache> = subjects
            .into_iter()
            .map(|val| {
                let subject_cache = SubjectCache::new(val.id, &val.name, &val.evaluation_type);
                (val.code, subject_cache)
            })
            .collect();

        let exam_types = sqlx::query!(
            r#"
            SELECT name, id, code 
            FROM exam_types 
            "#
        )
        .fetch_all(pool)
        .await?;

        let exam_types: DashMap<String, ExamTypesCache> = exam_types
            .into_iter()
            .map(|val| {
                let exam_types_cache = ExamTypesCache::new(val.id, &val.name);
                (val.code, exam_types_cache)
            })
            .collect();

        let grade_subjects_rows = sqlx::query!(
            r#"
            SELECT grade_id, subject_id 
            FROM grade_subjects 
            "#
        )
        .fetch_all(pool)
        .await?;

        let grade_subjects: DashMap<Uuid, Vec<Uuid>> = DashMap::new();

        for val in grade_subjects_rows {
            grade_subjects
                .entry(val.grade_id)
                .or_default()
                .push(val.subject_id);
        }

        let grade_terms = sqlx::query!(
            r#"
            SELECT grade_id, exam_type_id, term_id 
            FROM grade_terms 
            "#
        )
        .fetch_all(pool)
        .await?;

        let grade_terms: DashSet<(Uuid, Uuid, Uuid)> = grade_terms
            .into_iter()
            .map(|val| (val.grade_id, val.exam_type_id, val.term_id))
            .collect();

        let max_marks = sqlx::query!(
            r#"
            SELECT grade_id, exam_type_id, max_marks 
            FROM max_marks
            "#
        )
        .fetch_all(pool)
        .await?;

        let max_marks: DashMap<(Uuid, Uuid), i32> = max_marks
            .into_iter()
            .map(|val| ((val.grade_id, val.exam_type_id), val.max_marks))
            .collect();

        Ok(Self {
            grades,
            terms,
            sections,
            exam_types,
            subjects,
            grade_subjects,
            grade_terms,
            max_marks,
        })
    }

    /// Returns the grade id using grade name from the grades cache
    pub fn get_grade_id(&self, grade_name: &Grade) -> Option<Uuid> {
        Some(
            self.grades
                .by_name
                .get(grade_name.as_ref())?
                .value()
                .clone(),
        )
    }

    /// Returns the grade name using the given Uuid from the grades cache
    pub fn get_grade_name(&self, grade_id: &Uuid) -> Option<String> {
        Some(self.grades.by_id.get(grade_id)?.value().clone())
    }

    /// Returns the section_id from the grades cache
    pub fn get_section_id(&self, section_name: &Sections) -> Option<Uuid> {
        Some(self.sections.get(section_name.as_ref())?.value().clone())
    }
}
