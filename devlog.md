# Devlog

## 2026-05-25

13043817825332783000 (2**63.5)

## 2026-05-24

More cleverness?

Ok, the remove and leave gap is pretty unavoidable...

### the two vec solution

- node is (K,V,usize)
- between half is kept sorted in one array, and the rest unsorted in the other
- the usize is used as pointer: both point to a buddy in the other array, though
  the buddy is optional for the longer array.
- so an insert in principle ends up inserted at the end, with a pointer to its
  proper position in the list.
- there is a pointer the other way, for deletes. now this is interesting: can
  insert push more than one? what if there is nothing there when a delete must
  happen?
- the get starts with the binary search, but may have to linearly search the
  rest...

A limitation of the Map trait is that get must not change the state of the tree.
Perhaps that should be relaxed: It may help to optimize the structure will
searching the data structure.

In particular: lazy inserts. Just add inserts to a list to search linearly, When
that list gets long, sort and merge it.

How'd ya do delete then?

The delete has to give up ownership, also limits the options: just copying data
and leaving the key in place is not allowed.

### some details

Treat the optionals positions as even, and the required positions as even.

Growth: start at len+1 and find the element that should go there--this is the
element at ca 76% of that index. To replace this element, another is needed,
again as a percentage of that index, and so on. Eventually, the point is reached
where the element may stay in its place. It feels a bit risky: what ensures that
the main array remains properly ordered?

### idea

To grow, just fill or empty the main array up to capacity, then merge. The issue
is always that if you move stuff into the main array, the stuff wedged in
between is out of place. To shrink, ...

### binary numbers and such

- have a bunch of arrays of length 2^n
- to get, sadly, all have to be binary searched.. unless there is a way to
  relate the contents of these arrays

I am losing interest in this project. Arbtree shockingly seems the answer.

## 2026-05-23

### new plan for gap array

Store lengths in the gaps.

So I've this method for laying out trees with fixed depth in memory, including
variations.

Like the `left(i) = 2*i+1` and `right(i) = 2*i+2`. Elements would not be stored
in order, unless you consider the layers: each layer is ordered.

Why do this?

Blame the success of arbtree. The fact that this silly solution beats everything
else forces.

How would it work?

- do a binary search on leaf node, all of which should be inhabited
- when the position is found, inserting there bumps some element to the next
  layer obviously, the choice is bumping the element to the left or to the right
  or going up on its own. A, but if the insert is not between siblings, it
  becomes more complex! bias toward lower indices? i.e. always insert by
  replacing the leave node bolwe it, and then insert that element a layer up.

### analysis

A full tree has many layers filled. pushing up may not always be possible, so
instead, push down until it becomes an option. this way some element ends up at
the end.

combine with scew binary? like: leave out the levels

### parallel to binary numbers?

like keep each level at length 2^n?

or with scew binary numbers at 2^n-1?

## 2026-05-22

### cost analysis

It's like the dynamic array: growing has a lot of cost, bu by growing
exponentially, the cost is amortized to be constant. So do the same thing here:
restore the

## 2026-05-21

### disappointing

The solution with the whole segment tree structure doesn't seem to work that
well. There are bugs in the code that might explain something...

I spend some time pondering how to do the redistribution, because the array
can't lose elements, but i think I found the solution: traverse twice in
opposite directions, and in both directions only move the elements that can be
moved.

### easy be effective strategies?

Different idea: if no gap is found within distance log(N), just start the
rebalance process, pushing elements as far away as needd to get under the
threshold.

### getting the number right

How to get log N performance out of these trees?

the trick with doubling in size and copying over is that the chance of
triggering the expensive operation halves

## 2026-05-20

### refactoring the packed array

The hard part is the segment tree of segments. The number of elements in each
segment is dynamic, and has to be stored somewhere, and the number of segments
grows, so how to do that?

Without any attempt at rebalancing, the values now fall in the normal range.

## 2026-05-17

### letting copilot implement a packed memory array

It seemed to known what it was talking about, but failed. It use linear search
to find elements, and therefore is far slower than the other implementations. I
also notice an unused 'occupied' field, that according to its docs helps make
search efficient, but does no such thing. I'll commit it before trying to fix
it.

### break up

Note this idea: just have arrays of keys and values unordered, using swaps like
the arb tree, but keep pointers in a gapped array. Disadvantage: when comparing
keys during binary search, they come from far way. Advantage: an array of
integer is easier to work with in rust.

maybe do something with rebalancing by swapping elements in an array of indices?

