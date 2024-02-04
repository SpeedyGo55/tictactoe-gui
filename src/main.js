const { invoke } = window.__TAURI__.tauri;

let board = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
let id = 0;

function is_game_over() {
    return new Promise((resolve, reject) => {
        invoke("is_game_over", { board: board }).then((res) => {
            if (res) {
                invoke("evaluate", { board: board }).then((res) => {
                    let resultText = "";
                    switch (res) {
                        case 10:
                            resultText = "You lose!";
                            break;
                        case -10:
                            resultText = "You win!";
                            break;
                        case 0:
                            resultText = "Draw";
                            break;
                    }
                    resolve(resultText);
                });
            } else {
                resolve(null);
            }
        }).catch(err => reject(err));
    });
}

function print_board() {
    id = 0;
    for (let element in board) {
        for (let cell in board[element]) {
            switch (board[element][cell]) {
                case 0:
                    document.getElementById("cell-" + id.toString()).innerText = "";
                    break;
                case 1:
                    document.getElementById("cell-" + id.toString()).innerText = "O";
                    break;
                case -1:
                    document.getElementById("cell-" + id.toString()).innerText = "X";
                    break;
            }
            id++;
        }
    }
}

async function updateBoard() {
    try {

        let res = await invoke("best_move", { board: board });
        board[res[0]][res[1]] = 1;


        // Visual board update
        print_board();
        let gameOverResult = await is_game_over();
        if (gameOverResult) {
            document.getElementById("result").innerText = gameOverResult;
            return;
        }
    } catch (error) {
        console.error("Error updating board:", error);
    }
}

async function move(x, y) {
    if (board[x][y] == 0) {
        board[x][y] = -1;
        let gameOverResult = await is_game_over();
        if (gameOverResult) {
            print_board();
            document.getElementById("result").innerText = gameOverResult;
            return;
        }
        await updateBoard();
    }
}

function init() {
    id = 0;
    for (let x = 0; x < 3; x++) {
        for (let y = 0; y < 3; y++) {
            document.getElementById("cell-" + id.toString()).addEventListener("click", () => move(x, y));
            id++;
        }
    }
    document.getElementById("restart").addEventListener("click", () => {
        board = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
        print_board();
        document.getElementById("result").innerText = "";
    });
    document.getElementById("quit").addEventListener("click", () => {
        window.close();
    });
    document.getElementById("bot_start").addEventListener("click", async () => {
        await updateBoard();
    });
}

init();

print_board();