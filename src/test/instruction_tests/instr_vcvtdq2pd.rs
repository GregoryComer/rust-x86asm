use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtdq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 236], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 49], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 216], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 28, 79], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 235], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 56], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 245], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 415352540, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 188, 115, 220, 198, 193, 24], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 230, 244], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1796421449, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 230, 20, 245, 73, 55, 19, 107], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 142, 230, 221], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM21)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 1430826499, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 126, 143, 230, 172, 80, 3, 174, 72, 85], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 175, 230, 231], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 169, 230, 51], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 126, 173, 230, 224], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM30)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 2005398616, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 126, 173, 230, 52, 181, 88, 244, 135, 119], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 203, 230, 215], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 204, 230, 4, 66], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(YMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 126, 202, 230, 212], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 654220602, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 126, 203, 230, 12, 197, 58, 157, 254, 38], OperandSize::Qword)
}

