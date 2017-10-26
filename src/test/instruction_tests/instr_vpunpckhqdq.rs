use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckhqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 109, 202], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1126787981, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 109, 60, 205, 141, 107, 41, 67], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 109, 198], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 109, 20, 192], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 109, 193], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 663379252, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 109, 156, 255, 52, 93, 138, 39], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 109, 253], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 109, 10], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 197, 141, 109, 235], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 1547529095, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 142, 109, 156, 138, 135, 107, 61, 92], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 205, 155, 109, 22], OperandSize::Dword)
}

#[test]
fn vpunpckhqdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 149, 141, 109, 227], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 468155758, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 189, 134, 109, 140, 223, 110, 125, 231, 27], OperandSize::Qword)
}

#[test]
fn vpunpckhqdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHQDQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 1941533583, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 149, 146, 109, 148, 136, 143, 115, 185, 115], OperandSize::Qword)
}

