.[[Measuring Intelligence]].
  , [[ai2025-03-17 GGP]],
  [[ai2025-03-18 Measuring AGI on unseen]],
  [[ai2025-03-18 Measuring AGI Performance on Unseen Problems_ Majo]],
  , [[ai2025-03-18 Checkers]], 



[[GGP]] 





###  Proposal

#### Assertions
- isa(KLASS1, KLASS2)
- has(KLASS, PART)
- PART -- part_parent: KLASS, part_key: str
- MODIFIER()


#### As Python
```python


class Entity:
	modifiers: Set[Modifier]
	parts: Set[Part]
	part_parent: Optional[Entity]
	part_key: Optional[str]

class Modifier:
	pass  # args included in subclasses

class Part:
	entity: Entity
	# part kinds may have other args

class Agent:
	#

class World:
	entities: Set[Entity]
	agents: Set[Agent]

class Action(Term):
	# Typically involves entities and arguments





ASSERTIONS

INSTANCE(ENTITY, KLASS)
PART(ENTITY)
MODIFIER(ENTITY, ...)

Example Entity:
e1 = ChessPosition( ... ) = {"_": "ChessPosition", 0:"WB", 1:...}
e2 = 

```

#### Tic Tac Toe

```python
from pydantic import BaseModel, conlist
import copy


## PUZZLE SPECIFICATION CLASSES

class Entity:
	"""Used for all things within the puzzle world.  May recursively contain sub-entities"""

class World:
	"""Typically contains fields made up of entities, or constant values."""

ActionFn = Callable[[World, ...], List[World]]

ScoreFn = Callable[[World], float]

class Puzzle:
	"""Fully specifies a puzzle to be solved"""
	start: Board
	actions: List[ActionFn]
	goal: ScoreFn
	def __init__(*, start: Board, actions: List[ActionFn], goal: GoalFn):
		self.start, self.actions, self.goal = start, actions, goal


### EXAMPLE USAGE: Defining the Tick Tac Toe game using this format

class Piece(Entity, Enum):
	X = 'x'
	O = 'o'
	EMPTY = ' '  # empty cell

class Board(World):
	board: conlist(item_type: Piece, min_length=9, max_length=9)
	next_to_move: Piece
	def __init__(b: Board, n:Piece):
		self.board, self.next_to_move = b, n

def place_piece(b: Board, loc: int = Field(ge=0, lt=9)) -> List[Board]
	if b.board[loc] == Piece.EMPTY:
		new_b = copy.copy()
		new_b.board[loc] = b.next_to_move
		new_b.next_to_move = Piece.X if b.next_to_move == Piece.O else Piece.O
		return [new_b]
	else:
		return 

def three_in_a_row(b: Board, p: Piece):
	return 
		b.board[0] == p and b.board[1] == p and b.board[2] or
		b.board[3] == p and b.board[4] == p and b.board[5] or
		b.board[6] == p and b.board[7] == p and b.board[8] or
		b.board[0] == p and b.board[3] == p and b.board[6] or
		b.board[1] == p and b.board[4] == p and b.board[7] or
		b.board[2] == p and b.board[5] == p and b.board[8] or
		b.board[0] == p and b.board[4] == p and b.board[8] or
		b.board[2] == p and b.board[4] == p and b.board[6]

def x_wins(b: Board) -> float:
	return 100 if three_in_a_row(b, Piece.X) and not three_in_a_row(b, Piece.O) else 0


starting_board = Board([' ', ' ', ' ',  ' ', ' ', ' ',  ' ', ' ', ' '], 'x')
tick_tac_toe = Puzzle(start=starting_board, actions=[place_piece], goal=x_wins)



```