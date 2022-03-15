# Javascript --

    [CONTROL]    <<>>  <<>>
      function viewIt(title, obj) <<>>  <<>>*comment*<<>> <<>> comment
      if (name!=null && name!="") <<>>
      for (ele in lst) <<>>
      for (prop in map) <<>>
      var i=0; for (i=0;i<=5;i++) <<>>
      while (i<=5) <<>>
      try <<>> catch(err) <<>>
      TIMER var t=setTimeout("alertMsg()",3000);  <-- called in 3 seconds
      null  undefined  + - * <<>> % ++ --   = += -=  == ===  != < <=  && || !  q?1:2;

    [ARRAY] a=new Array("1", "2", "3");  c=a.concat(b); .push(ele) .pop() .reverse() .slice(start, end) .sort()
       .shift() .unshift() .join() .splice(start, end, ele1, ele2, ...)
    [OBJ/MAP]
      function obj(x) <<>>   \\  o=new obj("a new obj")
      obj=<<>>

    [IO]
      console.log(">>>" + output  );   alert('');
      var name=prompt("Enter name","Harry Potter");  nil|"string"
      document.write("Hello " + name + "! How are you today?");

        
    [STRING] a+b .length .toLowerCase() .toUpperCase()  .match(x)==x
      .substring(strat,len) .charAt(0)
      s.replace(old,new); .indexOf(target)==-1|0...
      var patt=new RegExp(pattern,modifiers);  mod=ig
      var patt=<<>>;    patt.test(str)  patt.exec()->matchStr 



    [DOM]
      document.write('xxx')
      document.getElementById('txt').value='2 seconds!' 


    <<>> a globally-scoped variable
    var a=1;

    <<>> global scope
    function one()<<>>

    <<>> local scope
    function two(a)<<>>

    <<>> local scope again
    function three()<<>>

    <<>> Intermediate: no such thing as block scope in javascript
    function four()<<>>

        alert(a); <<>> alerts '4', not the global value of '1'
    }


    <<>> Intermediate: object properties
    function Five()<<>>


    <<>> Advanced: closure
    var six = function()<<>>
    }()






    <<>> Advanced: prototype-based scope resolution
    function Seven()<<>>

    <<>> [object].prototype.property loses to [object].property in the scope chain
    Seven.prototype.a = -1; <<>> won't get reached, because 'a' is set in the constructor above.
    Seven.prototype.b = 8; <<>> Will get reached, even though 'b' is NOT set in the constructor.

    <<>> These will print 1-8
    one();
    two(2);
    three();
    four();
    alert(new Five().a);
    six();
    alert(new Seven().a);
    alert(new Seven().b);



    <<>> JSONalert(obj) <<>>
    function JSONAlert(title, obj) <<>>
        alert(title + ': ' + JSON.stringify(obj)); }

    <<>> JSONLog(obj) <<>>
    function JSONLog(title, obj) <<>>
        console.log(title + ' ' + JSON.stringify(obj)); }
