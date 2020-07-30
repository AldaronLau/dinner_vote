# MealVote
An app for choosing dinner.

The app `POST`s to `/meal_vote` to communicate.

A flutter Isolate running as a background task opens Server Sent Events at 
`/meal_vote/sse` to get notifications on when it's time to vote.

## Messages
- "l" => Get entire list of dinner options
- "g {}" => Get details for a specific dinner option (pass index)
- "v {}\\{}" => Vote (pass (User ID, index))
- "u {}\\{}" => Revoke Vote (pass (User ID, index))
- "a {}" => View all votes (pass User ID)
- "c {}" => Create account (pass user's name)
- "n {}\\{}" => New dinner option (pass (User ID, Shortname))
- "s {}\\{}\\{}" => Edit shortname (pass (User ID, index, Shortname))
- "t {}\\{}\\{}" => Edit title / longname (pass (User ID, index, Shortname))
- "m {}\\{}\\{}" => Edit More details (pass (User ID, index, Shortname))
- "p {}\\{}\\{}" => Edit picture (pass (User ID, index, Shortname))
- "d {}\\{}" => Delete dinner option (pass (User ID, index))
- "r {}\\{}\\{}" => Set rating (pass (User ID, index, rating))
- "y {}\\{?}" => View analytics (pass (User ID, index?))
- "h {}\\" => Get number of votes (pass (User ID))
- "z {}\\{}" => Set number of votes (pass (User ID, number))


