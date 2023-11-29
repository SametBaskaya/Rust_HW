#[ic_cdk_macros::update]
fn update_exam(key: u64, new_exam_details: Exam) -> Option<Exam> {
    
    let mut exam_storage = storage::get_mut::<HashMap<u64, Exam>>();
    if let Some(existing_exam) = exam_storage.get_mut(&key) {
        
        existing_exam.field1 = new_exam_details.field1;
        existing_exam.field2 = new_exam_details.field2;

        Some(existing_exam.clone())
    } else {
        None
    }
}