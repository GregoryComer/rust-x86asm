use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshuff64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 213, 206, 35, 229, 92], OperandSize::Dword)
}

#[test]
fn vshuff64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(33)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 205, 203, 35, 55, 33], OperandSize::Dword)
}

#[test]
fn vshuff64x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 1493099101, Some(OperandSize::Qword), None)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 245, 222, 35, 164, 64, 93, 226, 254, 88, 119], OperandSize::Dword)
}

#[test]
fn vshuff64x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 99, 237, 202, 35, 249, 5], OperandSize::Qword)
}

#[test]
fn vshuff64x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(84)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 227, 165, 198, 35, 12, 198, 84], OperandSize::Qword)
}

#[test]
fn vshuff64x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(RDX, 510244917, Some(OperandSize::Qword), None)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 205, 222, 35, 154, 53, 184, 105, 30, 105], OperandSize::Qword)
}

