use std::collections::HashMap;

type EmployeesMap = HashMap<String, Vec<String>>;

struct Employee {
  name: String,
  dept: String,
}

struct Company {
  employees_map: EmployeesMap,
}

pub fn run() {
  let mut company = Company {
    employees_map: HashMap::new(),
  };

  let Some(res) = company.add_employee_to_dept("Add Sally to Engineering") else {
    return;
  };

  println!("Response {:#?}", res);

  company.add_employee_to_dept("Add Amir to Sales");
  company.add_employee_to_dept("Add Fola to Sales");
  company.add_employee_to_dept("Add Kola to Engineering");
  company.add_employee_to_dept("Add Tunji to Marketing");
  company.add_employee_to_dept("Add Kola to Sales");

  println!("Employees map {:#?}", company.employees_map);

  let Some(engineers) = company.fetch_employees_in_dept("Engineering") else {
    return;
  };

  println!("Employees in Engineering: {:#?}", engineers);
  println!("All Employees: {:#?}", company.fetch_all_employees());
}

impl Company {
  /// accepts string of format "Add {name} to {department}"
  fn add_employee_to_dept(self: &mut Self, text: &str) -> Option<Vec<String>> {
    let Employee { name, dept } = parse_employee_from_text(text)?;

    let from_dept = self.employees_map.entry(dept).or_insert(vec![]);

    from_dept.push(name);

    Some(from_dept.to_vec())
  }

  fn fetch_employees_in_dept(self: &Self, dept: &str) -> Option<Vec<String>> {
    let employees = self.employees_map.get(dept)?;

    Some(employees.to_vec())
  }

  fn fetch_all_employees(self: &Self) -> Vec<&String> {
    let all_employees: Vec<&String> = self.employees_map.values().flatten().collect();

    all_employees
  }
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
