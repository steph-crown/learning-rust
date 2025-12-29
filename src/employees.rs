use std::collections::HashMap;

type EmployeesMap = HashMap<String, Vec<String>>;

struct Employee {
  name: String,
  dept: String,
}

pub struct Company {
  employees_map: EmployeesMap,
}

pub fn run() {
  let mut company = Company {
    employees_map: HashMap::new(),
  };

  company.add_employee_to_dept("Add Sally to Engineering");

  company.add_employee_to_dept("Add Amir to Sales");
  company.add_employee_to_dept("Add Fola to Sales");
  company.add_employee_to_dept("Add Kola to Engineering");
  company.add_employee_to_dept("Add Tunji to Marketing");
  company.add_employee_to_dept("Add Kola to Sales");

  println!("Employees map {:#?}", company.employees_map);

  let Some(engineers) = company.fetch_employees_in_dept("Engineering") else {
    return;
  };

  // let x = company.fetch_employees_in_dept("Engineering").unwrap();

  println!("Employees in Engineering: {:#?}", engineers);
  println!("All Employees: {:#?}", company.fetch_all_employees());
}

impl Company {
  /// accepts string of format "Add {name} to {department}"
  fn add_employee_to_dept(&mut self, text: &str) -> bool {
    let Some(Employee { name, dept }) = parse_employee_from_text(text) else {
      return false;
    };

    let from_dept = self.employees_map.entry(dept).or_insert(vec![]);

    from_dept.push(name);

    true
  }

  fn fetch_employees_in_dept(&self, dept: &str) -> Option<Vec<String>> {
    self.employees_map.get(dept).map(|names| {
      let mut sorted_names = names.clone();
      sorted_names.sort_unstable();

      sorted_names
    })
  }

  fn fetch_all_employees(&self) -> Vec<&String> {
    let mut all_employees: Vec<&String> = self.employees_map.values().flatten().collect();

    all_employees.sort_unstable();

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
