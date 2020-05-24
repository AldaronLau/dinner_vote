# MealVote
An app for choosing dinner.

The app `POST`s to `/meal_vote` to communicate.

## Messages
- "l" => Get entire list of dinner options
- "g{}" => Get details for a specific dinner option (pass index)
- "v{}" => Vote (pass User ID)
- "r{}" => Revoke Vote (pass User ID)
- "a{}" => View all votes (pass User ID)
- "c{}" => Create account (pass user's name)
- "n{} {}" => New dinner option (pass (User ID, Shortname))
- "s{} {} {}" => Edit shortname (pass (User ID, index, Shortname))
- "t{} {} {}" => Edit title / longname (pass (User ID, index, Shortname))
- "m{} {} {}" => Edit More details (pass (User ID, index, Shortname))
- "p{} {} {}" => Edit picture (pass (User ID, index, Shortname))
- "d{} {}" => Delete dinner option (pass (User ID, index))
- "r{} {} {}" => Set rating (pass (User ID, index, rating))
- "y{} {?}" => View analytics (pass (User ID, index?))


