INSERT INTO restaurant 
  (id, smiley_restaurant_id, name, address, zipcode, city, cvr, pnr, latitude, longitude) 
    VALUES 
  (1, 548708, 'SAMPLE Min restaurant', 'Torvet 2', '8000', 'Aarhus C', '20067535', '1019551292', 56.2678987, 10.8153082),
  (2, 570643, 'SAMPLE Hansens Is', 'Supervej 54',  '9000', 'Aalborg', '20067535', '1019552094', 57.0206236, 9.9391328),
  (3, 3, 'SAMPLE Jensens Bøfhus', 'Storegade 23C',  '5000', 'Odense C', '00000000', '1022505252', 55.351556,10.3821213);

INSERT INTO smiley_report 
  (id, restaurant_id, smiley, report_id, date) 
    VALUES 
  (1, 1, 1, 'Virk1862541', '2021-02-18'),
  (2, 1, 2, 'Virk1766163', '2020-06-09'),
  (2, 1, 2, 'Virk1766163', '2020-06-09'),
  (3, 3, 4, 'Virk1638155', '2019-03-06');

INSERT INTO subscription 
  (id, restaurant_id, token) 
    VALUES 
  (1, 1, 'TODO'),
  (2, 2, 'TODO'),
  (3, 1, 'TODO2');

INSERT INTO notification_history 
  (id, subscription_id, timestamp, data, title, body) 
    VALUES 
  (1, 1, '2021-01-01 12:00:00.300', 'Hej', 'Hej', 'Hej'),
  (2, 1, '2021-01-03 12:00:00.300', 'Hej', 'Hej', 'Hej'),
  (3, 3, '2021-01-06 12:00:00.300', 'Hej', 'Hej', 'Hej');