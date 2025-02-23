- [x] mettre en place des tests unitaire
- [x] mettre en place des tests d'integration
- [x] pousser les test
  - [x] tester les erreurs de connection à mongo
  - [x] tester le find_all
  - [x] tester le fetch_all
  - [x] tester les erreurs dao
- [x] mettre en place une config pour les tests d'integration
- [ ] (query) implémenter les expression : lessthan heigher than
- [ ] implementer les expressions pour les type i64 (voir mettre en place un new type Number(i64))
- [ ] implémenter les expressions pour les dates (NaiveDate, Date ... etc)


- [ ] maybe :
- - [ ] mettre en place une implementation de DAO pour redis
  - [ ] mettre en place une abstraction pour le caching redis
  - [ ] mettre des config de compilation #[cfg(feature="mongo")] #[cfg(feature="redis")] #[cfg(feature="cache")] ... etc