import berserk
import chess
import math
import random
import time
import datetime
import chesslibrary ## Rust functions

import os

session = berserk.TokenSession(os.getenv("LICHESS_API_KEY"))
client = berserk.Client(session)

board = chess.Board()

## Run by "maturin develop" in terminal and run this file

## Makes the move on the lichess board. Handles disconnections
def make_move_on_board(game_id, move, max_retries):
    
    for attempt in range(max_retries):

        try:

            client.bots.make_move(game_id, move)
            return
        
        except (berserk.exceptions.ApiError) as e:
            print(f"Attempt {attempt + 1} failed: {e}")
            time.sleep(2 ** attempt)

    print("Failed to make move after multiple attempts.")

## Accepts challenge and runs the logic for accepting input from the lichess board
def play():
    
    game_in_progress = False
    for challenge in client.bots.stream_incoming_events():

        if game_in_progress == False:

            if challenge.get('type') == 'challenge':

                if accept_challenge(challenge):

                    game_id = challenge['challenge']['id']
                    client.challenges.accept(game_id)
                    game_in_progress = True

                    while game_in_progress:

                        for event in client.bots.stream_game_state(game_id):

                            ## Game over logic
                            if event.get('status') != None:
                                if(event['status'] != 'started'):
                                    print("Game Over")
                                    game_in_progress = False
                                    break

                            ## Board setup and first move if white
                            if event.get('type') == 'gameFull':

                                if event['initialFen'] != 'startpos':
                                    board.set_fen(event['initialFen'])

                                else:
                                    board.set_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")

                                bot_colour = get_bot_colour(event)
                                if bot_colour == 'white':
                                    next_move = random.choice(["e2e4","d2d4","g1f3"])
                                    make_move_on_board(game_id, next_move, 3)

                            ## Handles bot moves
                            elif event['type'] == 'gameState':
                                move_list = event['moves'].split(" ")
                                last_move = move_list[-1]
                                board.push(chess.Move.from_uci(last_move))

                                ## Makes bot move
                                if is_bot_move(bot_colour):

                                    if len(move_list) > 1:
                                        bot_time = get_time(bot_colour, event)
                                        next_move = get_best_move(bot_colour, bot_time)
                                        make_move_on_board(game_id, next_move, 3)

                                    ## Make first move as black quickly so opponent doesn't abandon
                                    else:
                                        next_move = get_best_move(bot_colour, 0)
                                        make_move_on_board(game_id, next_move, 3)


## Determines if a challenge should be accepted or declined based on time control and variant. Returns a boolean
def accept_challenge(challenge):
    clock_time = challenge['challenge']['timeControl']['limit']
    clock_increment = challenge['challenge']['timeControl']['increment']
    variant = challenge['challenge']['variant']['name']
    if variant == 'Standard':
        if clock_time >= 15:
            if clock_increment >= 1:
                return True
    return False

## Gets the best move
def get_best_move(bot_colour, bot_time):
    game_fen = board.fen()
    best_move = chesslibrary.pick_move(game_fen, 4, bot_colour)
    return best_move[0]

## Determines if it's the bots turn
def is_bot_move(bot_colour):
    return (board.turn == (bot_colour == 'white'))

## Gets colour of bot pieces
def get_bot_colour(game_info):
    if game_info['white']['id'] == 'garntbot':
        return 'white'
    return 'black'

## Gets time on the bot's clock
def get_time(bot_colour, event):
    if(bot_colour == 'white'):
        return math.floor((event['wtime'] - datetime.datetime(1970, 1, 1, tzinfo=datetime.timezone.utc)).total_seconds())
    return math.floor((event['btime'] - datetime.datetime(1970, 1, 1, tzinfo=datetime.timezone.utc)).total_seconds())

if __name__ == "__main__":
    play()