### buddy system and fullness

During the binary search, when the segment where element needs to go is full,
look for the buddy segment and redistribute values. This means each segment has
a counter. If the buddy segment is full, this should be detected a level up, and
hence not happen.

Here we can reintroduce the gap threshold: i.e. full means fewer then 'len / log
len' gaps left. Similarly, on delete, redistribution may trigger if fewer than
'len / log len' elements would remain in a segment, with the same args as
before.

Issue: two subsection may be at their thresholds, but the segment above it not,
due to the higher tolerance! Log len or log size still works, it simply must be
the global number, and not differ by segment.

Question: does the threshold do any good? It triggers a uniform fill more
regularly, and then? Is there any less shifting when an element is ultimately
inserted?

It could be more about chosing how densely to pack the buddy: for one side full,
there is a unform distribution of gaps, but based on the threshold, a denser
distribution might outsmart 'hammering' similar for deletes: if part of the
array is getiing empty, intentionally move more or fewer elements in the way
than a uniform distro would ask.

Another criterion is more directly about balance: how different are buddies
allowed to get? In the red black tree, the limit is the square, but that cannot
work.

A lot of shifting would happen as larger sections gradually fill up. A lot of
searching if they gradually lose elements, though the counters make it easy to
skip empty sections. Sp maybe a lower bound on elements is not necessary. Hense
the thresholds: they trade repeated shift higher up with repeated shifts lower
down.

## 2026-05-16

### reflections

I had hoped this would be simpler somehow.

What I have is a packed memory array, but instead of keeping updates locally, it
can only works through the whole array. It does have a solution for sparcity.

### managing gaps and set segment tree.

To speed up skipping bigger gaps,

Let gaps point to either other gaps or to the back of the array? How does this
interact with binary search? It seems better to point down to the least gap, _if
in range_! Because those ranges are the trick.

- On delete: create a gap, and merge it with the gap right below if there is
  one. Note the range rules! Don't forget to merge any gap above it as well,
  though! i.e. the range rule is that no gap can point down further than a
  position with more trailing ones.
- On insert: one can use the rules to alwasy fill gaps from the start, or to
  split a gaps in two. Inserting at the end would save work, but finding the end
  is work. Either way, there are up to log N gaps now pointing to the wrong
  element, and they will be updated. to be specific, the direct left neighbour,
  and all the gaps with more trailing ones above if they were part of the same
  gap.

Changes compared to what I have now:

1. point to a gap: consistency, as there is a vritual gap beyond the end of the
   array, and to use a well know set data structure,
2. point down instead of up: during binary search the middle of the search range
   can move either way. this is a hassle
3. trailing ones instead of zeroes: comes natrually with the change in
   direction, which I am doubting more serious every second.

As elements are shifted into the gaps from below, letting gaps point up has the
advantage that it does not require as many updates.

Another change: put differences in index into the gaps. When copying sliced of
the array, those offset remain valid, even though the indices change.

How to grow? After shifting log N elements, there is no room: insert room there!
how much though?

Gap arrray has magically improved btw...

### coarse grained approach

I have this picture in mind where an insert triggers moving ranges across
greater distances. Every time the insert goes to the first half, ensure space by
splitting the tree in two and moving the second half basically by its own
length.

Well, the idea is to shift on the way down, creating gaps for the insert that is
about to be done. Ideally, these shifts are over greater distances, to create
many gaps at the same time. This is closer to the rotations in the red black
tree...

Yes, something completely different: keep track of occupied element in each
segment, so space shortages can be seen ahead of time, and move can be made.

Why? think about manageing these collections in real live: would just not just
move larger sections

### shift

While shifting, insert the occassional gap to keep the density below a
threshhold. Ok, this does not help against hammering...

## 2026-05-15

### bounded linear search

Note how the array updates:

- when inserting an element, normally shfit alle elment above it one place up

- when deleting an element, either leave a gap, or move the next element in
  position if possible

Either update could leave elements in awkward positions for future updates. So
teh best strategy might be to accept linear search, but within bounds.

It may be difficult bookkeeping:

- cost of binary search, to the extend that that is done
- cost of linear search to its extend
- cost of redisctributing elements, amortized over how often it is needed.

### segment tree

- Keep for each segment the least or greatest occupied index.

