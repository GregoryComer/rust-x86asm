use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 212, 245], OperandSize::Dword)
}

#[test]
fn vpaddq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 212, 12, 192], OperandSize::Dword)
}

#[test]
fn vpaddq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 212, 212], OperandSize::Qword)
}

#[test]
fn vpaddq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1616316389, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 212, 60, 125, 229, 7, 87, 96], OperandSize::Qword)
}

#[test]
fn vpaddq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 212, 219], OperandSize::Dword)
}

#[test]
fn vpaddq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(ECX, 1770252551, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 212, 161, 7, 233, 131, 105], OperandSize::Dword)
}

#[test]
fn vpaddq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 212, 220], OperandSize::Qword)
}

#[test]
fn vpaddq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 212, 40], OperandSize::Qword)
}

#[test]
fn vpaddq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 138, 212, 255], OperandSize::Dword)
}

#[test]
fn vpaddq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EAX, 1171364018, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 205, 141, 212, 144, 178, 152, 209, 69], OperandSize::Dword)
}

#[test]
fn vpaddq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 245, 155, 212, 1], OperandSize::Dword)
}

#[test]
fn vpaddq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 165, 141, 212, 204], OperandSize::Qword)
}

#[test]
fn vpaddq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectDisplaced(RSI, 1825689233, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 237, 135, 212, 174, 145, 206, 209, 108], OperandSize::Qword)
}

#[test]
fn vpaddq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 162625844, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 133, 146, 212, 20, 213, 52, 121, 177, 9], OperandSize::Qword)
}

