.[[2023 Volleyball SV Product]].
  , ,
  , ,
  DELS: [[2023 Volleyball Product]],[[vball]], 



TODO
- [ ] Download a vball dataset
- [ ] Run dataset thru Halos
- [ ] Plan to detect actions in vball





# LOG

### 2023-10-03  Spec notes from Aaron

VOLLEYBALL SPEC  
Below is a note from Aaron at BallerTV about Volleyball and my response.  I think this gives us a nice framing for our early analysis into volleyball.  I notice that he is pegging things at FOUR hours as our bogey for something that he could treat as an MVP.  He did not give any indication of accuracies, but I bet it is going to be similar to Basketball.  (which we intuit might start being relevant at 80% accuracy, but feels much stronger when we get towards 90% or better)**DANS SUMMARY**  
This is great feedback. Thanks! I will run with this for now, and as we make progress I will let you know, and we can loop your VP of product into the discussion to ensure we are aiming correctly. For now I will:- Use the BallerTV portal and just search “Volleyball” then sample from the games we find.  
- Hopefully we are able to find games both from the side and from the back, so we can optimize our system for both cases.  
- We will investigate all of the events you listed: Serves, Bumps, Sets, Spikes, and BlocksYes, I definitely know about Balltime. There manual annotation interfaces look really very nice. I noticed that their demo videos Don’t show Jersey numbers by default, but instead show randomly generated ID numbers for each player, so I am guessing that they still don’t have Jersey ID really nailed.But I also have no doubt that they will be improving, so we need to HUSTLE!**AARONS NOTE**  
Hi Dan,Great to hear that volleyball is on your radar.It's a great question in terms of MVP. Volleyball is a bit trickier as there are relevant events happening every few seconds without fail... e.g. surve, bump, set, spike, bump, set, spike, etc.Ideally we'd want the following events, although I'd want to loop our VP of Product in on this as she was the captain of the Harvard volleyball team back in the day. I really don't know volleyball that well as I've never played it, even after watching probably 1000s of matches in the last 5 years.  

- Serves
- Bumps / receptions
- Sets
- Spikes / attacks
- Blocks [nice to have]

The other tricky part about volleyball is that is can be filmed from the back and the side. Generally there is a preference for the back view for the purposes of coaching and recruiting, and parents who want to watch prefer the side panning view (e.g. when you watch an ESPN volleyball broadcast it is always from the side). We switched most of our filming to the side view a year ago.We've found event detection in volleyball to be relatively straightforward (easier than basketball). Jersey number detection and assignment can be very difficult though. Ponytails are a big problem for jersey detection in volleyball.Ideally, we are targeting 2 hours or less for this. 4 hours may get it done as a proof of concept, but 8 is too long.#0 mvp isn't enough for us to do an integration. We actually have timestamps for every serve as use use that for our panning production. That is all done on device live. We just haven't had the time to build a product around cutting up the game around knowing.You are probably aware of an AI volleyball startup that does (0), (1), (2), and (3).  
[https://balltime.co/](https://balltime.co/)I suggest taking a look. The quality is ok, but it's still kind of cool. I like what they are building, and I suspect it will get better.Best,