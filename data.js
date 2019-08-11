function input(b,OKIp) {
        var dat = [];
      for (var x = 0; x < b.length; x++) {
        if (x >  b.length-2) {
          continue;
        }
        var v = false;
        for (var y = 1; y < b.length-x; y++) {
          var str = b.substring(x,y);
          if (y<4) {
            continue;
          }
          for (var g of OKIp) {
            if (g.includes(str)) {
              v = true;
              break;
            }
          }

          if (v) {
            continue;
          }
          dat.push(str);
        }
      }
      return dat;
}

function filter(dat,VPNS) {
  var gdat = [];
  for (var q of dat) {
    var count = 0;
    for (var d of VPNS) {
      if (d.includes(q)) {
        count++;
      }
    }
    if (count < 2) {
      continue;
    }
    gdat.push(q);
  }
  var lua = [];
  for (var t of gdat) {
  	var u = false;
    for (var y of gdat) {
      if (y.includes(t) && y.length > t.length) {
      	u = true;
        break;
      }
    }
    if (u) {
    	continue;
    }
    if (!lua.includes(t)) {
      lua.push(t);
    }
  }
  return lua;
}

function normalize(input) {
  return input.toLowerCase().trim();
}

function async(func) {
    setTimeout(func, 0);
}


function train(VPN,NORMAL,end) {
    var VPN1 = [];
    var NORMAL1 = [];
    for (var t of VPN) {
      var f = normalize(t);
      if (f.length > 2) {
        VPN1.push(f);
      }
    }
    for (var t of NORMAL) {
      var f = normalize(t);
      if (f.length > 2) {
        NORMAL1.push(f);
      }
    }
    var date = [];
    var t = VPN1.length;
    var t2 = 0;
    for (var t1 of VPN1) {
	     for (var g of filter(input(t1,NORMAL1),VPN1)) {
	        date.push(g);
	      }
	      t2++;
        console.log(Math.floor(t2*100/t));
	     
    }
    end(filter(date,VPN1));
}

function stringToList(str) {
  var qa = [];
  for (var ju of str.split("\n")) {
    var t = ju.toLowerCase().replace("\r","").trim();
    if (t.length>3) {
      qa.push(t);
    }
  }
  return qa;
}
var fs = require("fs");
var N = fs.readFileSync('NORMAL.txt', 'utf8');
var V = fs.readFileSync('VPN.txt', 'utf8');


console.time('Training on dataset');
var a = stringToList(V);
var b = stringToList(N);
train(a,b,function(e) {
	console.log(e);
	console.timeEnd('Training on dataset');
	console.log("dataset size:",a.length,b.length)
});
