use crate::tree_node::TreeNode;
use crate::tree_node_additions::TreeNodeAdditions;
use std::cell::RefCell;
use std::rc::Rc;

/// Serialization is the process of converting a data structure or object into
/// a sequence of bits so that it can be stored in a file or memory buffer, or
/// transmitted across a network connection link to be reconstructed later in
/// the same or another computer environment.
///
/// Design an algorithm to serialize and deserialize a binary tree. There is no
/// restriction on how your serialization/deserialization algorithm should
/// work. You just need to ensure that a binary tree can be serialized to a
/// string and this string can be deserialized to the original tree structure.
///
/// Clarification: The input/output format is the same as how LeetCode
/// serializes a binary tree. You do not necessarily need to follow this format
/// so please be creative and come up with different approaches yourself.
struct Codec;

impl Codec {

    fn new() -> Self {
        Codec { }
    }

    fn serialize(&self, _root: Option<Rc<RefCell<TreeNode>>>) -> String {
        "".to_string()
    }

    fn deserialize(&self, _data: String) -> Option<Rc<RefCell<TreeNode>>> {
        None
    }

}

// import scala.collection.mutable.ArrayBuffer
// import scala.collection.mutable.Queue
//
// class Codec {
//   def serialize(root: TreeNode): String = {
//     if (root == null) "[]"
//     else {
//       val queue: Queue[TreeNode] = new Queue[TreeNode]()
//       queue.enqueue(root)
//       val ab: ArrayBuffer[TreeNode] = new ArrayBuffer[TreeNode]()
//       serialize(queue, ab)
//       ab.map(item => if (item == null) "null" else item.value.toString()).mkString("[", ",",
//       "]")
//     }
//  }
//
//  def serialize (queue.nonEmpty) {
//    while (queue.nonEmpty) {
//      val node = queue.dequeue()
//      ab.addOne(node)
//      if (node != null) {
//        queue.enqueue(node.left)
//        queue.enqueue(node.right)
//      }
//    }
//  }
//
//  def deserialize(data: String): TreeNode = {
//    if (data == "[]") null
//    else {
//      val items: Array[String] = data.stripPrefix("[").stripSuffix("]").split(",")
//      val nodes: Array[TreeNode] = items.map {
//        value => if (value == "null") null else new TreeNode(value.toInt)
//      }
//      val queue: Queue[TreeNode] = new Queue[TreeNode]
//      queue.enqueue(nodes.head)
//
//      var index = 1
//      var isLeft: Boolean = true
//      var node: TreeNode = null
//      while (index < nodes.size) {
//        if (isLeft) {
//          node = queue.dequeue();
//          node.left = nodes(index)
//          if (node.left != null) queue.enqueue(node.left)
//        } else {
//          node.right = nodes(index)
//          if (node.right != null) queue.enqueue(node.right)
//        }
//        isLeft = !isLeft
//        index += 1
//      }
//      nodes.head
//    }
//  }
//}
