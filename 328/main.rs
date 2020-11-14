impl Solution {
    // leetcode 328
    fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut headZ = head;
        let mut head1 = Some(Box::new(ListNode::new(-1)));
        let mut head2 = Some(Box::new(ListNode::new(-1)));
        let mut temp1 = &mut head1;
        let mut temp2 = &mut head2;
        let mut i = 1;

        while let Some(inner_value) = headZ {
            if i % 2 == 1 {
                // take the valle from HeadZ and assign it to mutable reference temp1
                temp1.as_mut().unwrap().next = Some(inner_value);
                // temp1 is now equal to the next value in the list
                temp1 = &mut temp1.as_mut().unwrap().next;
                // head is moved to the value which is not consumed as of now by the while let loop
                headZ = temp1.as_mut().unwrap().next.take();
            } else {
                temp2.as_mut().unwrap().next = Some(inner_value);
                temp2 = &mut temp2.as_mut().unwrap().next;
                headZ = temp2.as_mut().unwrap().next.take();
            }
            i += 1;
        }
        // use as_mut so you can use it as a reference/pointer to head1, unwrap is used to remove the negative one
        temp1.as_mut().unwrap().next = head2.unwrap().next;
        head1.unwrap().next
    }
}
