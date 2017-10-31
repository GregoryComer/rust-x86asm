use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtdq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 213], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 136786569, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 4, 85, 137, 50, 39, 8], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 238], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RDI, 1052518743, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 175, 87, 41, 188, 62], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 203], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1520890125, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 60, 157, 13, 241, 166, 90], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 204], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 2100428928, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 132, 143, 128, 0, 50, 125], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 230, 209], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 1519248297, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 230, 140, 120, 169, 227, 141, 90], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 65, 126, 142, 230, 204], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM26)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 126, 138, 230, 22], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 169, 230, 234], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 170, 230, 23], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 126, 171, 230, 217], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM24)), operand2: Some(IndirectDisplaced(RDI, 964501249, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 126, 172, 230, 135, 1, 31, 125, 57], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 204, 230, 227], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 429958108, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 206, 230, 36, 69, 220, 163, 160, 25], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 126, 203, 230, 243], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM10)), operand2: Some(IndirectDisplaced(RDI, 732836078, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 126, 206, 230, 151, 238, 48, 174, 43], OperandSize::Qword)
}

