use std::io;

struct People {
    people: Vec<Person>,
}

struct Person {
    male: u64,
    female: u64,
    parent: u64,
    rank: u64,
}

impl Person {
    fn new_male(identifier: u64) -> Self {
        Person {
            male: 1,
            female: 0,
            parent: identifier,
            rank: 1,
        }
    }
    fn new_female(identifier: u64) -> Self {
        Person {
            male: 0,
            female: 1,
            parent: identifier,
            rank: 1,
        }
    }
}

impl People {
    fn new() -> Self {
        People { people: Vec::new() }
    }
    fn add_male(&mut self, identifier: u64) {
        self.people.push(Person::new_male(identifier));
    }
    fn add_female(&mut self, identifier: u64) {
        self.people.push(Person::new_female(identifier));
    }

    fn find(&mut self, a: usize) -> u64 {
        if self.people[a].parent != a as u64 {
            self.people[a].parent = self.find(self.people[a].parent as usize);
        }
        self.people[a].parent
    }
    fn union(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }
        let aroot = self.find(a) as usize;
        let broot = self.find(b) as usize;
        if self.people[aroot].rank < self.people[broot].rank {
            self.people[aroot].parent = broot as u64;
            self.people[broot].rank += self.people[aroot].rank;
            self.people[broot].female += self.people[aroot].female;
            self.people[broot].male += self.people[aroot].male;
        } else {
            self.people[broot].parent = aroot as u64;
            self.people[aroot].rank += self.people[broot].rank;
            self.people[aroot].female += self.people[broot].female;
            self.people[aroot].male += self.people[broot].male;
        }
    }
}

fn marriage() {
    let n = read_input();
    let num_m = n[0];
    let num_f = n[1];
    let n = num_m + num_f;
    let mut ppl = People::new();
    for i in 0..n {
        if i < num_m {
            ppl.add_male(i);
        } else {
            ppl.add_female(i);
        }
    }
    let q1 = read_input()[0];

    for _i in 0..q1 {
        let ipt = read_input();
        let x = ppl.find(ipt[0] as usize - 1) as usize;
        let y = ppl.find(ipt[1] as usize - 1) as usize;
        ppl.union(x, y);
    }
    let q2 = read_input()[0];
    for _i in 0..q2 {
        let ipt = read_input();
        let x = ppl.find((num_m + ipt[0] - 1) as usize);
        let y = ppl.find((num_m + ipt[1] - 1) as usize);
        ppl.union(x as usize, y as usize);
    }
    let q3 = read_input()[0];
    for _i in 0..q3 {
        let ipt = read_input();
        let x = ppl.find((ipt[0] - 1) as usize);
        let y = ppl.find((num_m + ipt[1] - 1) as usize);
        ppl.union(x as usize, y as usize);
    }
    let mut ans = 0u64;
    for i in 0..num_m {
        let tmp = ppl.find(i as usize);
        ans += num_f - ppl.people[tmp as usize].female;
    }
    println!("{}", ans);
}

fn read_input() -> Vec<u64> {
    let mut ipt = String::new();
    io::stdin().read_line(&mut ipt).expect("Error");
    ipt.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>()
}
fn main() {
    marriage();
}
