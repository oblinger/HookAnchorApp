#log 
n:: Misc Text Notes

### 2023-07-08  production bug

We don't yet know what is going on with this bug, so no comment on that.
Still lets use this bug to level up our game!  Three ideas in the thread under this...

DIAGNOSIS
@brian and @tomas once we get to the bottom of this bug leave a succinct summary here.  We might learn more about how we can improve our process.

VERSION CHECKING
@brian it seems possible to version our front and backends, then explicitly ping the back end to check version number each time we wake, and anytime there is a bug or hang.  When we find they are out of sync we popup a message saying:  "App has been updated, please refresh".  If this is possible, we should do it YESTERDAY!  It is sad when we have a bug, and sadder still when a user believes we had one and we didn't even have one!

KPI-tracking
Jasu and I are working on KPIs to track our QA and field testing.  This is fair warning that Jasu will be bugging you to get this data, and soon I will have quotes of stage tests per push.  So she will be pushing folks to make their quota. ;-).   (Jasu I updated our "Stage Customer Game" KPI to be "Stage games per push" since I think we really want to know how many tests were done on each stage image before that push, not just a total.)