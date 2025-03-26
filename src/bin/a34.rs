// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

struct Luggage<State>{
    userid: i32,
    state: State
}

impl <State> Luggage<State> {
    fn move_item<NextState>(self, next:NextState) -> Luggage<NextState>{
        Luggage { userid: self.userid, state: next }
    }
}



struct CheckIn{}
impl CheckIn {
    fn new(self,userid:i32) -> Luggage<CheckIn>{
        Luggage { userid, state: self }
    }
}

impl Luggage<CheckIn> {
    fn move_to(self, next:OnLoading) -> Luggage<OnLoading>{
        self.move_item(next)
    }
}

struct OnLoading{}
impl Luggage<OnLoading> {
    fn move_to(self, next:Offloading) -> Luggage<Offloading>{
        self.move_item(next)
    }
}

struct Offloading {}
impl Luggage<Offloading> {
    fn move_to(self, next:AwaitingPickup) -> Luggage<AwaitingPickup>{
        self.move_item(next)
    }
}
struct AwaitingPickup{}
impl Luggage<AwaitingPickup> {
    fn move_to(self, next:EndCustody) -> Luggage<EndCustody>{
        self.move_item(next)
    }
}
struct EndCustody {}
impl Luggage<EndCustody> {
    fn collect_item(self) -> Result<bool,String>{
        let collect_succ = false;
        if collect_succ {
            Ok(true)
        } else {
            Err("user did not receive item".to_owned())
        }
    }
}
fn main() {
    let user = CheckIn::new(CheckIn {  }, 1);
    let got_item = user.move_to(OnLoading {  }).move_to(Offloading {  }).move_to(AwaitingPickup {  }).move_to(EndCustody{}).collect_item();
    match got_item {
        Ok(bo) => println!("user receive item successfully"),
        Err(e) => println!("Error: {}", e)
    }
}
