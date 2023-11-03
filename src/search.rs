use aur_rpc;
 pub async fn search(pkg: String, num: u8) -> String {
    let mut packages = aur_rpc::search(pkg).await.unwrap();
    packages.sort_by(|a, b| b.num_votes.cmp(&a.num_votes));
    let mut result = Vec::new();

    for (index, package) in packages.iter().enumerate().take(num as usize) {
        result.push(format!("{}. {}", index+1, package.name));
    }
    let response = result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n");
    return response;
}