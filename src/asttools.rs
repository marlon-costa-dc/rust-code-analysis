use crate::node::Node;

#[allow(dead_code)]
pub fn get_parent<'a>(node: &'a Node<'a>, level: usize) -> Option<Node<'a>> {
    let mut level = level;
    let mut node = *node;
    while level != 0 {
        if let Some(parent) = node.object().parent() {
            node = Node::new(parent);
        } else {
            return None;
        }
        level -= 1;
    }

    Some(node)
}

// Traverse a tree passing from children to children in search of a specific
// token or series of tokens
pub(crate) fn traverse_children<'a, F>(node: &'a Node<'a>, token_list: &[F]) -> Option<Node<'a>>
where
    F: FnOnce(u16) -> bool + Copy,
{
    let mut node = *node;
    'outer: for token in token_list {
        for temp_node in node.children() {
            if token(temp_node.object().kind_id()) {
                node = temp_node;
                continue 'outer;
            }
        }
        // If a token has not been found, return None
        return None;
    }
    Some(node)
}

macro_rules! has_ancestors {
    ($node:expr, $( $typs:pat_param )|*, $( $typ:pat_param ),+) => {{
        let mut res = false;
        loop {
            let mut node = *$node;
            $(
                if let Some(parent) = node.object().parent() {
                    match parent.kind_id().into() {
                        $typ => {
                            node = Node::new(parent);
                        },
                        _ => {
                            break;
                        }
                    }
                } else {
                    break;
                }
            )*
            if let Some(parent) = node.object().parent() {
                match parent.kind_id().into() {
                    $( $typs )|+ => {
                        res = true;
                    },
                    _ => {
                        break;
                    }
                }
            } else {
                break;
            }
            break;
        }
        res
    }};
}

macro_rules! count_specific_ancestors {
    ($node:expr, $( $typs:pat_param )|*, $( $stops:pat_param )|*) => {{
        let mut count = 0;
        let mut node = *$node;
        while let Some(parent) = node.object().parent() {
            match parent.kind_id().into() {
                $( $typs )|* => {
                    if !Self::is_else_if(&Node::new(parent)) {
                        count += 1;
                    }
                },
                $( $stops )|* => break,
                _ => {}
            }
            node = Node::new(parent);
        }
        count
    }};
}
