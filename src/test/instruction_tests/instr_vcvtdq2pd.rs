use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtdq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 212], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EBX, 1723198665, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 171, 201, 236, 181, 102], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 225], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1151819291, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 230, 52, 253, 27, 94, 167, 68], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 214], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 827744486, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 140, 145, 230, 96, 86, 49], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 248], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 230, 3], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 143, 230, 232], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 1974055215, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 139, 230, 180, 128, 47, 177, 169, 117], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 126, 138, 230, 196], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(XMM31)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 126, 143, 230, 63], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 170, 230, 223], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 2068662946, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 171, 230, 172, 155, 162, 74, 77, 123], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 126, 175, 230, 194], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(RDI, 482228505, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 174, 230, 191, 25, 57, 190, 28], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 207, 230, 210], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 1498126874, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 202, 230, 180, 248, 26, 154, 75, 89], OperandSize::Dword)
}

#[test]
fn vcvtdq2pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 126, 203, 230, 254], OperandSize::Qword)
}

#[test]
fn vcvtdq2pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PD, operand1: Some(Direct(ZMM11)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 126, 203, 230, 24], OperandSize::Qword)
}

