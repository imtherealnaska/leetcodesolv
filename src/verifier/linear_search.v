Require Import Coq.Lists.List.
Require Import Coq.Arith.PeanoNat.

Import ListNotations.

Fixpoint linear_search {A : Type } (eq_dec : forall x y : A , {x = y} + {x <> y})
            (l : list A) ( x : A) : option nat :=
    match l with
    | [] => None
    | h :: t =>
    if eq_dec  h x
    then Some 0
    else match linear_search eq_dec t x with
    | None => None
    | Some n => Some ( S n)
    end
    end.
Theorem linear_search_correct :
  forall (A : Type) (eq_dec : forall x y : A, {x = y} + {x <> y}) (l : list A) (x : A),
    match linear_search eq_dec l x with
    | Some i => nth i l = Some x
    | None => ~ In x l
    end.
Proof.
  intros A eq_dec l x. induction l as [|h t IH].
  - simpl. intros H. auto.
  - simpl. destruct (eq_dec h x) as [Heq | Hneq].
    + rewrite Heq. reflexivity.
    + destruct (linear_search eq_dec t x) as [i|] eqn:E.
      * simpl. apply IH.
      * intros [Hh | Ht].
        -- congruence.
        -- apply IH. assumption.
Qed.
