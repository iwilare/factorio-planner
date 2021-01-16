struct Item<'a> {
    name: &'a str,
    factory: &'a Factory<'a>,
    count: i32,
    time: f64,
    components: &'a [(i32, &'a Item<'a>)],
}

struct Factory<'a> {
    name: &'a str,
    crafting_speed: f64,
    is_assembly: bool,
}

#[derive(Clone)]
struct Production<'a> {
    target: &'a Item<'a>,
    output_per_second: f64,
    location: Vec<&'a Item<'a>>,
}

impl Item<'_> {
    fn output_per_second(&self) -> f64 {
        self.factory.crafting_speed * self.count as f64 / self.time
    }
    fn required_machines(&self, required_output_per_second: f64) -> f64 {
        required_output_per_second / self.output_per_second()
    }
}

const EXTR : Factory = Factory { name: "mining drill",         crafting_speed: 0.5,  is_assembly: false };
const UEXT : Factory = Factory { name: "uranium drill",        crafting_speed: 0.25, is_assembly: false };
const FURN : Factory = Factory { name: "furnace",              crafting_speed: 1.0,  is_assembly: false };
const EFUR : Factory = Factory { name: "electric furnace",     crafting_speed: 2.0,  is_assembly: false };
const PUMP : Factory = Factory { name: "pumpjack",             crafting_speed: 1.0,  is_assembly: false };
const CHEM : Factory = Factory { name: "chemical plant",       crafting_speed: 1.0,  is_assembly: false };
const OILR : Factory = Factory { name: "oil refinery",         crafting_speed: 1.0,  is_assembly: false };
const ASM1 : Factory = Factory { name: "assembling machine 1", crafting_speed: 0.5,  is_assembly: true  };
const ASM2 : Factory = Factory { name: "assembling machine 2", crafting_speed: 0.75, is_assembly: true  };
const ASM3 : Factory = Factory { name: "assembling machine 3", crafting_speed: 1.25, is_assembly: true  };

const ASM_ : Factory = ASM1;

const IRONORE   : Item = Item { name: "iron ore",         factory: &EXTR, count: 1,  time: 1.0,  components: &[] };
const COPPERORE : Item = Item { name: "copper ore",       factory: &EXTR, count: 1,  time: 1.0,  components: &[] };
const COAL      : Item = Item { name: "coal",             factory: &EXTR, count: 1,  time: 1.0,  components: &[] };
const IRON      : Item = Item { name: "iron",             factory: &EFUR, count: 1,  time: 1.0,  components: &[(1, &IRONORE)] };
const COPPER    : Item = Item { name: "copper",           factory: &EFUR, count: 1,  time: 1.0,  components: &[(1, &COPPERORE)] };
const STONE     : Item = Item { name: "stone",            factory: &EFUR, count: 1,  time: 0.5,  components: &[] };
const LUBRICANT : Item = Item { name: "lubricant",        factory: &CHEM, count: 10, time: 1.0,  components: &[] };
const PLASTIC   : Item = Item { name: "plastic",          factory: &CHEM, count: 2,  time: 0.5,  components: &[] };
const ACID      : Item = Item { name: "acid",             factory: &CHEM, count: 50, time: 1.0,  components: &[] };
const SULFUR    : Item = Item { name: "sulfur",           factory: &CHEM, count: 2,  time: 1.0,  components: &[] };
const STEEL     : Item = Item { name: "steel",            factory: &EFUR, count: 1,  time: 16.0, components: &[(5,  &IRON)] };

