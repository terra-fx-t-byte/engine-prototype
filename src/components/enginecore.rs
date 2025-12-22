#[allow(unused)]
use std::collections::HashMap;
#[allow(unused)]
use std::any::{Any, TypeId};

#[allow(unused)]
type Entity = u32;

#[allow(unused)]
#[allow(nonstandard_style)]
struct Universe{
    entities: Vec<Entity>,
    components: HashMap<TypeId, HashMap<Entity, Box<dyn Any>>>,
    next_EID: Entity,
}

#[allow(unused)]
#[allow(nonstandard_style)]
impl Universe {
    fn new() -> Self {
        Self { entities: Vec::new(), components: HashMap::new(), next_EID: 0 }
    }
    fn create_enitiy(&mut self) -> Entity {
        let ent: u32 = self.next_EID;
        self.next_EID += 1;
        self.entities.push(ent);
        ent
    }
    fn add_component<T: 'static>(&mut self, _entity: Entity, _component: T) {
        let type_id: TypeId = TypeId::of::<T>();
        let component_map: &mut HashMap<u32, Box<dyn Any + 'static>> = self.components.entry(type_id).or_insert_with(HashMap::new);
        component_map.insert(_entity, Box::new(_component));
    }
    fn get_component<T: 'static>(&self, _entity: Entity) -> Option<&T> {
        let type_id: TypeId = TypeId::of::<T>();
        self.components.get(&type_id)?
            .get(&_entity)?
            .downcast_ref::<T>()
    }
    fn get_component_mut<T: 'static>(&mut self, _entity: Entity) -> Option<&mut T> {
        let type_id: TypeId = TypeId::of::<T>();
        self.components.get_mut(&type_id)?
            .get_mut(&_entity)?
            .downcast_mut::<T>()
    }
}