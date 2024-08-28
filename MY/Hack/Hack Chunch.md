  
above = count / total >= self._window_ratio_threshold  
if (  # count!=0 or  
      #  above or        (str(player_id) in ["2907"])):  # "2982",  
    print(f"### {player_id=}  {count=} / {total=}  =  {above=}")