Note: deduplicate the information. e.g. the least of the first half the the
segment is the least of the segment, and is the least of the segment is not in
the first half, there is nothing to store there. Also, if the segment is
completely occupied, then the least occupied index is the least index of the
segment. So: one array, with two types of values: 1. key-value pairs or 2.
indices The index points up the the least element in the segment. What is the
segment? Let's say it depends on trailing zeros: the segment is all the way up
to the point where those zeros becomes ones. No trailing zeros: the segment can
only point one place up. four trailing zeros? The segment has 16 = 2 ^ 4
elements. And empty segment can be identified quickly based on their index being
too high. This solves problems on how to update: any new insert or deleted are
only inside a log number of segments.

### more ideas

Pointing to the next occupied slot creates the issue that when an insert is
done, A big scan is needed to reset every pointer past it. Solution: Never point
past a 'parent' node, defined as a node with more trailing zeroes.

When an empty slot get taken, the number of updates is limited to the depth. In
this case, take the newly occupied index `i` and only update `(MAX<<k) & i` as
needed. o/c, stop if evidence of smaller elements is found.

### gap array weakness?

- performance is bad for insert ordered.
- insert random: behind the others, but not by much
- get random: same
- remove random: terrible again

I think the principle is demonstrated, but the details need massaging. One thing
to consider is that elements can be pushed down as well as up, perhaps it would
be logical to switch that based on the side of the array one is working on.

I guess the regular redistro makes ordered insert a worst case scenario.
Something to optimize for?

1. always check the very last element first, so this special case becomes easy
2. occasssionally insert gaps to maintain the 1/log N gap ratio.
3. maybe use segment tree structure to get localized

### more ideas

It does not seem to hard to solve the ordered input problem: just append
elements to the end, occassionally throw in a gap to maintain the ratio of 1 per
log N. The gaps aren't even needed in this specific case. Now suppose the inputs
go downward, though...

Maybe the array should be sparser at the front end, where it has less room to
grow. Or maybe crowd the center instead of the ends: when growing the array,
everything cold move to the center of. With this motion in mind, inserts should
generally move elements away from the center.

Keep in mind that these cases aren't great for trees either: continuous
rebalancing is required if elements come in this way.

The issue may be that an even distribution is simply not a good idea. Instead, a
segment reaching capacity may be considered popular, which would be a reason to
create multiple gap at ones. this turn into higher level gaps: whole blocks in
the array kept empty so other blocks can move in without distubing the reast of
the array, amounting to a fractal structure fro the gaps.

Ultimately, though, it is about the amortized costs. It is fine to occassionally
move many elements, but not necessaryly desirable to create an uniform
distrubution.

-> popularity: a full section is likely popular, an empty section likely
unpopular. Reason to make room _around_ the popular section so it can expand,
while _contracting_ the unpopular sections, probably to the extend, that the
popular section has a lower density than the unpopular sector--but still, within
the thresholds tha otherwise trigger redistributions.

## 2026-05-12

### Linear search

Linear search to solve binary search takes time, suggest that the least number
of elements is k/log(k) as well: if part of the array gets too empty, resize and
maybe shrink.

## 2026-05-11

### now the real deal

Thinking of the structure as a kind of tree, The idea is now that only subtrees
up to a certains levels can be allowed to fill up. Let the level be defined by
the number of trailing zeros. Index 0 is special: it will always hold the least
element.

For very small collections, just keeping in order may be the best options. We've
learned that cache lines are 64-byte, so use that as a first threshold. for
4-byte keys, like the benchmark, this means 16 entries. At that level, 4 slots
should be left empty. A better way so say that is that each sub array of 4
elements should keep an empty slot.

I keep mixing this up. about N/log N element should be left empty, But another
way is the subdivide the array in log N segments, and demand that none are
completely full.

The opposite may also matter.

### disjoint set array structure

Idea: put a pointer to the nearest occupied cell in the empty slots. That way,
search doesn't have to take so long.

## 2026-05-09

### red black deletion

Distiguish actual removal and detatching a node from the tree. If a node has two
children, it should be _replaced_ by its successor. Either node can be deleted,
I guess the question is whether moving keys and values is cheaper than moving
parents and children. Thanks to the swap, the problem of rebalancing after
delete reduce to the case where the node has one or two children.

## 2026-05-06

### switch to MaybeUninit

Just like switching to u32, that should make a difference but did not.

### couple of ideas

For faster soarb: to avoid costs of Vec, introduce a dynamic array type, Which
always behaves like an max length array of possibly invalid values.

