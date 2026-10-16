pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let word: Vec<char> = word.chars().collect();
    let mut board = board;

    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if dfs(&mut board, &word, row, col, 0) {
                return true;
            }
        }
    }

    false
}

fn dfs(board: &mut Vec<Vec<char>>, word: &[char], row: usize, col: usize, i: usize) -> bool {
    if board[row][col] != word[i] { return false; }
    if i + 1 == word.len() { return true; }

    let saved = board[row][col];
    board[row][col] = '#'; // mark instantly

    let found = (row > 0 && dfs(board, word, row - 1, col, i + 1))
        || (row + 1 < board.len() && dfs(board, word, row + 1, col, i + 1))
        || (col > 0 && dfs(board, word, row, col - 1, i + 1))
        || (col + 1 < board[0].len() && dfs(board, word, row, col + 1, i + 1));

    board[row][col] = saved;
    found
}
