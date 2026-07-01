#![allow(dead_code, unused_variables)]

struct ConsultingWork {
    what: String,
    hours: f32,
    rate: f32,
}

impl Default for ConsultingWork {
    fn default() -> Self {
        Self {
            what: String::from("Rust Consulting"),
            hours: 8.0,
            rate: 100.0,
        }
    }
}

trait Billable {
    // Required
    fn description(&self) -> &str;
    fn total(&self) -> f32;

    // Provided
    fn formatted_total(&self) -> String {
        format!("${:.2} for {}", self.total(), self.description())
    }
}

impl Billable for ConsultingWork {
    fn description(&self) -> &str {
        &self.what
    }

    fn total(&self) -> f32 {
        self.hours * self.rate
    }
}

// Just ad hoc sample regarding methods with identical name
trait AbsTrait { fn abs(self) -> f32; }
impl AbsTrait for f32 {
    fn abs(self) -> f32 { if self < 0.0 { -self } else { self } }
}

impl Billable for f32 {
    fn description(&self) -> &str {
        "Flat fee"
    }

    fn total(&self) -> f32 {
        *self
    }
}

struct FlatFee(f32);

impl Billable for FlatFee {
    fn description(&self) -> &str {
        "Flat fee"
    }

    fn total(&self) -> f32 {
        self.0
    }

    // Assumption: We want to override the default implementation of formatted_total for FlatFee
    fn formatted_total(&self) -> String {
        format!("{} costs ${:.2}", self.description(), self.total())
    }
}

fn print_total(item: &impl Billable) {
    println!("{}", item.formatted_total());
}

/*
fn generate_billable() -> impl Billable {
    if true /* imagine: some runtime condition */ {
        150.0_f32 // returns a flat fee
    } else {
        ConsultingWork::default() // does NOT work with impl
    }
}
*/

fn generate_billable() -> impl Billable {
    150.0_f32 // returns a flat fee
}

fn generate_abs() -> impl AbsTrait {
    -150.0_f32 // returns a flat fee
}

trait Pointworthy {
    fn points(&self) -> u32;
}

impl<T: Billable> Pointworthy for T {
    fn points(&self) -> u32 {
        (self.total() / 10.0) as u32
    }
}

fn generate_pointworthy() -> impl Pointworthy {
    [ConsultingWork::default(), ConsultingWork::default()]
    //[1.0, 2.0, 3.0]
    //ConsultingWork::default()
    //42.0
    //FlatFee(300.0)
}

impl<T: Billable, const C: usize> Billable for [T; C] {
    fn description(&self) -> &str {
        "Fees"
    }

    fn total(&self) -> f32 {
        self.iter().map(|item| item.total()).sum()
    }
}

fn main() {
    let work: ConsultingWork = Default::default();
    print_total(&work);

    let fee: f32 = 200.0;
    print_total(&fee);

    let flat_fee = FlatFee(300.0);
    print_total(&flat_fee);

    let billable = generate_billable();
    print_total(&billable);

    let abs_value = generate_abs();
    println!("Absolute value is {}", abs_value.abs());

    // Demo abs resolution
    let negative_fee: f32 = -150.0;
    println!("Absolute value of {} is {}", negative_fee, negative_fee.abs());
    println!("Absolute value of {} is {}", negative_fee, f32::abs(negative_fee));
    // Use the AbsTrait to call our abs
    let negative_fee_trait: f32 = -150.0;
    println!("Absolute value of {} is {}", negative_fee_trait, AbsTrait::abs(negative_fee_trait));
    println!("Absolute value of {} is {}", negative_fee_trait, <f32 as AbsTrait>::abs(negative_fee_trait));
}
