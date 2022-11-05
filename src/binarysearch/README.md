# Binary Search
This one was... surprisingly easy? Binary Search is one of those slippery devils to get right, but I think we got this one good. Lets review.

## How it works
Binary search answers the question: 'Is something in my list'. Notably, it only answers that question for lists that are in order. So if you've got random data, find something else, this definitely won't work for you. Unless you sort it, but then we're looking at [other algoritms](https://github.com/philipfranchi/ruststructs/blob/master/README.md#16)

Anyways, the most straightforward way of searchign through a list is to loop over it and look at every element, which is a great approach if:
- You have unordeded data that isn't worth sorting
- Your data IS ordered but you couldn't be bothered

But it's not performant, checking every element is `O(n)`. If you want to do better than that, we've got `O(log(n))` right here!

## How it works
1. Look at the sorted list. Compare what we're after to the middle element in the list. It's either bigger, smaller, or the same.
2. If it's the same, we're good!
3. If it's bigger, then our target is in the smaller half. If it's larger, the target is in the bigger half! 

