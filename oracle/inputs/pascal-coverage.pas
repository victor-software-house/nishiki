{ Reaches the units of Shiki's pascal grammar that the other inputs miss. }
unit Coverage;

interface

procedure Forwarded(A: Integer; B: (X)); forward;
function Imported; external 'lib';
function TBox.Make(const S: string);   Attribute;
constructor Create; FORWARD;

implementation

(* A parenthesised comment
   across two lines *)
    // A comment after blanks
	// A comment after a tab

procedure Check;
begin
  Assert(1 < 2);
  ASSERT(True); (* inline *)
end;

end.
