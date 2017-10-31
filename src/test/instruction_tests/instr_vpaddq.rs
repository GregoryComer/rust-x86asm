use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 212, 230], OperandSize::Dword)
}

#[test]
fn vpaddq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDX, 819959589, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 212, 154, 37, 151, 223, 48], OperandSize::Dword)
}

#[test]
fn vpaddq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 212, 234], OperandSize::Qword)
}

#[test]
fn vpaddq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RDX, 1086109275, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 212, 170, 91, 182, 188, 64], OperandSize::Qword)
}

#[test]
fn vpaddq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 212, 254], OperandSize::Dword)
}

#[test]
fn vpaddq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1276637312, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 212, 20, 213, 128, 240, 23, 76], OperandSize::Dword)
}

#[test]
fn vpaddq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 212, 247], OperandSize::Qword)
}

#[test]
fn vpaddq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1171761406, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 212, 12, 189, 254, 168, 215, 69], OperandSize::Qword)
}

#[test]
fn vpaddq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 205, 139, 212, 206], OperandSize::Dword)
}

#[test]
fn vpaddq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 229, 139, 212, 33], OperandSize::Dword)
}

#[test]
fn vpaddq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 154, 212, 12, 131], OperandSize::Dword)
}

#[test]
fn vpaddq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 81, 173, 135, 212, 206], OperandSize::Qword)
}

#[test]
fn vpaddq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM24)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 189, 134, 212, 38], OperandSize::Qword)
}

#[test]
fn vpaddq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 538068318, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 141, 147, 212, 4, 189, 94, 69, 18, 32], OperandSize::Qword)
}

