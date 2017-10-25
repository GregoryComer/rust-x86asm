use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextractf32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(Direct(XMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 202, 25, 226, 70], OperandSize::Dword)
}

#[test]
fn vextractf32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(IndirectScaledIndexed(EDX, ECX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 25, 36, 74, 18], OperandSize::Dword)
}

#[test]
fn vextractf32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(Direct(XMM12)), operand2: Some(Direct(ZMM27)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 67, 125, 203, 25, 220, 31], OperandSize::Qword)
}

#[test]
fn vextractf32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(IndirectDisplaced(RAX, 1542969091, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM17)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 125, 72, 25, 136, 3, 215, 247, 91, 96], OperandSize::Qword)
}

