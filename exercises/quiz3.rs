// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
//一个假想的魔法学校有一个新的成绩单生成系统//用Rust写的！目前，系统仅支持创建用数字表示//学生成绩的成绩单(例如1.0 -> 5.5)。但是//学校也是按字母顺序发成绩(A+ -> F-)，需要//能打印两种类型的成绩单！////在struct ReportCard和impl块中进行必要的代码更改//以支持按字母顺序排列的报告卡。将第二次测试中的成绩改为//“A+”，以表明您的更改允许按字母排序的成绩。
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

// I AM DONE

use std::fmt::Display; // 导入特征
pub struct ReportCard<T> { // T:String,f32
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T> ReportCard<T> where T:Display { // 由于是编译型语言，需要为泛型T限制Display特征（其余还有default等等）
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard { // 无须指定<f32/&str>，可以自行推断
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
