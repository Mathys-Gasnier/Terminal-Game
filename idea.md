# Game Ideas

## Someking of management game:
- city administration
- postal/delivery service
- forest chopping and management service


## For the "interpreter"

It's returned in an access and functioncall structure:
Access(member_to_access, option<instruction_to_execute_on_member>)
FunctionCall(function_to_call, args)

To store the data that can be accessed:
For the moment we have an enum soring every possible values, object for key, value pairs, function, int values, etc
Need a better way ?

to evaluate:
access: get member, if inst_to_exec is none then return member else return evaluate of member
function call: execute the fn, take it's return and return it the to the user

struct Handles {
  coins: u64
}

impl Handles {

  fn handle(&mut self, inst: Instruction) -> Result<Value, HandleError> {
    match inst {
      Instruction::Access("coins", _) => Ok(Value::Int(self.coins)),
      Instruction::FunctionCall("add", args) => {
        self.coins += 1;
      }
    }
  }
  
}

## Need a saving system

Storing in json ?
So for exemple Root:
{
  coins: 99,
  has_unlocked_this: false
}

## If a forest management game

Only one tree ? Or little "plots" ?
Do we take into account polution and "survavibility" of the forest/tree ?
Is the currency the logs and something rare like an apple the "gems" ?
Or do we sell the logs to have some dollars ?
Do we mutate/boost the trees, new species and all ?
Two "game paths" one being manual labor and the other one relying on the per second cooldown ?
Extendability of the game with end game features like prestiges ? ability/talent points, that give both pasive and active effects ?

## If a postal/delivery service

Which scale ? City, Country, Worldwide, Universe Wide ?
What is handled in which steps:
- Postbox and offices deposits
- Sorting
- Delivery (one, bulk, by size, by weigth ?)
Packages have a size and weigth ?
Start with a door to door mailman to end with delivery drones ?
Money reward per delivery * weigth * size ?
Postal "zone" scaling with time ?
