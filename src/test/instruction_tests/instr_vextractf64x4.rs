use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextractf64x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(Direct(YMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 203, 27, 221, 83], OperandSize::Dword)
}

#[test]
fn vextractf64x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1217806021, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 253, 72, 27, 20, 69, 197, 62, 150, 72, 46], OperandSize::Dword)
}

#[test]
fn vextractf64x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(Direct(YMM19)), operand2: Some(Direct(ZMM19)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 163, 253, 203, 27, 219, 76], OperandSize::Qword)
}

#[test]
fn vextractf64x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(IndirectDisplaced(RCX, 680470691, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM30)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 99, 253, 72, 27, 177, 163, 40, 143, 40, 63], OperandSize::Qword)
}