Note: I have no idea if this makes any sense. For all I know, I can keep working
with `[MaybeUninit<T>]` and use slice copy to get all the dynamic behavior I
want. ...or not use slice copy, but instead transfer the tree to improve cache
locality somehow. What would actually work? Every node has three edges, so one
loses out. Maybe alternatingly sacrifice left and right, in hopes of evening out
the load.

Yet another array backed tree: each node is an array of keys, of values and of
subtrees. The key are kept sorted, and the subtrees have keys in the intervals.

### benchmarking issues

I was doing this for a bit fit memory allocator, so I try to get a benchmark
where the distribution of values could be similar to that situation. The
implementation used here has something else in mind, though: of equivalent
elements, only one is kept. I am updating the benchmark based on these new
insights. Different benchmarks show less dramatic difference in performance.

## 2026-05-05

### SOARB

Started on the struct of arrays red-black tree structure, but landed at the
following issue. When removing a node, a swap remove doesn't work because it
changes the index of the last node, and the parent node then loses track of
their children.

- One solution: keep a parent vector, so parent nodes can be updated with the
  new position. It requires an extra column of indices.
- Another: keep a free list and reuse the free positions for future allocations.
  Issue: when the node is free, the structure should not own the key and value
  anymore, nor return them on request. Upside: slightly closer to what the
  allocator must do for the original red-black-tree.

### red black density

Each layer doubles the number of elements in the tree. Perhaps this informs how
arrays of gaps should work: red indices indicate fullness for subtrees,
prompting rebalancing measures, possibly even resizing the entire structure.

An array of 2^k nodes should have about 2^k/k gaps... this would means all
subtrees at a depth of log(k)-log(log(k)) should have gaps. Count all the
elements to decide whether resizing is needed
`len <= capacity*(1 - 1 / log(capacity))`, and use one bit per index to track if
subtrees are full, to deal with hammering.

### generic benchmarks

Yes, I copy and modify as well, but in this case I foresee modifications to the
benchmark based on use.

### first results

The SOA version is slower, ca 40%. Obviously, neither has had many optimization
attempts, and perhaps the solution to the issue I mentioned is no good.

## 2026-05-04

- Added a baseline red-black tree implementation in `src/rb_tree.rs`.
- Added `criterion` as a benchmark dependency in `Cargo.toml`.
- Created `benches/benchmark.rs` for insertion, lookup, and removal benchmarks.
- Updated `src/main.rs` with a small usage example.
- Added `README.md` and this `devlog.md`.

## Baseline status

- [x] RB tree implementation
- [x] Criterion benchmark harness
- [x] Example crate usage
- [x] Documentation and project structure

## Next steps

- Add array-based alternatives for comparison.
- Target gap array / packed-memory array implementations next.
- Use the benchmark harness to measure and compare against the RB baseline.

## Review

Note: the lines above are generated by copilot. Perhaps these devlogs can help
to focus copilot over longer times, if not messed with by human users, but such
is not my intention. Moreover, this cost 20 percent of a monthly chat budget.
Keep this going and most of the work here will be me anyway.

- There was a mistake in the criterion configs, causes the benchmakrs not to
  run. Fortunately easy to fix.
- There is a nice safe red black tree implementation, for comparison to other
  implmentations, but is that fair? The example I have in mind is a tree of free
  memory blocks, which uses pointers instead of optional boxes. It might be good
  to try to sacrifice safety for performance, to really challenge the red black
  tree.
- Also, what about a red black tree with an array of nodes, or a struct of
  arrays for all node data? On one hand, perhaps that is the best optimization.
  Perhaps it becomes that if the methods for recycling and rotating nodes are
  carefully designed to keep related nodes near eachother.
- Copilot mentioned packed-memory arrays. This looks like a complicated version
  of what I hoped to do.

So many ideas:

- Binary search offers variations, including how to deal with the gaps. Should
  the search itself move elements?
- Consider these variations: use a dynamic array, or always an array with 2^k
  elements; always start at the position with the most trailing zeros, or the
  most trailing ones, and step by step shorten the tail of repeated elements,
  regardless of array length, or alwasy split the array in roughly equal halves.
- Assuming the trailing ones or zeroes system, some indices are alwasy leaves,
  while other are increasingly near to the root. Should the gaps be at the
  leaves as much as possible, or at the roots as much as possible?
- Do bookkeeping through the structure to determine how much shifting is needed.
- Condence trees after deletions or work with sparse arrays.
- How bad is a naive implementation really?

The worst case scenario is 'hammering', where many elements are inserted near
the same position in the tree. Could be good to have a benchmark for. How well
do red-black trees cope, anyway?
