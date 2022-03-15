# Java.Execution-times --

            K/sec     uSecs          Code           Operation  
          =========  ======= ====================  =========== 
            147,058    0.007 a = a & 0x100;        get element of int bits 
                314    3.180 bitset.get(3);        get element of Bitset 
             20,000    0.050 obj = objs[1];        get element of Array 
              5,263    0.190 str.charAt(5);        get element of String 
                361    2.770 buf.charAt(5);        get element of StringBuffer 
                337    2.960 objs2.elementAt(1);   get element of Vector 
                241    4.140 hash.get("a");        get element of Hashtable 
     
                336    2.970 bitset.set(3);        set element of Bitset 
              5,555    0.180 objs[1] = obj;        set element of Array 
                355    2.810 buf.setCharAt(5,' ')  set element of StringBuffer 
                308    3.240 objs2.setElementAt(1  set element of Vector 
                237    4.210 hash.put("a", obj);   set element of Hashtable 
     
     
     
