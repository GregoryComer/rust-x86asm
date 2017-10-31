use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 98, 224], OperandSize::Dword)
}

#[test]
fn vpunpckldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1711574500, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 98, 172, 87, 228, 141, 4, 102], OperandSize::Dword)
}

#[test]
fn vpunpckldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 98, 200], OperandSize::Qword)
}

#[test]
fn vpunpckldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 1568997613, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 98, 188, 216, 237, 0, 133, 93], OperandSize::Qword)
}

#[test]
fn vpunpckldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 98, 242], OperandSize::Dword)
}

#[test]
fn vpunpckldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1395562428, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 98, 188, 114, 188, 151, 46, 83], OperandSize::Dword)
}

#[test]
fn vpunpckldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 98, 230], OperandSize::Qword)
}

#[test]
fn vpunpckldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 98, 52, 219], OperandSize::Qword)
}

#[test]
fn vpunpckldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 137, 98, 233], OperandSize::Dword)
}

#[test]
fn vpunpckldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 138, 98, 28, 201], OperandSize::Dword)
}

#[test]
fn vpunpckldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 69, 153, 98, 28, 187], OperandSize::Dword)
}

#[test]
fn vpunpckldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 85, 140, 98, 253], OperandSize::Qword)
}

#[test]
fn vpunpckldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM29)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 21, 132, 98, 50], OperandSize::Qword)
}

#[test]
fn vpunpckldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 2124166802, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 45, 155, 98, 36, 205, 146, 54, 156, 126], OperandSize::Qword)
}