const GEAR      : Item = Item { name: "gear",             factory: &ASM_, count: 1,  time: 0.5,  components: &[(2,  &IRON)] };
const SP1       : Item = Item { name: "sp red",           factory: &ASM_, count: 1,  time: 5.0,  components: &[(1,  &COPPER), (1, &GEAR)] };
const CABLE     : Item = Item { name: "cable",            factory: &ASM_, count: 2,  time: 0.5,  components: &[(1,  &COPPER)] };
const CIRCUIT1  : Item = Item { name: "circuit1",         factory: &ASM_, count: 1,  time: 0.5,  components: &[(3,  &CABLE), (1, &IRON)] };
const INSERTER  : Item = Item { name: "inserter",         factory: &ASM_, count: 1,  time: 0.5,  components: &[(1,  &CIRCUIT1), (1, &GEAR), (1, &IRON)] };
const CIRCUIT2  : Item = Item { name: "circuit2",         factory: &ASM_, count: 1,  time: 6.0,  components: &[(4,  &CABLE), (2, &CIRCUIT1), (2, &PLASTIC)] };
const BELT1     : Item = Item { name: "belt1",            factory: &ASM_, count: 1,  time: 0.5,  components: &[(1,  &GEAR), (1, &IRON)] };
const SP2       : Item = Item { name: "sp green",         factory: &ASM_, count: 1,  time: 6.0,  components: &[(1,  &INSERTER), (1, &BELT1)] };
const PIPE      : Item = Item { name: "pipe",             factory: &ASM_, count: 1,  time: 0.5,  components: &[(1,  &IRON)] };
const ENGINE    : Item = Item { name: "engine",           factory: &ASM_, count: 1,  time: 10.0, components: &[(1,  &GEAR), (2, &PIPE), (1, &STEEL)] };
const SP3       : Item = Item { name: "sp blue",          factory: &ASM_, count: 2,  time: 24.0, components: &[(3,  &CIRCUIT2), (2, &ENGINE), (1, &SULFUR)] };
const STICK     : Item = Item { name: "stick",            factory: &ASM_, count: 2,  time: 0.5,  components: &[(1,  &IRON)] };
const RAIL      : Item = Item { name: "rail",             factory: &ASM_, count: 2,  time: 0.5,  components: &[(1,  &STICK), (1, &STEEL), (1, &STONE)] };
const PROD      : Item = Item { name: "productivity mod", factory: &ASM_, count: 1,  time: 15.0, components: &[(5,  &CIRCUIT1), (5, &CIRCUIT2)] };
const BRICK     : Item = Item { name: "brick",            factory: &ASM_, count: 1,  time: 3.2,  components: &[(2,  &STONE)] };
const EFURNACE  : Item = Item { name: "electric furnace", factory: &ASM_, count: 1,  time: 0.5,  components: &[(10, &BRICK), (10, &STEEL), (5, &CIRCUIT2)] };
const SP4       : Item = Item { name: "sp purple",        factory: &ASM_, count: 3,  time: 21.0, components: &[(30, &RAIL), (1, &EFURNACE), (1, &PROD)] };
const GRENADE   : Item = Item { name: "grenade",          factory: &ASM_, count: 1,  time: 8.0,  components: &[(10, &COAL), (5, &IRON)] };
const WALL      : Item = Item { name: "wall",             factory: &ASM_, count: 1,  time: 0.5,  components: &[(5,  &BRICK)] };
const MAGAZINE  : Item = Item { name: "magazine",         factory: &ASM_, count: 1,  time: 1.0,  components: &[(4,  &IRON)] };
const PIERCING  : Item = Item { name: "piercing",         factory: &ASM_, count: 1,  time: 3.0,  components: &[(5,  &COPPER), (1, &MAGAZINE), (1, &STEEL)] };
const SPM       : Item = Item { name: "sp black" ,        factory: &ASM_, count: 2,  time: 10.0, components: &[(2,  &WALL), (1, &GRENADE), (1, &PIERCING)] };
const EENGINE   : Item = Item { name: "electric engine",  factory: &ASM_, count: 1,  time: 10.0, components: &[(15, &LUBRICANT), (2, &CIRCUIT1), (1, &ENGINE)] };
const BATTERY   : Item = Item { name: "battery",          factory: &ASM_, count: 1,  time: 4.0,  components: &[(20, &ACID), (1, &COPPER), (1, &IRON)] };
const ROBOT     : Item = Item { name: "robot",            factory: &ASM_, count: 1,  time: 20.0, components: &[(3,  &CIRCUIT1), (2, &BATTERY), (1, &EENGINE), (1, &STEEL)] };
const LDS       : Item = Item { name: "lds",              factory: &ASM_, count: 1,  time: 20.0, components: &[(20, &COPPER), (5, &PLASTIC), (2, &STEEL)] };
const CIRCUIT3  : Item = Item { name: "circuit3",         factory: &ASM_, count: 1,  time: 10.0, components: &[(20, &CIRCUIT1), (5, &ACID), (2, &CIRCUIT2)] };
const SPEED     : Item = Item { name: "speed mod",        factory: &ASM_, count: 1,  time: 15.0, components: &[(5,  &CIRCUIT2), (5, &CIRCUIT1)] };
const SP5       : Item = Item { name: "sp yellow",        factory: &ASM_, count: 3,  time: 21.0, components: &[(3,  &LDS), (2, &CIRCUIT3), (1, &ROBOT)] };

fn solve<'a>(target: &'a Item, output_per_second: f64) -> Vec<Production<'a>> {
    target.components
          .iter()
          .flat_map(|&(required, item)| solve(item, required as f64 * output_per_second / target.count as f64))
          .map(|mut p| { p.location.push(target); p })
          .chain(core::iter::once(Production { target, output_per_second, location: vec![] }))
          .collect()
}

fn print_production<'a>(result: impl Iterator<Item = &'a Production<'a>>) {
    println!("{: <16} | {: >10} | {: >12} | {: <50}", "Product", "Factories", "Production/s", "Destination");
    println!("{:-<16}-+-{:->10}-+-{:->12}-+-{:-<50}", "", "", "", "");
    for Production { target, output_per_second, location } in result {
        let location = location.iter().map(|i| i.name).rev().collect::<Vec<_>>().join(" ← ");
        println!("{: <16} | {: >10.3} | {: >12.3} | {: <50}", target.name, target.required_machines(*output_per_second), output_per_second, location);
    }
}

fn print_global_production(result: &Vec<Production> ) {
    let mut sorted_result = result.clone();
    sorted_result.sort_by_key(|p| p.target.name);
    print_production(sorted_result.iter().filter(|p| !p.target.factory.is_assembly));
    println!("{: <16}   {: >10}   {: >12}   {: <50}", "", "", "", "");
    print_production(result.iter());
}

fn main() {
    print_global_production(&solve(&SP3, 1200.0/60.0));
}
