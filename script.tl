def greet(name)
    puts "Hello, " + name + "!";
end;

greet("world");

x = 10;
y = 3;
puts x + y;

def factorial(n)
    if n < 2 then
        return 1;
    end;
    return n * factorial(n - 1);
end;

result = factorial(5);
puts result;

i = 1;
while i < 6 do
    puts i;
    i = i + 1;
end;

if 10 > 5 then
    puts "ten is greater";
else
    puts "this won't print";
end;

puts "======================";

puts fetch("http://example.com");


puts " ";
puts " ";
puts " ";
puts "=========================";

yell "HACK CLUB RULES";

craft fire_fire()
    i = 67;
    while i < 69 do
        yell "WHO LET THE DOG OUT";
        i = i + 1;
    end;
end;

fire_fire();