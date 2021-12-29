use std::{
    collections::{HashSet, LinkedList},
    i32::MIN,
};

pub fn remove_duplicates(list: &mut LinkedList<i32>) {
    let mut set = HashSet::<i32>::new();

    list.clone().iter().enumerate().for_each(|(index, data)| {
        if set.contains(data) {
            list.remove(index);
        } else {
            set.insert(*data);
        }
    });
}

pub fn remove_duplicates_without_buffer(list: &mut LinkedList<i32>) {
    let mut current_data: i32 = MIN;

    list.clone()
        .iter_mut()
        .enumerate()
        .for_each(|(_index, data)| {
            current_data = *data;
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicats() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(2);
        list.push_back(3);

        remove_duplicates(&mut list);

        let mut expected_list = LinkedList::<i32>::new();
        expected_list.push_back(1);
        expected_list.push_back(2);
        expected_list.push_back(3);

        assert_eq!(&expected_list, &list);
    }
}
