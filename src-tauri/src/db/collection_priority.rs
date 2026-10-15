// Higher value PriorityItems are higher priority, ie. evaluated sooner
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum PriorityItem {
    // Lowest priority
    Language,
    ChoiceSelection,
    Spell,
    MagicTalent,
    TraditionTalent,
    PickTradition,
    PathTalent,
    Score,
    // Highest priority
}

#[derive(PartialEq, PartialOrd, Eq, Clone)]
pub struct CollectionPriority {
    level: i64,
    items: Vec<PriorityItem>,
}

impl CollectionPriority {
    pub fn new(level: i64, init: PriorityItem) -> Self {
        Self {
            level,
            items: vec![init],
        }
    }

    pub fn next(&self, item: PriorityItem) -> Self {
        let mut new_item = self.clone();
        new_item.items.push(item);
        new_item
    }
}

// The lower level,
impl Ord for CollectionPriority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Lower level has higher priority, and is the most important factor
        if let Some(ord) = self.level.partial_cmp(&other.level) {
            return ord.reverse();
        }

        for (s, o) in self.items.iter().zip(other.items.iter()) {
            if let Some(ord) = s.partial_cmp(o) {
                return ord;
            }
        }

        // If we get here, then the items from both self and other are equal up to
        // the index of the shorter element, which means the only difference is that
        // one items vec is longer than the other. In this case, the shorter vec has
        // the higher priority, so the ordering is reversed.
        return self.items.len().cmp(&other.items.len()).reverse();
    }
}
