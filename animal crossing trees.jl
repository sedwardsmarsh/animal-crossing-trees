# A program that emulates the behavior of trees in animal crossing: new horizons
using Random

# let's define a struct for a tree
mutable struct Tree
    num_branches::UInt32
    
    function Tree()
        new_num_branches::UInt32 = rand(1:5)
        new(new_num_branches)
    end
end

function getNumBranches(tree::Tree)
    return tree.num_branches
end


# in AC: NH you can shake a tree to get sticks from it,
# let's setup a function that allows us to replicate this behavior
"Shakes the tree and randomly decides whether to decrease its branches count."
function shake!(tree::Tree)
    # 65% of the time, a stick drops
    if rand() > 0.65
        tree.num_branches -= 1
        return 1
    end
    return 0
end


# lets create a tree
randomTree = Tree()
println("This tree has $(randomTree.num_branches) branches.")
print("Do you want to shake the tree to get its branches? \ntype y\\n: ")
start_shaking = readline()

if start_shaking == "y"
    # shake the tree!
    while getNumBranches(randomTree) > 0
        branches_dropped = shake!(randomTree)
        if branches_dropped > 0
            println("You got a branch!\n")
        else
            println("You didn't get any branches...\n")
        end
        
        print("Press the Enter key to shake the tree again!: ")
        readline()
    end

    println("\nThis tree doesn't have any more branches.")

else
    println("\nYou didn't want to shake the tree...")
end