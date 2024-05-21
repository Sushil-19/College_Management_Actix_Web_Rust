use actix_web::{web, HttpResponse};
use serde_json::json;
use std::sync::Mutex;
use uuid::Uuid;

use crate::models::{CreateStudent, Student};  // Correctly import CreateStudent here

type StudentList = Mutex<Vec<Student>>;

pub async fn get_students(data: web::Data<StudentList>) -> HttpResponse {
    let students = data.lock().unwrap();
    HttpResponse::Ok().json(&*students)
}

pub async fn get_student_by_id(data: web::Data<StudentList>, id: web::Path<Uuid>) -> HttpResponse {
    let students = data.lock().unwrap();
    if let Some(student) = students.iter().find(|student| student.id == *id) {
        HttpResponse::Ok().json(student)
    } else {
        HttpResponse::NotFound().json(json!({"error": "Student not found"}))
    }
}

pub async fn create_student(data: web::Data<StudentList>, new_student: web::Json<CreateStudent>) -> HttpResponse {
    let mut students = data.lock().unwrap();
    let student = Student::new(new_student.name.clone(), new_student.age, new_student.department.clone());
    students.push(student);
    HttpResponse::Created().json(&*students)
}

pub async fn update_student(data: web::Data<StudentList>, id: web::Path<Uuid>, updated_student: web::Json<CreateStudent>) -> HttpResponse {
    let mut students = data.lock().unwrap();
    if let Some(student) = students.iter_mut().find(|student| student.id == *id) {
        student.name = updated_student.name.clone();
        student.age = updated_student.age;
        student.department = updated_student.department.clone();
        HttpResponse::Ok().json(student)
    } else {
        HttpResponse::NotFound().json(json!({"error": "Student not found"}))
    }
}

pub async fn delete_student(data: web::Data<StudentList>, id: web::Path<Uuid>) -> HttpResponse {
    let mut students = data.lock().unwrap();
    if let Some(pos) = students.iter().position(|student| student.id == *id) {
        students.remove(pos);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().json(json!({"error": "Student not found"}))
    }
}
