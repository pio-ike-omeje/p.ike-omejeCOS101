fn main() {
    use std::io;
    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
    
    let office_admin_roles = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic_roles = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer_roles = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher_roles = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"]; 
       println!("Input your profession:");
    let mut profession_input = String::new();
    io::stdin().read_line(&mut profession_input).expect("Failed to read input.");
    let profession = profession_input.trim();

    println!("Input your role: ");
    let mut role_input = String::new();
    io::stdin().read_line(&mut role_input).expect("Failed to read input.");
    let role = role_input.trim();

    let mut found = false;

    if profession == "Office Admin" {
        let mut i = 0;
        while i < office_admin_roles.len() {
            if office_admin_roles[i] == role {
                println!("Your level is: {}", levels[i]);
                found = true;
                break;
            }
            i += 1;
        }
    } 
    else if profession == "Academic" {
        let mut i = 0;
        while i < academic_roles.len() {
            if academic_roles[i] == role {
                println!("Your level is: {}", levels[i]);
                found = true;
                break;
            }
            i += 1;
        }
    } 
    else if profession == "Lawyer" {
        let mut i = 0;
        while i < lawyer_roles.len() {
            if lawyer_roles[i] == role {
                println!("Your level is: {}", levels[i]);
                found = true;
                break;
            }
            i += 1;
        }
    } 
    else if profession == "Teacher" {
        let mut i = 0;
        while i < teacher_roles.len() {
            if teacher_roles[i] == role {
                println!("Your level is: {}", levels[i]);
                found = true;
                break;
            }
            i += 1;
        }
    }          
if !found {
    println!("Profession or role not found. ");
}
}
