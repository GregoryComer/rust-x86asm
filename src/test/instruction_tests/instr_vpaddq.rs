use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 212, 227], OperandSize::Dword)
}

#[test]
fn vpaddq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 212, 60, 216], OperandSize::Dword)
}

#[test]
fn vpaddq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 212, 231], OperandSize::Qword)
}

#[test]
fn vpaddq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1808172036, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 212, 28, 213, 4, 132, 198, 107], OperandSize::Qword)
}

#[test]
fn vpaddq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 212, 227], OperandSize::Dword)
}

#[test]
fn vpaddq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 212, 24], OperandSize::Dword)
}

#[test]
fn vpaddq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 212, 245], OperandSize::Qword)
}

#[test]
fn vpaddq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 212, 6], OperandSize::Qword)
}

#[test]
fn vpaddq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 237, 142, 212, 215], OperandSize::Dword)
}

#[test]
fn vpaddq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ESI, 2117296551, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 221, 143, 212, 142, 167, 97, 51, 126], OperandSize::Dword)
}

#[test]
fn vpaddq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 158, 212, 8], OperandSize::Dword)
}

#[test]
fn vpaddq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 133, 139, 212, 201], OperandSize::Qword)
}

#[test]
fn vpaddq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 181, 129, 212, 44, 129], OperandSize::Qword)
}

#[test]
fn vpaddq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1185614426, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 165, 151, 212, 12, 205, 90, 10, 171, 70], OperandSize::Qword)
}

