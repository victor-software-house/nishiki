{-# LANGUAGE GADTs, NoImplicitPrelude, AutoDeriveTypeable #-}
{-# OPTIONS_GHC -Wall #-}
{-# SPECIALISE instance Eq (T Int) #-}
{-# SPECIALIZE [~1] instance Show (T Int) #-}
{-# SPECIALISE INLINE [1] f :: Int -> Int #-}
{-# INLINE [~2] g #-}
{-# DEPRECATED f "use g instead" #-}
{-# WARNING h "unsafe" #-}
{-@ measure size @-}
#if defined(FOO)
module Coverage
  ( module Data.List
  , pattern Pair
  , pattern (:>)
  , type (++)
  , type T
  , foo
  , Bar (..)
  ) where
#endif

import qualified Data.Map as M
import Data.List hiding (insert)

infixl 6 <+>
infixr 5 :>
infix 4 `elem'`

-- | A Haddock comment
-- that continues here.
x = 1
  -- ^ after the binding
{- block
   comment -}
{-| Haddock block -}
{- ^ more -}

class (Eq a, Show a) => Pretty a where
  pretty :: a -> String

instance Pretty Int where
  pretty = show

data T a = A a | B { field :: a, (<#>) :: Int } | a :+ a | a `Op` a
  deriving (Eq, Show)
  deriving stock Ord
  deriving anyclass (NFData)
  deriving newtype Num via (Sum Int)

data P = (:%) Int Int | (Int, Int) :* Int | [Int] :& Int

newtype N = N Int deriving Show

data G a where
  GInt :: Int -> G Int
  (:::) :: a -> G a -> G a
  GRec :: { gField :: Int } -> G Int
  deriving (Show)

data instance F Int where
  FI :: F Int

data family D a
data instance D Int = DI
newtype instance D Bool = DB Bool

type family TF a where
  TF Int = Bool
type instance TF Char = Int
type S = (,) Int
type role Ptr nominal representational phantom

pattern Pair :: a -> b -> (a, b)
pattern Pair a b = (a, b)
pattern  :: Int
pattern p :> q <- (p, q)

foreign import ccall unsafe "math.h sin" c_sin :: Double -> Double
foreign import capi safe "stdio.h puts" (<!>) :: CString -> IO ()
foreign import stdcall interruptible
  "&free" p_free :: FunPtr (Ptr a -> IO ())
foreign export ccall foo :: Int -> Int
foreign import jvm "java.lang.Math.abs" jabs :: Int -> Int
foreign import prim safe
  bar :: Int

deriving instance Show a => Show (T a)
deriving stock instance Eq N
deriving via (Sum Int) instance Semigroup N

f :: forall a b. (Show a) => a -> b -> String
f x _ = show x
g, h :: ∀ a. a → a
g = id
(<+>) :: Int -> Int -> Int
a <+> b = a + b

k :: Int
  -> (# Int, Int #)
  -> (# , #) Int Int
  -> (# #)
  -> '[ 'True, 'False ]
  -> '( 'Int, ' x' )
  -> '[]
  -> '()
  -> '(,) Int Int
  -> [] Int
  -> () -> (,,) a b c
  -> a ': b
  -> a `Either` b
  -> '`Cons` a
  -> M.Map ': Int
  -> (M.+++) a b
  -> (# a | b #)
  -> Proxy "sym"
  -> 'Just 'c'
k = undefined

main :: IO ()
main = do
  let y = f @Int 1 'a'
      z = g @(Maybe Int) Nothing
      w = g @'[Int] []
      v = show @"lit" ()
      u = read @a' "1"
  print (y, z, w, v)
  case x of
    1 -> pure ()
    _ -> pure ()
  if True then pure () else pure ()
  mdo pure ()
  rec pure ()
  proc x -> f -< x
  proc x -> f -<< x
  arr >- y
  arr >>- y
  arr ⤚ y
  arr ⤜ y
  arr ↢ y
  arr ⤛ y
  x ← action
  let r = R { a = 1, b.c = 2, .. }
      s = R { M.a = 1 }
      t = r { (+++) = 3 }
  pure $ x ! y
  pure $! ~x
  pure $$x
  pure !x
  pure -x
  pure ~(a, b)
  pure a@(Just _)
  pure x#
  pure (# a, b #)
  pure (#a#)
  pure (# a #)
  pure #label
  pure #"quoted label"
  pure (x `M.lookup` m)
  pure (a M.!? b)
  pure (a ''M.T.+ b)
  pure (a ..: b)
  pure [1 .. 10]
  pure (\x -> x)
  pure (λx -> x)
  pure 0x1F_FF
  pure 0o17
  pure 0b1010
  pure 1_000.5e-3
  pure 0x1.8p3
  pure 0xA.Bp-2
  pure 'x'
  pure '\n'
  pure '\o17'
  pure '\x7F'
  pure '\^A'
  pure '\SOH'
  pure "\NUL\SOH\o17\x41\123\^Z\&\"end"
  pure "gap \
       \continues"
  pure [d| x = 1 |]
  pure [e|| y ||]
  pure [p|z|]
  pure [t| Int |]
  pure [expr| a + b |]
  pure [$qq| a |]
  pure [$$qq|| a ||]
  pure [M.qq| a |]
  pure ()
  pure [ ]
  pure (,)
  pure (, ,)
  where
    helper = let { a = 1; b = 2 } in a
    other = do { a; b }
    more = case x of { _ -> 1 }
    mdoBlock = mdo { a }
    whr = 1 where { q = 2 }

data X = X
  { xa :: Int
  , (+++) :: Int
  , xb, xc :: Bool
  }

class C a where { cm :: a }
instance C Int where { cm = 1 }
data W = W where { }
stray = 1 -}

uu = (# #)
ut = (# , #) a b

data Q = (a] :+ b

  -- ^ indented Haddock at the root

  -- indented comment at the root

data K = forall a. (Show a) => K a
data E = E deriving (Eq)
data E2 = E2 deriving stock (Eq)
newtype E3 = E3 Int deriving newtype Num via Sum Int
data E4 = E4 deriving (Semigroup) via (Sum Int)

foreign import ccall safe "x.h y"
  y :: Int
foreign import ccall "x.h z" (<+.>) :: Int

fa :: forall a -> a -> a
kk :: Proxy (a :: Type) -> '(:+) a b -> a M.+ b -> a `M.Either` b
kk = g @'(Int, Int) $x "a\ junk \b"

data GG where
  GA, (:%%), GB :: Int -> GG

data GH where GH1 :: GH
data GO where (:%%%) :: Int -> GO

{-# SPECIALISE [~1x] f :: Int -> Int #-}

type family TF2 a where { TF2 Int = Bool; TF2 a = a }
signature Sig lower where

data V where {
  VC :: V;
  VD :: V }
