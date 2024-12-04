// use rattlebrain::query::query_ai;
// use std::fs;

// #[test]
// fn test_query_output_file_exists() {
//     // Define the path to the query output file
//     let match_guid = "383F0B0411EFAC27082CAFA884251EFF".to_string();
//     let query_output_path = format!("output/{}.query.txt", match_guid);
//     let focus = "all".to_string();

//     // println!("Querying AI for insights...");
//     // match query_ai(match_guid, focus).await {
//     //     Ok(response) => {
//     //         println!("{}", &response);
//     //     }
//     //     Err(e) => eprintln!("Failed to save AI feedback: {}", e),
//     // }

//     // Check if the output file exists
//     assert!(
//         fs::metadata(query_output_path).is_ok(),
//         "The query output file does not exist at the expected location: {}",
//         query_output_path
//     );

//     // Cleanupfile
//     fs::remove_file(query_output_path).expect("Failed to remove query text file");
// }

// #[test]
// async fn test_query_output_file_exists() {
//     // Define the path to the query output file
//     let match_guid = "383F0B0411EFAC27082CAFA884251EFF".to_string();
//     let query_output_path = format!("output/{}.query.txt", match_guid);
//     let focus = "all".to_string();

//     println!("Querying AI for insights...");
//     match query::query_ai(match_guid, focus).await {
//         Ok(response) => {
//             println!("{}", &response);
//         }
//         Err(e) => eprintln!("Failed to save AI feedback: {}", e),
//     }

//     // Check if the output file exists
//     assert!(
//         fs::metadata(query_output_path).is_ok(),
//         "The query output file does not exist at the expected location: {}",
//         query_output_path
//     );

//     // Cleanupfile
//     fs::remove_file(query_output_path).expect("Failed to remove query text file");
// }
