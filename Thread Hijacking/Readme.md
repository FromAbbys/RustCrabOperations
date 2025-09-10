
# Thread Hijacking


- local -> Creates a dummy thread using CreateThread with the CREATE_SUSPENDED flag, then changes its context to modify the instruction pointer.
