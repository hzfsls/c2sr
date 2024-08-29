#[repr(C)]
pub struct VosListHeadS {
    pub next: *mut VosListHeadS,
    pub prev: *mut VosListHeadS,
}

macro_rules! define_val {
    ($val: ident) => {
        let $val = 0;
    }
}

macro_rules! vos_list_init_val {
    ($list: ident) => {
        VosListHeadS {
            next: &($list),
            prev: &($list),
        }
    };
}

macro_rules! vos_list_declare_and_init {
    ($list: ident) => {
        let mut $list = vos_list_init_val!($list);
    };
}

macro_rules! vos_list_init {
    ($head: expr) => {
        unsafe {
            (*$head).next = $head;
            (*$head).prev = $head;
        }
    };
}

macro_rules! vos_list_add {
    ($item: expr, $where: expr) => {
        unsafe {
            (*$item).next = (*$where).next;
            (*$item).prev = $where;
            (*$where).next = $item;
            (*$item).next.prev = $item;
        }
    };
}

macro_rules! vos_list_add_before {
    ($item: expr, $where: expr) => {
        vos_list_add!($item, (*$where).prev);
    };
}

macro_rules! vos_list_remove {
    ($item: expr) => {
        unsafe {
            (*$item).prev.next = (*$item).next;
            (*$item).next.prev = (*$item).prev;
        }
    };
}

macro_rules! vos_list_is_empty {
    ($head: expr) => {
        unsafe {
            (*$head).next == $head
        }
    };
}

macro_rules! vos_list_for_each_item {
    ($item: ident, $head: expr, $block: block) => {
        let mut $item = unsafe { (*$head).next };
        while $item != $head {
            $block
            $item = unsafe { (*$item).next };
        }
    };
}

macro_rules! vos_list_for_each_item_safe {
    ($item: ident, $temp: expr, $head: expr, $block: block) => {
        let mut $item = unsafe { (*$head).next };
        let mut $temp = unsafe { (*$item).next };
        while $item != $head {
            $block
            $item = $temp;
            $temp = unsafe { (*$item).next };
        }
    };
}

macro_rules! vos_list_for_each_item_rev {
    ($item: ident, $head: expr, $block: block) => {
        let mut $item = unsafe { (*$head).prev };
        while $item != $head {
            $block
            $item = unsafe { (*$item).prev };
        }
    };
}

macro_rules! vos_list_for_each_item_rev_from {
    ($item: ident, $start: expr, $head: expr, $block: block) => {
        let mut $item = $start;
        while $item != $head {
            $block
            $item = unsafe { (*$item).prev };
        }
    };
}


macro_rules! vos_list_entry {
    ($item: expr, $type: ty, $member: ident) => {
        unsafe {
            ($item as *const _ as usize - &(*(0 as *const $type)).$member as *const _ as usize) as *const $type     
        }
    };
}

// tests

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(C)]
    pub struct VosListDataS {
        pub data: i32,
        pub list: VosListHeadS,
    }

    #[test]
    fn test_vos_list() {
        let mut list_d = VosListHeadS {
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
        };
        let mut list_d2 = VosListHeadS {
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
        };
        let mut list = &list_d as *const VosListHeadS as *mut VosListHeadS;
        let mut list1 = &list_d2 as *const VosListHeadS as *mut VosListHeadS;
        vos_list_init!(list);
        vos_list_init!(list1);
        vos_list_add!(list1, list);
        // unsafe {
        //     (*list).next = list;
        //     (*list).prev = list;
        // }
        

        assert_eq!(vos_list_is_empty!(list), true);

    }
}