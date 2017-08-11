use facade_attr::facade;

#[facade] mod sub1;
#[facade(hide)] mod sub2;
