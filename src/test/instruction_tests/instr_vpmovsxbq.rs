use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxbq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 253], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 39], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 208], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 1252846155, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 148, 128, 75, 234, 172, 74], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 211], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(ECX, 428334018, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 145, 194, 219, 135, 25], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 216], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 41], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 34, 248], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1093818330, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 34, 20, 253, 218, 87, 50, 65], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 139, 34, 239], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM23)), operand2: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 125, 141, 34, 62], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 34, 234], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 1715490081, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 34, 140, 120, 33, 77, 64, 102], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 125, 170, 34, 231], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM19)), operand2: Some(IndirectDisplaced(RSI, 1200352506, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 171, 34, 158, 250, 236, 139, 71], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 34, 247], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 34, 40], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 125, 207, 34, 203], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 1186197084, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 34, 140, 176, 92, 238, 179, 70], OperandSize::Qword)
}

