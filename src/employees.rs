use std::collections::HashMap;

type EmployeesMap = HashMap<String, Vec<String>>;

struct Employee {
  name: String,
  dept: String,
}

pub fn run() {
  let mut employees_map: EmployeesMap = HashMap::new();

  let Some(res) = add_employee_to_dept("Add Sally to Engineering", &mut employees_map) else {
    return;
  };

  println!("Response {:#?}", res);

  add_employee_to_dept("Add Amir to Sales", &mut employees_map);
  add_employee_to_dept("Add Fola to Sales", &mut employees_map);
  add_employee_to_dept("Add Kola to Engineering", &mut employees_map);
  add_employee_to_dept("Add Tunji to Marketing", &mut employees_map);
  add_employee_to_dept("Add Kola to Sales", &mut employees_map);

  println!("Employees map {:#?}", employees_map);

  let Some(engineers) = fetch_employees_in_dept("Engineering", &mut employees_map) else {
    return;
  };

  println!("Employees in Engineering: {:#?}", engineers);
  println!("All Employees: {:#?}", fetch_all_employees(&employees_map));
}

/// accepts string of format "Add {name} to {department}"
fn add_employee_to_dept(text: &str, employees_map: &mut EmployeesMap) -> Option<Vec<String>> {
  let Employee { name, dept } = parse_employee_from_text(text)?;

  let from_dept = employees_map.entry(dept).or_insert(vec![]);

  from_dept.push(name);

  Some(from_dept.to_vec())
}

fn parse_employee_from_text(text: &str) -> Option<Employee> {
  let mut words = text.split_whitespace();

  let first = words.next()?;

  if !first.eq_ignore_ascii_case("Add") {
    return None;
  }

  let name = words.next()?.to_string();
  let third = words.next()?;

  if !third.eq_ignore_ascii_case("to") {
    return None;
  }

  let dept: String = words.collect();

  return Some(Employee { name, dept });
}

fn fetch_employees_in_dept(dept: &str, employees_map: &mut EmployeesMap) -> Option<Vec<String>> {
  let employees = employees_map.get(dept)?;

  Some(employees.to_vec())
}

fn fetch_all_employees(employees_map: &EmployeesMap) -> Vec<&String> {
  let all_employees: Vec<&String> = employees_map.values().flatten().collect();

  all_employees
}
