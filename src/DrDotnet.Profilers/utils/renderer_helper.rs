// 
// mod renderer {
//     use std::fmt::Display;
//     use std::ops::AddAssign;
//     use crate::session::Report;
//     use crate::utils::TreeNode;
// 
//     const PADDING: usize = 5;
//     /// if < 0 will print all thread ids
//     const NB_THREAD_IDS_TO_PRINT: usize = 4;
// 
//     pub fn to_html<T, V, F>(tree: &TreeNode<T, V>, format: &F, is_same_level: bool, report: &mut Report)
//         where F: Fn(&TreeNode<T, V>) -> String, V: for<'a> AddAssign<&'a V> + Default + Eq
//     {
//         let inclusive = &tree.get_inclusive_value();
//         let nb_threads = inclusive.0.len();
//         let result = format(tree);
// 
//         if !is_same_level {
//             report.write(format!("\n<details><summary>{result}</summary>"));
//         } else {
//             report.write(format!("\n<li><span>{result}</li>"));
//         }
// 
//         for child in &tree.children {
//             // pstacks style, less nested
//             // let has_same_alignment = nb_threads == child.get_inclusive_value().0.len();
//             // nest even if child has same count of threads because of multiple children
//             let has_same_alignment = (child.children.is_empty() || child.children.len() == 1) && nb_threads == child.get_inclusive_value().0.len();
// 
//             if has_same_alignment && !is_same_level {
//                 report.write(format!("\n<ul>\n"));
//             }
//             to_html(child, format,has_same_alignment, report);
// 
//             if has_same_alignment && !is_same_level {
//                 report.write(format!("\n</ul>\n"));
//             }
//         }
// 
//         if !is_same_level {
//             report.write(format!("\n</details>\n"));
//         }
//     }
// 
//     pub fn to_markdown<T, V:Display, F>(tree: &TreeNode<T, V>, format: &F, depth: usize, report: &mut Report)
//         where F: Fn(&TreeNode<T, V>) -> String
//     {
//         let tabs = str::repeat(" ", PADDING * depth);
//         let new_line = format!("\r\n{tabs}");
// 
//         let inclusive = &tree.get_inclusive_value();
//         let thread_count = format!("{:>PADDING$} ", inclusive.0.len());
// 
//         report.write(new_line.as_str());
//         report.write(thread_count);
//         let frame = format(tree);
//         report.write(frame);
// 
//         if let Some(value) = &tree.value {
//             report.write(new_line.as_str());
//             report.write(format! ("~~~~ {}", value))
//         }
// 
//         for child in &tree.children {
// 
//             let child_depth = if inclusive.0.len() != child.get_inclusive_value().0.len() { depth + 1} else { depth};
// 
//             to_markdown(child, format, child_depth, report);
//         }
//     }
// }