use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallbackId((usize, usize));

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

impl CellId {
    pub fn value(&self) -> usize {
        match &self {
            CellId::Input(InputCellId(id)) => *id,
            CellId::Compute(ComputeCellId(id)) => *id,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<'a, T> {
    cells: Vec<T>,
    fn_of: HashMap<usize, Box<dyn 'a + Fn(&[T]) -> T>>,
    dependants_of: HashMap<usize, Vec<usize>>,
    sources_of: HashMap<usize, Vec<usize>>,
    callbacks_of: HashMap<usize, Vec<Option<Box<dyn 'a + FnMut(T)>>>>,
}

impl<'a, T: Copy + PartialEq + Debug> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            cells: vec![],
            fn_of: HashMap::new(),
            dependants_of: HashMap::new(),
            sources_of: HashMap::new(),
            callbacks_of: HashMap::new(),
        }
    }

    pub fn create_input(&mut self, val: T) -> InputCellId {
        self.cells.push(val);
        let idx = self.cells.len() - 1;
        self.set_sources(idx, &[]);
        InputCellId(idx)
    }

    pub fn create_compute<F: 'a + Fn(&[T]) -> T>(
        &mut self,
        _dependencies: &[CellId],
        _compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        let mut f_vals: Vec<T> = vec![];
        for cell_id in _dependencies {
            let idx = cell_id.value();
            match self.cells.get(idx) {
                Some(v) => f_vals.push(*v),
                None => return Err(*cell_id),
            }
        }

        let f_result = _compute_func(f_vals.as_slice());
        self.cells.push(f_result);
        let new_cell_idx = self.cells.len() - 1;

        self.fn_of.insert(new_cell_idx, Box::new(_compute_func));
        self.set_sources(new_cell_idx, _dependencies);
        self.add_to_deps(_dependencies, new_cell_idx);

        Ok(ComputeCellId(new_cell_idx))
    }

    fn set_sources(&mut self, new_cell_idx: usize, deps: &[CellId]) {
        let ids_iter = deps.iter().map(|&cell_id| cell_id.value());
        let sources_entry = self.sources_of
            .entry(new_cell_idx)
            .or_insert(vec![]);

        for id in ids_iter {
            (*sources_entry).push(id);
        }
    }

    fn add_to_deps(&mut self, deps: &[CellId], new_cell_idx: usize) {
        deps
            .iter()
            .map(|&cell_id| cell_id.value())
            .for_each(|id| {
                let dep_entry = self
                    .dependants_of
                    .entry(id)
                    .or_insert(vec![]);
                dep_entry.push(new_cell_idx);
            });
    }

    pub fn value(&self, id: CellId) -> Option<T> {
        match self.cells.get(id.value()) {
            None => None,
            Some(ref v) => Some(**v),
        }
    }

    pub fn set_value(&mut self, _id: InputCellId, _new_value: T) -> bool {
        let InputCellId(idx) = _id;
        if idx >= self.cells.len() {
            return false;
        }

        self.propagate(idx, _new_value);

        true
    }

    fn is_compute(&self, cell_id: usize) -> bool {
        self.fn_of.get(&cell_id).is_some()
    }

    fn propagate(&mut self, id: usize, val: T) {
        let mut changed_vals_hm: HashMap<usize, (T, T)> = HashMap::new();
        changed_vals_hm.entry(id).or_insert((self.cells[id], val));
        self.cells[id] = val;

        let mut queue = VecDeque::new();
        queue.push_front(id);

        while let Some(compute_cell_id) = queue.pop_front() {
            let maybe_depenants = self.dependants_of.get(&compute_cell_id);
            if let Some(depenants) = maybe_depenants {
                for id in depenants {
                    queue.push_back(*id);
                }
            }

            if !self.is_compute(compute_cell_id) {
                continue;
            }

            let f = self.fn_of.get(&compute_cell_id).unwrap();
            let f_args: Vec<T> = self.sources_of.get(&compute_cell_id)
                .unwrap()
                .iter()
                .map(|&cell_id| self.cells[cell_id])
                .collect();

            let old_val = self.cells[compute_cell_id];
            let new_val = f(f_args.as_slice());
            self.cells[compute_cell_id] = new_val;

            if old_val != new_val {
                let e = changed_vals_hm.entry(compute_cell_id).or_insert((old_val, new_val));
                if e.1 != new_val {
                    (*e) = (e.0, new_val);
                }
            }
        }

        for (cell_id, (old_v, new_v)) in changed_vals_hm {
            if let Some(callback_fns) = self.callbacks_of.get_mut(&cell_id) {
                for maybe_cb in callback_fns.iter_mut() {
                    if let Some(ref mut cb) = maybe_cb {
                        if old_v != new_v {
                            cb(new_v);
                        }
                    }
                }
            }
        }
    }

    pub fn add_callback<F: 'a + FnMut(T)>(
        &mut self,
        _id: ComputeCellId,
        _callback: F,
    ) -> Option<CallbackId> {
        let ComputeCellId(cell_idx) = _id;
        if cell_idx >= self.cells.len() {
            return None;
        }

        let entry = self.callbacks_of.entry(cell_idx).or_insert(vec![]);
        (*entry).push(Some(Box::new(_callback)));
        let callback_idx = (*entry).len() - 1;

        Some(CallbackId((cell_idx, callback_idx)))
    }

    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        let ComputeCellId(cell_idx) = cell;
        let CallbackId((_, callback_idx)) = callback;

        if cell_idx >= self.cells.len() {
            return Err(RemoveCallbackError::NonexistentCell);
        }

        let maybe_cell_callbacks = self.callbacks_of.get_mut(&cell_idx);
        if maybe_cell_callbacks.is_none() {
            return Err(RemoveCallbackError::NonexistentCallback);
        }

        let callbacks = maybe_cell_callbacks.unwrap();
        if callbacks.get(callback_idx).is_none() {
            return Err(RemoveCallbackError::NonexistentCallback);
        }

        let maybe_callback = &mut callbacks[callback_idx];
        if maybe_callback.is_none() {
            return Err(RemoveCallbackError::NonexistentCallback);
        }

        maybe_callback.take();
        Ok(())
    }
}
