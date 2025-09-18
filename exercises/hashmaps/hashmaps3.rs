// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
//给出一场足球比赛的分数列表(每行一个)。每一行都是的形式:" <球队_ 1 _名称>，<球队_ 2 _名称>，<球队_ 1 _进球>，<球队_ 2 _进球> "
// 例:英格兰，法国，4，2(英格兰进4球，法国2)。
//您必须建立一个分数表，其中包含球队名称、球队得分以及球队失球。构建scores表的一种方法是使用散列表。解决方案的一部分是使用Hashmap编写的，完成它才能通过测试。
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use std::collections::HashMap;

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.
        let team1_new = Team {
            goals_scored:team_1_score,
            goals_conceded:team_2_score,
        };
        let team2_new = Team {
            goals_scored:team_2_score,
            goals_conceded:team_1_score,
        };
        let team_1_names = team_1_name.clone();
        let team_2_names = team_2_name.clone();
        // Term1部分
        match scores.entry(team_1_names) {
            std::collections::hash_map::Entry::Occupied(mut Team_Score) => {
                let mut TeamNow = scores.entry(team_1_name).or_insert(team1_new);
                TeamNow.goals_scored+=team_1_score;
                TeamNow.goals_conceded+=team_2_score;
            },
            std::collections::hash_map::Entry::Vacant(Team_Score) => {
                scores.entry(team_1_name).or_insert(team1_new);
            },
        };
        // Term2部分
        match scores.entry(team_2_names) {
            std::collections::hash_map::Entry::Occupied(mut Team_Score) => {
                let mut TeamNow = scores.entry(team_2_name).or_insert(team2_new);
                TeamNow.goals_scored+=team_2_score;
                TeamNow.goals_conceded+=team_1_score;
            },
            std::collections::hash_map::Entry::Vacant(Team_Score) => {
                scores.entry(team_2_name).or_insert(team2_new);
            },
        };
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
