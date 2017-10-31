use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxbq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 237], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 20, 126], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 202], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 28, 247], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 236], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 26], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 216], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 563591470, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 36, 85, 46, 185, 151, 33], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 34, 255], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDI, 502814473, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 34, 167, 9, 87, 248, 29], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 18, 125, 140, 34, 201], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM20)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1252280168, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 138, 34, 36, 149, 104, 71, 164, 74], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 34, 203], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1450897758, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 34, 52, 93, 94, 241, 122, 86], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM23)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 194, 125, 170, 34, 248], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM29)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 1445862476, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 173, 34, 172, 72, 76, 28, 46, 86], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 34, 234], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 635688074, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 34, 20, 77, 138, 212, 227, 37], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 82, 125, 206, 34, 213], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 1842240112, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 34, 164, 255, 112, 90, 206, 109], OperandSize::Qword)
}

