pub trait Controller {
    fn next_action(&self) -> Action;
}

pub enum PlayerAction {
    Move(Dir),
    Attack(Dir)
}

pub struct Entity<C: Controller> {
    controller: C,

    species: Species,

    max_hp: u32,
    hp: u32,

    max_sp: u32,
    sp: u32,

    max_mp: u32,
    mp: u32,

    strength: u32,
    dexterity: u32,
    intelligence: u32,

    equipment: Vec<EquipmentSlot>,
    inventory: Vec<Item>,
    spells: Vec<Spell>,
    abilities: Option<Ability>,
}

pub struct BasicEntity<C: Controller> {
    controller: C,


}

pub enum Species {
    Human,
    Orc
}

pub enum Size {
    Tiny,
    VerySmall,
    Small,
    Average,
    Large,
    VeryLarge,
    Huge
}

pub enum Action {
    // Basic actions
    Walk(Dir),
    Sprint(Dir),
    BeginCrouch,
    EndCrouch,
    Wait,
    Rest,

    // Combat
    MeleeAttack(Dir),
    RangedAttack(Target),
    Cast(Spell, Target),

    // Special abilities
    UseAbility(Ability, Target),

    // Item usage
    Throw(Throwable, Target),
    Evoke(Evokable, Target),
    Use(Consumable),

    // Equipment
    Equip(Equippable),
    Unequip(Equippable),

    // Inventory management
    Grab(Item),
    Drop(Item)
}

pub enum Dir { N, NE, NW, E, S, SE, SW, W }
