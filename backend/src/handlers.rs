use axum::extract::{Query, State};
use axum::Json;

use shared::question::{CreateQuestion, GetQuestionById, Question, QuestionId};

use crate::db::AppDatabase;
use crate::http_error::{AppError, QuestionError};

// Basic hello-world route
pub async fn root() -> String {
    "Hello World!".into()
}

pub async fn get_questions(
    State(am_database): State<AppDatabase>,
) -> Result<Json<Vec<Question>>, AppError> {
    let questions = am_database.questions.lock().expect("Poisoned mutex");

    let all_questions: Vec<Question> = (*questions).clone();

        Ok(Json(all_questions))
}

pub async fn get_question_by_id(
    State(am_database): State<AppDatabase>,
    Query(query): Query<GetQuestionById>,
) -> Result<Json<Question>, AppError> {
    let questions = am_database.questions.lock().expect("Poisoned mutex");

    let result_question = (*questions).iter().find(|&q| q.id.0 == query.question_id);

    if let Some(question) = result_question {
        Ok(Json(question.clone()))
    } else {
        Err(AppError::Question(QuestionError::InvalidId))
    }
}

pub async fn create_question(
    State(am_database): State<AppDatabase>,
    Json(question): Json<CreateQuestion>,
) -> Result<Json<Question>, AppError> {
    let mut questions = am_database.questions.lock().expect("Poisoned mutex");
    let db_count = questions.len() as usize;
    let question_with_id = Question::new(
        QuestionId(db_count),
        question.content,
        question.title,
        question.tags,
    );
    // Must clone question to use it later
    (*questions).push(question_with_id.clone());

    // Send back the new question
    Ok(Json(question_with_id))
}

pub async fn delete_question(
    State(am_database): State<AppDatabase>,
    Query(query): Query<GetQuestionById>,
) -> Result<(), AppError> {
    let mut questions = am_database.questions.lock().expect("Poisoned mutex");

    let to_delete = questions.iter().position(|q| q.id.0 == query.question_id).unwrap();
    questions.remove(to_delete);

    Ok(())
}
